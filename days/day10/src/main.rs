use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::iter::Map;

pub fn part1_solution(lines: impl Iterator<Item = String>) -> impl Display {
    "todo"
}

pub fn part2_solution(lines: impl Iterator<Item = String>) -> impl Display {
    "todo"
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
