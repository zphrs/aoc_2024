use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead};
use std::num::{NonZeroUsize, ParseIntError};
use std::str::FromStr;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FileId(NonZeroUsize);

impl FileId {
    pub const ZERO: Self = Self::new(0);
    pub const fn new(id: usize) -> Self {
        Self(unsafe { NonZeroUsize::new_unchecked(id + 1) })
    }

    pub fn next_id(&self) -> Self {
        Self(self.0.checked_add(1).unwrap())
    }

    pub fn prev_id(&self) -> Self {
        Self::new(usize::from(self.0) - 2)
    }
}

impl From<FileId> for usize {
    fn from(value: FileId) -> Self {
        value.0.get() - 1
    }
}

pub struct Disk {
    blocks: Vec<Option<FileId>>,
    current_file_id: FileId,
}

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for block in self.blocks.iter() {
            write!(
                f,
                "{}",
                match block {
                    Some(id) => format!("({})", usize::from(*id)),
                    None => ".".to_string(),
                }
            )?;
        }
        Ok(())
    }
}

impl Disk {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            blocks: Vec::with_capacity(capacity),
            current_file_id: FileId::ZERO,
        }
    }

    pub fn fill(&mut self, len: usize, is_file: bool) {
        self.blocks
            .extend((0..len).map(|_| is_file.then(|| self.current_file_id)));
        if is_file {
            self.current_file_id = self.current_file_id.next_id();
        }
    }

    pub fn compact(&mut self) {
        let mut empty_ptr;
        let mut filled_ptr;
        let mut iter = 0;
        loop {
            empty_ptr = self
                .blocks
                .iter()
                .enumerate()
                .skip(iter)
                .find(|(_, a)| a.is_none())
                .map(|(a, _)| a);
            filled_ptr = self
                .blocks
                .iter()
                .enumerate()
                .rev()
                .skip(iter)
                .find(|(_, a)| a.is_some())
                .map(|(a, _)| a);
            let Some(empty_ptr) = empty_ptr else {
                break;
            };
            let Some(filled_ptr) = filled_ptr else {
                break;
            };
            if empty_ptr > filled_ptr {
                return;
            }
            self.blocks[empty_ptr] = self.blocks[filled_ptr];
            self.blocks[filled_ptr] = None;
            iter += 1;
        }
    }

    pub fn find_empty_space(
        &mut self,
        before: usize,
        len: usize,
    ) -> Option<&mut [Option<FileId>]> {
        'ind: for i in 0..self.blocks.len() {
            let end = i + len;
            if end >= before {
                return None;
            }
            for j in i..(i + len) {
                if let Some(v) = self.blocks.get(j) {
                    if v.is_some() {
                        continue 'ind;
                    }
                } else {
                    return None;
                }
            }
            // println!("Found empty space from {} to {}", i, i + len);
            return Some(&mut self.blocks[i..i + len]);
        }
        None
    }

    fn step_compact_without_fragmentation(
        &mut self,
        start_at: usize,
    ) -> Option<usize> {
        fn mwhile(
            current_file_id: FileId,
            input: (usize, &Option<FileId>),
        ) -> Option<(usize, FileId)> {
            let (ind, id) = input;
            if let Some(id) = id {
                if current_file_id != *id {
                    return None;
                }
            }
            (*id).map(|id| (ind, id))
        }
        self.current_file_id = self.current_file_id.prev_id();
        let contig_file: Vec<_> = self
            .blocks
            .iter()
            .enumerate()
            .rev()
            .skip(self.blocks.len() - start_at)
            .skip_while(|input| mwhile(self.current_file_id, *input).is_none())
            .map_while(|input| mwhile(self.current_file_id, input))
            .collect();
        // println!("{:?}", contig_file);
        let end_of_contig_file = contig_file[contig_file.len() - 1].0;
        let contig_file_len = contig_file.len();
        let empty_space =
            self.find_empty_space(contig_file[0].0, contig_file_len);
        let Some(empty_space) = empty_space else {
            return (self.current_file_id != FileId::ZERO)
                .then_some(end_of_contig_file);
        };
        for (empty, (_ind, id)) in
            empty_space.iter_mut().zip(contig_file.iter())
        {
            *empty = Some(*id);
        }
        for (ind, _) in contig_file.iter() {
            self.blocks[*ind] = None;
        }
        return (self.current_file_id != FileId::ZERO)
            .then_some(end_of_contig_file);
    }

    pub fn compact_without_fragmentation(&mut self) {
        // println!("{self}");
        let id = self.current_file_id;
        let mut skip_to = self.blocks.len();
        while let Some(skip) = self.step_compact_without_fragmentation(skip_to)
        {
            skip_to = skip;
            // println!("{self}\n");
            // break;
        }
        self.current_file_id = id;
    }

    pub fn hash(&self) -> usize {
        let mut out = 0;
        for (ind, id) in self
            .blocks
            .iter()
            .enumerate()
            .filter_map(|(ind, id)| (*id).map(|i| (ind, i)))
        {
            let v: usize = id.into();
            // println!("{v} * {ind}");
            out += v * ind;
        }
        out
    }
}

impl FromStr for Disk {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lengths = Vec::new();
        let mut total_len: usize = 0;
        for c in s.as_bytes() {
            let len = c - b'0';
            lengths.push(len);
            total_len += len as usize;
        }

        let mut out = Self::with_capacity(total_len);
        let mut iter = lengths.chunks_exact(2);
        for chunk in iter.by_ref() {
            let (file, free) = (chunk[0], chunk[1]);
            out.fill(file as usize, true);
            out.fill(free as usize, false);
        }
        let left = iter.remainder();
        if left.len() == 1 {
            out.fill(left[0] as usize, true)
        }
        Ok(out)
    }
}

pub fn part1_solution(mut lines: impl Iterator<Item = String>) -> impl Display {
    let mut disk: Disk = lines.next().unwrap().parse().unwrap();
    disk.compact();
    // println!("{disk}");
    disk.hash()
}

pub fn part2_solution(mut lines: impl Iterator<Item = String>) -> impl Display {
    let mut disk: Disk = lines.next().unwrap().parse().unwrap();
    disk.compact_without_fragmentation();
    // println!("{disk}");
    disk.hash()
}

fn main() {
    let file = File::open("input").unwrap();
    let lines = io::BufReader::new(file).lines().map(|l| l.unwrap());
    println!("Part 1: {}", part1_solution(lines));
    let file = File::open("input").unwrap();
    let lines = io::BufReader::new(file).lines().map(|l| l.unwrap());
    println!("Part 2: {}", part2_solution(lines));
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn t1() {}
}
