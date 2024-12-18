mod page_id;
mod page_ids;
mod rules;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::iter::Map;
use std::time::Instant;

use anyhow::Context;
use page_ids::PageIds;
use rules::RulesMap;

pub fn part1_solution(
    lines: Map<
        Lines<BufReader<File>>,
        impl FnMut(Result<String, io::Error>) -> String,
    >,
) -> impl Display {
    let mut lines = lines.into_iter();
    let map: RulesMap = lines.by_ref().take_while(|l| !l.is_empty()).collect();
    let mut total_middles: usize = 0;
    for line in lines {
        let ids: PageIds = line.parse().unwrap();
        match ids
            .validate_against_ruleset(&map)
            .with_context(|| format!("For update {}", ids))
        {
            Ok(_) => {
                total_middles += ids.middle_id();
            }
            Err(e) => {
                continue;
            }
        }
    }
    total_middles
}

pub fn part2_solution(
    lines: Map<
        Lines<BufReader<File>>,
        impl FnMut(Result<String, io::Error>) -> String,
    >,
) -> impl Display {
    let mut lines = lines.into_iter();
    let map: RulesMap = lines.by_ref().take_while(|l| !l.is_empty()).collect();
    let mut total_middles: usize = 0;
    for line in lines {
        let ids: PageIds = line.parse().unwrap();
        match ids
            .validate_against_ruleset(&map)
            .with_context(|| format!("For update {}", ids))
        {
            Ok(_) => {
                continue;
            }
            Err(e) => {
                let new_ids = PageIds::from_inner(
                    map.walk_graph(&ids.into_inner()).unwrap(),
                );
                total_middles += new_ids.middle_id()
            }
        }
    }
    total_middles
}

fn main() {
    let file = File::open("input").unwrap();
    let lines = io::BufReader::new(file).lines().map(|l| l.unwrap());
    println!("Part 1: {}", part1_solution(lines));
    let now = Instant::now();
    let file = File::open("input").unwrap();
    let lines = io::BufReader::new(file).lines().map(|l| l.unwrap());
    println!("Part 2: {} took {:?}", part2_solution(lines), now.elapsed());
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn t1() {}
}
