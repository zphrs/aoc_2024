use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};

pub fn part1_solution(lines: Lines<BufReader<File>>) -> impl Display {
    todo!();
    "todo"
}

pub fn part2_solution(lines: Lines<BufReader<File>>) -> impl Display {
    todo!();
    "todo"
}

fn main() {
    let file = File::open("input").unwrap();
    let lines = io::BufReader::new(file).lines();
    println!("Part 1: {}", part1_solution(lines));
    let file = File::open("input").unwrap();
    let lines = io::BufReader::new(file).lines();
    println!("Part 2: {}", part2_solution(lines));
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn t1() {}
}
