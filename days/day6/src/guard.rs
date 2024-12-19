use std::collections::HashMap;
use std::{isize, usize};

use crate::board::{Board, BoardState};
use crate::direction::Direction;
use crate::position::Position;
use crate::tile::Tile;

pub struct Guard {
    pos: Position,
    visited: HashMap<Position, Tile>,
    direction: Direction,
    initial_pos: Position,
}

#[derive(Debug)]
pub enum Error {
    OffMap,
    Loop,
}

impl From<Guard> for (HashMap<Position, Tile>, Guard) {
    fn from(value: Guard) -> Self {
        let reset = value.reset();
        (value.visited, reset)
    }
}

impl Guard {
    pub fn new(pos: Position) -> Self {
        let mut tile: Tile = pos.into();
        tile.visit(Direction::UP);
        Self {
            pos,
            initial_pos: pos,
            visited: HashMap::from([(pos, tile)]),
            direction: Direction::UP,
        }
    }

    pub fn get_guard_bounds(&self) -> (Position, Position) {
        let mut min_x = isize::MAX;
        let mut max_x = 0;
        let mut min_y = isize::MAX;
        let mut max_y: isize = 0;
        for (pos, _) in self.visited.iter() {
            let x = pos.x();
            let y = pos.y();
            if x < min_x {
                min_x = x;
            }
            if x > max_x {
                max_x = x;
            }
            if y < min_y {
                min_y = y;
            }
            if y > max_y {
                max_y = y;
            }
        }
        (
            Position::new(min_x - 1, min_y - 1),
            Position::new(max_x + 2, max_y + 2),
        )
    }
    pub fn crop_guard_visits(&self, bounds: (Position, Position)) -> Self {
        let (min, _max) = bounds;
        let mut out = Self::new(self.pos() - min);
        for (pos, tile) in self.visited.iter() {
            let mut tile = *tile;
            let pos = *pos - min;
            tile.set_pos(pos);
            out.visited.insert(pos, tile);
        }
        out
    }
    pub fn pos(&self) -> Position {
        self.pos
    }
    pub fn reset(&self) -> Self {
        Self::new(self.initial_pos)
    }
    /// - returns an error if a step will take guard off the board
    /// - otherwise will return None if guard just rotated
    /// - otherwise will return Some(Position) if guard just moved forward.
    pub fn step(
        &mut self,
        board: &mut Board,
    ) -> Result<Option<Position>, Error> {
        let next = self.direction.step_pos(self.pos);
        let Some(next_state) = board.get(next) else {
            return Err(Error::OffMap);
        };

        match next_state {
            BoardState::Guard => {
                panic!("Shouldn't be another guard.");
            }
            BoardState::Obstacle => {
                self.direction = self.direction.rotate_right_90();
                let mut tile = *self.visited.get(&self.pos).unwrap();
                tile.visit(self.direction);
                self.visited.insert(self.pos, tile);
                Ok(None)
            }
            BoardState::Visited(_) | BoardState::Empty => {
                self.pos = next;
                let mut tile: Tile = self
                    .visited
                    .get(&self.pos)
                    .copied()
                    .or(Some(self.pos.into()))
                    .unwrap();
                if tile.visited(self.direction) {
                    return Err(Error::Loop);
                }
                tile.visit(self.direction);
                self.visited.insert(self.pos, tile);
                Ok(Some(self.pos))
            }
        }
    }

    pub fn simulate(&mut self, board: &mut Board) -> Error {
        loop {
            if let Err(e) = self.step(board) {
                return e;
            }
        }
    }
    pub fn is_at(&self, pos: Position) -> bool {
        self.pos == pos
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn num_visited(&self) -> usize {
        self.visited.len()
    }

    pub fn visited(&self, pos: Position) -> Option<Tile> {
        self.visited.get(&pos).copied()
    }

    pub fn positions(&self) -> &HashMap<Position, Tile> {
        &self.visited
    }
}
