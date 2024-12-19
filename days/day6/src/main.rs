mod board;
mod direction;
mod guard;
mod position;
mod tile;

use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::iter::Map;

use board::Board;
use guard::{Error, Guard};

pub fn part1_solution(
    lines: Map<
        Lines<BufReader<File>>,
        impl FnMut(Result<String, io::Error>) -> String,
    >,
) -> impl Display {
    let mut board: Board = lines.collect();
    let mut guard: Guard = board.take_guard();
    let _e = guard.simulate(&mut board);
    let all_pos_visited: usize = guard.num_visited();
    if cfg!(debug_assertions) {
        board.set_guard(guard);
        println!("{board}");
    }
    all_pos_visited
}

pub fn part2_solution(
    lines: Map<
        Lines<BufReader<File>>,
        impl FnMut(Result<String, io::Error>) -> String,
    >,
) -> impl Display {
    let mut board: Board = lines.collect();
    let mut guard: Guard = board.take_guard();
    let _e = guard.simulate(&mut board);
    let (visited, mut reset) = guard.into();
    let original_pos = reset.pos();
    let mut loop_ct = 0;
    for (_i, (pos, _tile)) in visited.iter().enumerate() {
        if *pos == original_pos {
            continue;
        }
        // println!("Simulated {}/{}", i, visited.len());
        board.add_obstacle(*pos);
        if let Error::Loop = reset.simulate(&mut board) {
            loop_ct += 1;
            if cfg!(debug_assertions) {
                let old_pos = reset.pos();
                let old_direction = reset.direction();
                let mut new_guard = Guard::new(old_pos);
                new_guard.set_direction(old_direction);
                new_guard.simulate(&mut board);
                let bounds = new_guard.get_guard_bounds();
                board.set_guard(new_guard);
                let cropped = board.cropped_board(bounds);
                if cropped.width() * cropped.height() < 64 {
                    println!("{}\n{cropped}", bounds.0);
                }
                let _ = board.take_guard();
            }
        }
        reset = reset.reset();
        board.remove_obstacle(*pos);
    }
    loop_ct
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
