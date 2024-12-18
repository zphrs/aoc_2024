use crate::position::Position;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct Direction {
    x: i8,
    y: i8,
}

impl Direction {
    pub const UP: Direction = Direction { x: 0, y: -1 };
    pub const DOWN: Direction = Direction { x: 0, y: 1 };
    pub const LEFT: Direction = Direction { x: -1, y: 0 };
    pub const RIGHT: Direction = Direction { x: 1, y: 0 };

    pub fn rotate_right_90(&self) -> Self {
        if *self == Self::UP {
            return Self::RIGHT;
        }
        match *self {
            Self::UP => Self::RIGHT,
            Self::RIGHT => Self::DOWN,
            Self::DOWN => Self::LEFT,
            Self::LEFT => Self::UP,
            _ => {
                panic!("Rotating only supports the directions up, down, left, right")
            }
        }
    }

    pub fn invert(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }

    pub fn step_pos(&self, pos: Position) -> Position {
        let new_x = pos.x() + self.x as isize;
        let new_y = pos.y() + self.y as isize;
        (new_x, new_y).into()
    }
}
