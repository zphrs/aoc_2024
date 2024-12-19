mod equation;

use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

use equation::{Equation, Ops};

pub fn part1_solution(lines: impl Iterator<Item = String>) -> impl Display {
    let mut out = 0;
    let ops = vec![Ops::Add, Ops::Mul];
    for line in lines {
        let mut eq: Equation = line.parse().unwrap();
        if eq.is_solvable_with(&ops) {
            out += eq.result();
        }
    }
    out
}

pub fn part2_solution(lines: impl Iterator<Item = String>) -> impl Display {
    let mut out = 0;
    let ops = vec![Ops::Add, Ops::Mul, Ops::Concat];
    for line in lines {
        let mut eq: Equation = line.parse().unwrap();
        if eq.is_solvable_with(&ops) {
            out += eq.result();
        }
    }
    out
}

fn main() {
    let file = File::open("input").unwrap();
    let lines = io::BufReader::new(file).lines().map(|l| l.unwrap());
    println!("Part 1: {}", part1_solution(lines));
    let file = File::open("input").unwrap();
    let lines = io::BufReader::new(file).lines().map(|l| l.unwrap());
    let now = Instant::now();
    println!(
        "Part 2: {}, took {:?}",
        part2_solution(lines),
        now.elapsed()
    );
}
