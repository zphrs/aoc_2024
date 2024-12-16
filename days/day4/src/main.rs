use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::iter::Map;

struct Board {
    width: usize,
    board: Vec<u8>,
}

impl Board {
    pub fn get(&self, x: usize, y: usize) -> u8 {
        debug_assert!(x <= self.width);
        debug_assert!(y <= self.height());
        *self.board.get(x + y * self.width).unwrap()
    }

    pub fn height(&self) -> usize {
        self.board.len() / self.width
    }

    pub fn get_cursor<'a>(
        &'a self,
        x: usize,
        y: usize,
    ) -> Option<Cursor> {
        let width = self.width;
        let height = self.height();
        if x >= width {
            return None;
        }
        if y >= height {
            return None;
        }
        Cursor::new(x, y, self.width, self.height())
    }

    pub fn find_x(&self) -> usize {
        let a_pos: Vec<Cursor> = self.board.iter().enumerate().fold(
            Vec::new(),
            |mut v, b| {
                if *b.1 == b'A' {
                    let x = b.0 % self.width;
                    let y = b.0 / self.width;
                    v.push(self.get_cursor(x, y).unwrap());
                }
                v
            },
        );

        let iters = a_pos.iter().map(|c| c.get_x_iters(self));
        let mut out = 0;
        const MAS: [u8; 3] = *b"MAS";
        for iter in iters {
            let Some((d1, d2)) = iter else {
                continue;
            };

            if (d1.0.eq(MAS) || d1.1.eq(MAS))
                && (d2.0.eq(MAS) || d2.1.eq(MAS))
            {
                out += 1;
            }
        }
        out
    }

    pub fn find(&self, term: &[u8]) -> usize {
        let starting_cursors: Vec<Cursor> = self
            .board
            .iter()
            .enumerate()
            .fold(Vec::new(), |mut v, b| {
                if *b.1 == term[0] {
                    let x = b.0 % self.width;
                    let y = b.0 / self.width;
                    v.push(self.get_cursor(x, y).unwrap());
                }
                v
            });
        let iters = starting_cursors
            .iter()
            .map(|i| {
                i.get_all_iterators(self, term.len()).into_iter()
            })
            .flatten();
        let mut total = 0;
        for iter in iters {
            if iter.eq(term.to_owned()) {
                total += 1;
            }
        }
        total
    }
}

impl FromIterator<String> for Board {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let l1 = iter.next().unwrap();
        let mut board = Board {
            width: l1.len(),
            board: l1.into_bytes(),
        };
        for line in iter {
            board.board.extend_from_slice(line.as_bytes());
        }
        board
    }
}

#[derive(Clone, Copy, Debug)]
struct Cursor {
    x: usize,
    y: usize,
    board_width: usize,
    board_height: usize,
}

impl Cursor {
    pub fn new(
        x: usize,
        y: usize,
        board_width: usize,
        board_height: usize,
    ) -> Option<Self> {
        if x >= board_width {
            return None;
        }
        if y >= board_height {
            return None;
        }
        return Some(Self {
            x,
            y,
            board_width,
            board_height,
        });
    }
    pub fn shift(&self, direction: &Direction) -> Option<Self> {
        let Self {
            mut x,
            mut y,
            board_width,
            board_height,
        } = *self;
        let Direction {
            x: horizontal,
            y: vertical,
        } = *direction;
        if horizontal < 0 {
            if x == 0 {
                return None;
            }
            x -= 1;
        } else if horizontal > 0 {
            x += 1;
        }
        if vertical < 0 {
            if y == 0 {
                return None;
            }
            y -= 1;
        } else if vertical > 0 {
            y += 1;
        }
        Self::new(x, y, board_width, board_height)
    }
    pub fn to_iterator<'a>(
        &self,
        direction: Direction,
        board: &'a Board,
        length: usize,
    ) -> Option<CursorIterator<'a>> {
        CursorIterator::new(direction, *self, board, length)
    }

    pub fn get_all_iterators<'a>(
        &self,
        board: &'a Board,
        length: usize,
    ) -> Vec<CursorIterator<'a>> {
        let mut out = Vec::with_capacity(8);
        for x in -1i8..=1i8 {
            for y in -1i8..=1i8 {
                if x == 0 && y == 0 {
                    continue;
                }
                if let Some(iter) = self.to_iterator(
                    Direction { x, y },
                    board,
                    length,
                ) {
                    out.push(iter);
                }
            }
        }
        out
    }

    pub fn get_x_iters<'a>(
        &self,
        board: &'a Board,
    ) -> Option<(
        (CursorIterator<'a>, CursorIterator<'a>),
        (CursorIterator<'a>, CursorIterator<'a>),
    )> {
        let dir1 = Direction { x: 1, y: 1 };
        let op_dir1 = dir1.reversed();
        let dir2 = Direction { x: 1, y: -1 };
        let op_dir2 = dir2.reversed();
        let shifted_op = self.shift(&op_dir1)?;
        let iter1 = shifted_op.to_iterator(dir1, board, 3)?;
        let op_iter1 =
            self.shift(&dir1)?.to_iterator(op_dir1, board, 3)?;
        let iter2 =
            self.shift(&op_dir2)?.to_iterator(dir2, board, 3)?;
        let op_iter2 =
            self.shift(&dir2)?.to_iterator(op_dir2, board, 3)?;
        Some(((iter1, op_iter1), (iter2, op_iter2)))
    }
}

#[derive(Clone, Copy, Default, Debug)]
struct Direction {
    x: i8,
    y: i8,
}

impl Direction {
    pub fn reversed(&self) -> Self {
        Direction {
            x: -&self.x,
            y: -&self.y,
        }
    }
}

struct CursorIterator<'a> {
    direction: Direction,
    current: Cursor,
    board: &'a Board,
    left: usize,
}

impl Debug for CursorIterator<'_> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        #[derive(Debug)]
        struct Iter {
            direction: Direction,
            current: Cursor,
            left: usize,
        }
        let Self {
            direction,
            current,
            board: _,
            left,
        } = self;

        // per Chayim Friedman’s suggestion
        Debug::fmt(
            &Iter {
                direction: *direction,
                current: *current,
                left: *left,
            },
            f,
        )
    }
}

impl<'a> CursorIterator<'a> {
    pub fn new(
        direction: Direction,
        current: Cursor,
        board: &'a Board,
        length: usize,
    ) -> Option<Self> {
        let final_x: isize = current.x as isize
            + direction.x as isize * (length - 1) as isize;
        let final_y: isize = current.y as isize
            + direction.y as isize * (length - 1) as isize;
        if final_x < 0 || final_y < 0 {
            return None;
        }
        if final_x >= board.width as isize {
            return None;
        }
        if final_y >= board.height() as isize {
            return None;
        }
        Some(Self {
            direction,
            current,
            board,
            left: length,
        })
    }
}

impl<'a> Iterator for CursorIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.left == 0 {
            None
        } else {
            let out =
                Some(self.board.get(self.current.x, self.current.y));
            self.left -= 1;
            if self.left > 0 {
                self.current =
                    self.current.shift(&self.direction).expect(
                        "Verified in bounds at Iterator creation so bounds should be properly constrained.",
                    );
            }
            out
        }
    }
}

pub fn part1_solution(
    lines: Map<
        Lines<BufReader<File>>,
        impl FnMut(Result<String, io::Error>) -> String,
    >,
) -> impl Display {
    let board: Board = Board::from_iter(lines);
    board.find(b"XMAS")
}

pub fn part2_solution(
    lines: Map<
        Lines<BufReader<File>>,
        impl FnMut(Result<String, io::Error>) -> String,
    >,
) -> impl Display {
    let board: Board = Board::from_iter(lines);
    board.find_x()
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
    use crate::{Board, Direction};

    #[test]
    pub fn t1() {
        let board = Board {
            width: 4,
            board: b"XMAS".into(),
        };
        let cursor = board.get_cursor(0, 0).unwrap();
        let iter = cursor
            .to_iterator(Direction { x: 1, y: 0 }, &board, 4)
            .unwrap();
        for i in iter {
            println!("{}", char::from(i));
        }
    }
}
