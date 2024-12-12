use std::{
    fs::File,
    io::{self, BufRead},
    num::ParseIntError,
    ops::Sub,
    str::FromStr,
};

use thiserror::Error;

struct Report(Vec<u8>);
#[derive(Clone, Copy)]
struct Pair<T>(Element<T>, Element<T>)
where
    T: Sub<T, Output = T>;

impl<T: std::ops::Sub<Output = T>> From<(Element<T>, Element<T>)> for Pair<T> {
    fn from(value: (Element<T>, Element<T>)) -> Self {
        Pair(value.0, value.1)
    }
}

impl Pair<i16> {
    pub fn direction(&self) -> i16 {
        let Self(c, n) = *self;
        (n - c).signum()
    }
    pub fn is_valid(
        &self,
        prev_elem: Option<Element<i16>>,
        correct_direction: i16,
    ) -> Result<(), Error<i16>> {
        let Self(c, n) = *self;
        let diff = n - c;
        let sign = diff.signum();
        if sign == 0 {
            return Err(Error::diff_too_small(*self));
        }
        if let Some(prev_elem) = prev_elem {
            if correct_direction != sign {
                return Err(Error::pyramid(prev_elem, self.0, self.1));
            }
        }
        if diff.abs() > 3 {
            return Err(Error::diff_too_big(*self));
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct Element<T>(T, usize);

impl<T> From<(usize, T)> for Element<T> {
    fn from(value: (usize, T)) -> Self {
        Element(value.1, value.0)
    }
}

impl<T> Sub for Element<T>
where
    T: Sub<Output = T>,
{
    type Output = T;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

#[derive(Error)]
enum Error<T> {
    #[error("Difference between previous {0} and next {1} is too small.")]
    DiffTooSmall(Element<T>, Element<T>),
    #[error("Difference between previous {0} and next {1} is too big.")]
    DiffTooBig(Element<T>, Element<T>),
    #[error("Direction Pyramid: {0}, {1}, and {2}.")]
    Pyramid(Element<T>, Element<T>, Element<T>),
}

impl<T> Error<T>
where
    T: Sub<Output = T>,
{
    pub fn diff_too_small(pair: Pair<T>) -> Self {
        Self::DiffTooSmall(pair.0, pair.1)
    }

    pub fn diff_too_big(pair: Pair<T>) -> Self {
        Self::DiffTooBig(pair.0, pair.1)
    }

    pub fn pyramid(left: Element<T>, middle: Element<T>, right: Element<T>) -> Self {
        Self::Pyramid(left, middle, right)
    }
}

impl Report {
    pub fn is_valid(&self) -> Result<(), Error<i16>> {
        let curr = self.0.iter().enumerate();
        let mut next = curr.clone();
        next.next();
        let mut iter = curr.zip(next).map::<Pair<i16>, _>(|(c, n)| {
            ((c.0, *c.1 as i16).into(), (n.0, *n.1 as i16).into()).into()
        });
        let Some(pair) = iter.next() else {
            return Ok(());
        };
        let direction = pair.direction();
        pair.is_valid(None, direction)?;
        let mut prev = pair.0;
        for pair in iter {
            pair.is_valid(Some(prev), direction)?;
            prev = pair.0;
        }
        Ok(())
    }

    pub fn to_fixed(&self, elem: Element<i16>) -> Self {
        Self(
            self.0
                .iter()
                .enumerate()
                .filter_map(|(i, e)| if i == elem.1 { None } else { Some(*e) })
                .collect(),
        )
    }

    fn to_fixes(&self, error: Error<i16>) -> Vec<Report> {
        let elems = match error {
            Error::DiffTooSmall(element, element1) => {
                vec![element, element1]
            }
            Error::DiffTooBig(element, element1) => {
                vec![element, element1]
            }
            Error::Pyramid(prev, curr, next) => {
                vec![prev, curr, next]
            }
        };
        elems.into_iter().map(|elem| self.to_fixed(elem)).collect()
    }
    pub fn is_valid_with_fix(&self) -> bool {
        let Err(e) = self.is_valid() else {
            return true;
        };
        let to_try = self.to_fixes(e);
        for attempts in to_try {
            if attempts.is_valid().is_ok() {
                return true;
            }
        }
        false
    }
    pub fn _naive_is_valid_with_fix(&self) -> bool {
        let Err(_) = self.is_valid() else {
            return true;
        };
        for e in self.0.iter().enumerate() {
            let elem: Element<i16> = (e.0, 0).into();
            if self.to_fixed(elem).is_valid().is_ok() {
                return true;
            }
        }
        false
    }
}

impl FromStr for Report {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner: Result<Vec<u8>, _> = s.split(' ').map(|s| s.parse()).collect();
        Ok(Self(inner?))
    }
}

fn main() {
    let file = File::open("input").unwrap();

    let records: Result<Vec<Report>, _> = io::BufReader::new(file)
        .lines()
        .map(|f| f.unwrap().parse())
        .collect();
    let records = records.unwrap();
    let valid_record_count = records.iter().fold(0, |total, r| {
        if r.is_valid().is_ok() {
            total + 1
        } else {
            total
        }
    });
    println!("part 1: {valid_record_count}");
    let valid_record_count = records.iter().fold(0, |total, r| {
        // if r.is_valid_with_fix() != r._naive_is_valid_with_fix() {
        //     println!("{:?}", r.0);
        // }
        // if r._naive_is_valid_with_fix() {
        if r.is_valid_with_fix() {
            total + 1
        } else {
            total
        }
    });
    println!("part 2: {valid_record_count}");
}

#[cfg(test)]
mod tests {
    use crate::Report;
    #[test]
    pub fn simple() {
        let report: Report = "1 2 3 4 5".parse().unwrap();
        assert!(report.is_valid().is_ok())
    }
    #[test]
    pub fn big_gap() {
        let report: Report = "1 5".parse().unwrap();
        assert!(report.is_valid().is_err())
    }
}
