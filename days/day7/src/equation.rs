use std::{
    fmt::{Display, Write},
    num::ParseIntError,
    str::FromStr,
};

pub struct Equation {
    result: usize,
    values: Vec<usize>,
    dbg_str: String,
}

#[derive(Clone, Copy, Debug)]
pub enum Ops {
    Add,
    Mul,
    Concat,
}

impl Display for Ops {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Ops::Add => '+',
            Ops::Mul => '*',
            Ops::Concat => '|',
        };
        write!(f, "{c}")
    }
}

pub struct OperatorPermutationIterator<'a> {
    operation_indicies: Vec<usize>,
    done: bool,
    operators: &'a [Ops],
}

impl<'a> OperatorPermutationIterator<'a> {
    pub fn new(length: usize, operators: &'a [Ops]) -> Self {
        Self {
            operation_indicies: vec![0; length],
            done: false,
            operators,
        }
    }
}

impl<'a> Iterator for OperatorPermutationIterator<'a> {
    type Item = Vec<Ops>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        // iterates left to right.
        // Once left most == OPS_LEN - 1
        // increases right and resets left
        let out: Vec<Ops> = self
            .operation_indicies
            .iter()
            .map(|ind| self.operators[*ind])
            .collect();
        let mut iter = self.operation_indicies.iter_mut().enumerate();
        let mut got_to: Option<usize> = None;
        while let Some((i, ind)) = iter.next() {
            if *ind != self.operators.len() - 1 {
                *ind += 1;
                got_to = Some(i);
                break;
            }
        }
        let Some(got_to) = got_to else {
            self.done = true;
            return Some(out);
        };
        let len = self.operation_indicies.len();
        let rev = self
            .operation_indicies
            .iter_mut()
            .enumerate()
            .rev()
            .skip(len - got_to);
        // rev.next(); // skips one we just added to.
        for (_i, ind) in rev {
            *ind = 0;
        }
        Some(out)
    }
}

impl Equation {
    pub fn is_solvable_with(&mut self, ops: &[Ops]) -> bool {
        let ops_iter = OperatorPermutationIterator::new(self.values.len(), ops);
        for op in ops_iter {
            if self.check_if_solvable_with_ops(op) {
                return true;
            }
        }
        return false;
    }
    pub fn result(&self) -> usize {
        self.result
    }
    fn check_if_solvable_with_ops(&mut self, op: Vec<Ops>) -> bool {
        let mut value_iter = self.values.iter();
        let mut running = *value_iter.next().unwrap();
        let mut full_eq = &mut self.dbg_str;
        full_eq.clear();
        if cfg!(debug_assertions) {
            write!(&mut full_eq, "{} == ", self.result).unwrap();
            write!(&mut full_eq, "{running} ").unwrap();
        }
        for (val, op) in value_iter.zip(op.iter()) {
            if cfg!(debug_assertions) {
                write!(&mut full_eq, "{op} {val} ").unwrap();
            }
            match op {
                Ops::Add => {
                    running += *val;
                }
                Ops::Mul => {
                    running *= *val;
                }
                Ops::Concat => {
                    let val_log = val.checked_ilog10().or(Some(0)).unwrap() + 1;
                    let times_ten = 10usize.pow(val_log);
                    running = running * times_ten + val;
                }
            };
        }
        let out = running == self.result;
        if out && cfg!(debug_assertions) {
            println!("{full_eq}");
        }
        out
    }
}

impl FromStr for Equation {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (result, nums) = s.split_once(':').unwrap();
        let nums = nums.split(' ');
        let values: Result<Vec<usize>, _> = nums
            .filter(|num| !num.is_empty())
            .map(|num| num.parse())
            .collect();
        Ok(Self {
            result: result.parse()?,
            values: values?,
            dbg_str: String::with_capacity(1024),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Equation, OperatorPermutationIterator, Ops};
    const P1_OPS: [Ops; 2] = [Ops::Add, Ops::Mul];

    #[test]
    pub fn iter_test() {
        let iter = OperatorPermutationIterator::new(3, &P1_OPS);
        for i in iter {
            println!("{i:?}");
        }
    }
    #[test]
    pub fn eq_test() {
        fn eq_assert(string: &str, solvable: bool) {
            let mut eq: Equation = string.parse().unwrap();
            assert!(eq.is_solvable_with(&P1_OPS) == solvable);
        }
        eq_assert("190: 10 19", true);
        eq_assert("3267: 81 40 27", true);
        eq_assert("83: 17 5", false);
        eq_assert("156: 15 6", false);
        eq_assert("7290: 6 8 6 15", false);
        eq_assert("161011: 16 10 13", false);
        eq_assert("192: 17 8 14", false);
        eq_assert("21037: 9 7 18 13", false);
        eq_assert("292: 11 6 16 20", true);
    }
}
