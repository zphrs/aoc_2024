use std::{fmt::Display, ops::{Add, AddAssign, Neg, Sub, SubAssign}};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position(isize, isize);
impl Position {
    pub fn new(x: isize, y: isize) -> Self {
        Self(x, y)
    }

    pub fn x(&self) -> isize {
        self.0
    }

    pub fn y(&self) -> isize {
        self.1
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for Position {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Neg for Position {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1)
    }
}
