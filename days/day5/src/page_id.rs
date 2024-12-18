use std::{fmt::Display, num::ParseIntError, ops::AddAssign, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PageId(u8);

impl Display for PageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        u8::fmt(&self.0, f)
    }
}

impl FromStr for PageId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num: u8 = s.parse()?;
        Ok(PageId(num))
    }
}

impl AddAssign<PageId> for usize {
    fn add_assign(&mut self, rhs: PageId) {
        *self += rhs.0 as usize;
    }
}
