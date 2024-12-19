mod bounds;
mod position;

use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::iter::{Map, Peekable};
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

use bounds::Bounds;
use position::Position;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Frequency(char);

impl Frequency {
    pub fn new(frequency: char) -> Self {
        Self(frequency)
    }
}

#[derive(Clone, Copy)]
pub struct Antenna {
    frequency: Frequency,
    pos: Position,
}

impl Antenna {
    pub fn new(freq: Frequency, pos: Position) -> Self {
        Self {
            frequency: freq,
            pos,
        }
    }

    pub fn char(&self) -> char {
        self.frequency.0
    }

    pub fn pos(&self) -> Position {
        self.pos
    }

    pub fn get_antinodes_with(
        &self,
        other: Antenna,
        bounds: Option<Bounds>,
    ) -> HashSet<Position> {
        let diff = self.pos - other.pos;
        let mut out = HashSet::with_capacity(2);
        let Some(bounds) = bounds else {
            out.insert(self.pos + diff);
            out.insert(other.pos - diff);
            return out;
        };
        let mut pos = self.pos;
        while bounds.contains(pos) {
            out.insert(pos);
            pos += diff;
        }
        let mut pos = other.pos;
        while bounds.contains(pos) {
            out.insert(pos.clone());
            pos -= diff;
        }
        out
    }
}

pub struct AntennaSet(Vec<Antenna>);

impl AntennaSet {
    pub fn add(&mut self, antenna: Antenna) {
        self.0.push(antenna);
    }

    pub fn get_antinodes(&self, bounds: Option<Bounds>) -> HashSet<Position> {
        let mut out = HashSet::new();
        for (i, a) in self.0.iter().enumerate() {
            for b in &self.0[i + 1..] {
                out.extend(a.get_antinodes_with(*b, bounds));
            }
        }
        out
    }

    pub fn write_into(&self, s: &mut [u8], width: isize) {
        for antenna in &self.0 {
            let p = antenna.pos();
            let str_offset = p.y() * width + p.x();
            let c = antenna.char();
            s[str_offset as usize] = c as u8;
        }
    }
}

impl From<Vec<Antenna>> for AntennaSet {
    fn from(value: Vec<Antenna>) -> Self {
        Self(value)
    }
}

pub struct City {
    width: usize,
    height: usize,
    antennas: HashMap<Frequency, AntennaSet>,
}

impl Display for City {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out: Vec<u8> = vec![b'.'; self.height * self.width];
        for (f, antenna) in &self.antennas {
            antenna.write_into(out.as_mut(), self.width as isize);
        }
        for pos in self.get_all_antinodes(f.alternate()) {
            println!("{pos}");
            out[pos.x() as usize + pos.y() as usize * self.width] = b'#';
        }
        for y in 0..self.width {
            for x in 0..self.height {
                write!(f, "{}", out[x + y * self.width] as char)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl City {
    pub fn count_antinodes(&self, part2: bool) -> usize {
        self.get_all_antinodes(part2).len()
    }

    pub fn get_all_antinodes(&self, part2: bool) -> HashSet<Position> {
        let mut combined: HashSet<Position> = HashSet::new();
        for (freq, antenna_set) in &self.antennas {
            let bounds = part2.then(|| {
                Bounds::new(
                    Position::new(0, 0),
                    Position::new(
                        (self.width - 1) as isize,
                        (self.height - 1) as isize,
                    ),
                )
            });
            let antinodes = antenna_set.get_antinodes(bounds);
            if bounds.is_some() {
                combined.extend(antinodes);
            } else {
                combined.extend(antinodes.iter().filter(|p| {
                    p.x() >= 0
                        && p.y() >= 0
                        && p.x() < self.width as isize
                        && p.y() < self.height as isize
                }))
            }
        }
        combined
    }
}

impl FromIterator<String> for City {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut antennas: HashMap<Frequency, AntennaSet> = HashMap::new();
        let mut parse_line = |s: String, y: usize| {
            for (x, c) in s.chars().enumerate() {
                if c == '.' {
                    continue;
                }
                let freq = Frequency::new(c);
                let antenna =
                    Antenna::new(freq, Position::new(x as isize, y as isize));
                antennas
                    .entry(freq)
                    .and_modify(|v| v.add(antenna))
                    .or_insert(vec![antenna].into());
            }
        };
        let mut height = 0;
        let iter = iter.into_iter();
        let mut iter = iter.peekable();
        let width = iter.peek().unwrap().len();
        for line in iter {
            parse_line(line, height);
            height += 1;
        }
        Self {
            antennas,
            height,
            width,
        }
    }
}

pub fn part1_solution(lines: impl Iterator<Item = String>) -> impl Display {
    let city: City = lines.collect();
    println!("{city}");
    city.count_antinodes(false)
}

pub fn part2_solution(lines: impl Iterator<Item = String>) -> impl Display {
    let city: City = lines.collect();
    println!("w: {} h: {}", city.width, city.height);
    println!("{city:#}");
    city.count_antinodes(true)
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
