use std::{collections::HashSet, hash::Hash};

use crate::{direction::Direction, position::Position};

pub struct Obstacle {
    hit_from: HashSet<Direction>,
    position: Position,
}

impl Obstacle {
    pub fn new(position: Position) -> Self {
        Obstacle {
            hit_from: HashSet::new(),
            position,
        }
    }

    pub fn hit(&mut self, direction: Direction) {
        self.hit_from.insert(direction.invert());
    }

    pub fn pos(&mut self) -> Position {
        self.position
    }

    pub fn has_been_hit(&self) -> bool {
        !self.hit_from.is_empty()
    }
}

impl From<Position> for Obstacle {
    fn from(value: Position) -> Self {
        Self::new(value)
    }
}

impl Hash for Obstacle {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.position.hash(state);
    }
}
