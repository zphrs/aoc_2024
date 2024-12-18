use std::hash::Hash;

use bitflags::bitflags;

use crate::{direction::Direction, position::Position};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct DirectionFlags: u8 {
        const UP = 0b1;
        const DOWN = 0b10;
        const LEFT = 0b100;
        const RIGHT = 0b1000;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    visited_from: DirectionFlags,
    pos: Position,
}

impl From<Direction> for DirectionFlags {
    fn from(value: Direction) -> Self {
        match value {
            Direction::DOWN => Self::DOWN,
            Direction::UP => Self::UP,
            Direction::LEFT => Self::LEFT,
            Direction::RIGHT => Self::RIGHT,
            _ => panic!("Unsupported direction."),
        }
    }
}

impl Tile {
    pub fn visit(&mut self, direction: Direction) {
        self.visited_from.set(direction.into(), true);
    }

    pub fn visited(&mut self, direction: Direction) -> bool {
        self.visited_from.contains(direction.into())
    }

    pub fn set_pos(&mut self, pos: Position) {
        self.pos = pos;
    }

    pub fn to_char(&self) -> char {
        let vertical = self
            .visited_from
            .intersects(DirectionFlags::DOWN | DirectionFlags::UP);
        let horizontal = self
            .visited_from
            .intersects(DirectionFlags::LEFT | DirectionFlags::RIGHT);
        match (horizontal, vertical) {
            (true, true) => '+',
            (true, false) => {
                let both = DirectionFlags::LEFT & DirectionFlags::RIGHT;
                if self.visited_from == both {
                    '-'
                } else if self.visited_from.contains(DirectionFlags::LEFT) {
                    '-'
                } else if self.visited_from.contains(DirectionFlags::RIGHT) {
                    '-'
                } else {
                    unreachable!()
                }
            }
            (false, true) => {
                let both = DirectionFlags::UP & DirectionFlags::DOWN;
                if self.visited_from == both {
                    '|'
                } else if self.visited_from.contains(DirectionFlags::UP) {
                    '|'
                } else if self.visited_from.contains(DirectionFlags::DOWN) {
                    '|'
                } else {
                    unreachable!()
                }
            }
            (false, false) => panic!("Should be visited at least once!"),
        }
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl Eq for Tile {}

impl Hash for Tile {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}

impl From<Position> for Tile {
    fn from(value: Position) -> Self {
        Tile {
            pos: value,
            visited_from: DirectionFlags::empty(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{direction::Direction, position::Position};

    use super::Tile;

    #[test]
    fn test() {
        let mut tile: Tile = Position::new(0, 0).into();
        tile.visit(Direction::UP);
        tile.visit(Direction::RIGHT);
        println!("{}", tile.to_char());
    }
}
