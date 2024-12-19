use crate::position::Position;

/// Inclusive of min and max
#[derive(Clone, Copy)]
pub struct Bounds {
    min: Position,
    max: Position,
}

impl Bounds {
    pub fn new(min: Position, max: Position) -> Self {
        assert!(min.x() <= max.x());
        assert!(min.y() <= max.y());
        Bounds { min, max }
    }

    pub fn contains(&self, pos: Position) -> bool {
        let Self { min, max } = self;
        if min.x() > pos.x() {
            return false;
        }
        if min.y() > pos.y() {
            return false;
        }

        if max.x() < pos.x() {
            return false;
        }
        if max.y() < pos.y() {
            return false;
        }

        return true;
    }
}
