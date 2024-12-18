use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use crate::{
    direction::Direction, guard::Guard, obstacle::Obstacle, position::Position,
    tile::Tile,
};

pub struct Board {
    guard: Option<Guard>,
    obstacles: HashSet<Position>,
    width: usize,
    height: usize,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..(self.height as isize) {
            if y != 0 {
                writeln!(f, "")?;
            }
            for x in 0..(self.width as isize) {
                let pos = Position::new(x, y);
                let at = self.get(pos).unwrap();
                let c: char = match at {
                    BoardState::Guard => {
                        match self.guard.as_ref().unwrap().direction() {
                            Direction::UP => '^',
                            Direction::DOWN => 'v',
                            Direction::LEFT => '<',
                            Direction::RIGHT => '>',
                            _ => panic!("unsupported guard direction"),
                        }
                    }
                    BoardState::Obstacle => '#',
                    BoardState::Visited(t) => t.to_char(),
                    BoardState::Empty => '.',
                };
                write!(f, "{c}")?;
            }
        }
        Ok(())
    }
}

pub enum BoardState {
    Guard,
    Obstacle,
    Empty,
    Visited(Tile),
}

impl Board {
    pub fn get(&self, pos: Position) -> Option<BoardState> {
        if pos.x() >= self.width as isize {
            return None;
        }
        if pos.y() >= self.height as isize {
            return None;
        }
        if pos.x() < 0 {
            return None;
        }

        if pos.y() < 0 {
            return None;
        }

        if let Some(guard) = &self.guard {
            if let Some(tile) = guard.visited(pos) {
                return Some(BoardState::Visited(tile));
            }
            if guard.is_at(pos) {
                return Some(BoardState::Guard);
            }
        }
        if let Some(_obstacle) = self.obstacles.get(&pos) {
            return Some(BoardState::Obstacle);
        }
        return Some(BoardState::Empty);
    }
    pub fn take_guard(&mut self) -> Guard {
        self.guard
            .take()
            .expect("Guard should be on board if you're taking it.")
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn cropped_board(&self, bounds: (Position, Position)) -> Self {
        let (min, max) = bounds;
        let size = max - min;
        let new_guard = self
            .guard
            .as_ref()
            .map(|guard| guard.crop_guard_visits(bounds));
        let obstacles: HashSet<Position> =
            self.obstacles.iter().map(|pos| *pos - min).collect();
        let out = Self {
            guard: new_guard,
            obstacles,
            width: size.x() as usize,
            height: size.y() as usize,
        };
        out
    }

    pub fn add_obstacle(&mut self, pos: Position) {
        self.obstacles.insert(pos);
    }

    pub fn remove_obstacle(&mut self, pos: Position) {
        self.obstacles.remove(&pos);
    }

    pub fn set_guard(&mut self, guard: Guard) {
        self.guard = Some(guard)
    }
}

impl FromIterator<String> for Board {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut obstacles = HashSet::new();
        let mut guard = None;
        let mut handle_line = |y: usize, line: String| {
            for (x, byte) in line.as_bytes().iter().enumerate() {
                match byte {
                    b'^' => {
                        let pos = Position::new(x as isize, y as isize);
                        if guard.is_some() {
                            panic!("Multiple guards detected.");
                        }
                        guard = Some(Guard::new(pos));
                    }
                    b'.' => {}
                    b'#' => {
                        let pos = Position::new(x as isize, y as isize);
                        obstacles.insert(pos);
                    }
                    byte => {
                        panic!("unsupported character {}", *byte as char)
                    }
                }
            }
        };
        let mut iter = iter.into_iter().peekable();
        let mut height = 0;
        let width = iter.peek().unwrap().len();
        for line in iter {
            handle_line(height, line);
            height += 1;
        }
        let Some(guard) = guard else {
            panic!("Guard not found.")
        };
        Self {
            width,
            height,
            guard: Some(guard),
            obstacles,
        }
    }
}
