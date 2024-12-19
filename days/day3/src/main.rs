use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::iter::Map;
use std::str::FromStr;
use std::sync::OnceLock;

use anyhow::anyhow;
use regex::Regex;

struct SumInstruction(u16, u16);

enum Instruction {
    Sum(SumInstruction),
    Do,
    Dont,
}

impl SumInstruction {
    pub fn evaluate(&self) -> u32 {
        (self.0 as u32) * (self.1 as u32)
    }
}

static INSTR_REGEX: OnceLock<Regex> = OnceLock::new();

impl TryFrom<regex::Captures<'_>> for Instruction {
    type Error = anyhow::Error;

    fn try_from(
        value: regex::Captures<'_>,
    ) -> Result<Self, Self::Error> {
        let cmd = value.get(1).ok_or(anyhow!("Missing command."))?;
        if cmd.as_str() == "don't" {
            return Ok(Self::Dont);
        }
        if cmd.as_str() == "do" {
            return Ok(Self::Do);
        }
        let n1 = value
            .get(2)
            .ok_or(anyhow!("Missing first number in mul."))?;
        let n2 = value
            .get(3)
            .ok_or(anyhow!("Missing second number in mul."))?;

        Ok(Self::Sum(SumInstruction(
            n1.as_str().parse()?,
            n2.as_str().parse()?,
        )))
    }
}

#[derive(Default)]
struct InstructionSet(Vec<Instruction>);

impl FromStr for InstructionSet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regexp = INSTR_REGEX.get_or_init(|| {
            Regex::new(r"(?:(mul|don't|do)\(([0-9]+)?,?([0-9]+)?\))")
                .expect("Should be a valid regex")
        });
        let iter = regexp.captures_iter(s);
        let inner: Vec<Instruction> =
            iter.filter_map(|c| c.try_into().ok()).collect();
        Ok(Self(inner))
    }
}

impl InstructionSet {
    pub fn evaluate(&self, ignore_dos: bool) -> u64 {
        let mut sum = 0u64;
        let mut on = true;
        for instr in self.0.iter() {
            match instr {
                Instruction::Sum(sum_instruction) => {
                    if on || ignore_dos {
                        sum += sum_instruction.evaluate() as u64;
                    }
                }
                Instruction::Do => {
                    on = true;
                }
                Instruction::Dont => {
                    on = false;
                }
            }
        }
        sum
    }

    pub fn append(&mut self, mut other: Self) {
        self.0.append(&mut other.0);
    }
}

pub fn part1_solution(
    lines: Map<
        Lines<BufReader<File>>,
        impl FnMut(Result<String, io::Error>) -> String,
    >,
) -> impl Display {
    let mut sum = 0u64;
    for line in lines {
        let set: InstructionSet = line.parse().unwrap();
        sum += set.evaluate(true);
    }
    sum
}

pub fn part2_solution(
    lines: Map<
        Lines<BufReader<File>>,
        impl FnMut(Result<String, io::Error>) -> String,
    >,
) -> impl Display {
    let set: InstructionSet =
        lines.fold(Default::default(), |mut prev, line| {
            prev.append(line.parse().unwrap());
            prev
        });
    set.evaluate(false)
}

fn main() {
    let file = File::open("input").unwrap();
    let lines = io::BufReader::new(file).lines().map(|l| l.unwrap());
    println!("Part 1: {}", part1_solution(lines));
    let file = File::open("input").unwrap();
    let lines = io::BufReader::new(file).lines().map(|l| l.unwrap());
    println!("Part 2: {}", part2_solution(lines));
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn t1() {}
}
