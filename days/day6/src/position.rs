use std::{fmt::Display, ops::Sub};

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position(isize, isize);

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.sub(rhs.0), self.1.sub(rhs.1))
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Position {
    pub fn new(x: isize, y: isize) -> Position {
        Position(x, y)
    }

    pub fn x(&self) -> isize {
        self.0
    }

    pub fn y(&self) -> isize {
        self.1
    }
}

impl<A, B> From<(A, B)> for Position
where
    A: Into<isize>,
    B: Into<isize>,
{
    fn from(value: (A, B)) -> Self {
        Position::new(value.0.into(), value.1.into())
    }
}
