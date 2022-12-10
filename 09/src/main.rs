#![allow(dead_code)]

mod ropebridge;
use std::{env, fs};

use ropebridge::*;

fn parse_movements(input: &str) -> Vec<(Direction, usize)> {
    input
        .lines()
        .map(|ln| {
            let (d, c) = ln.split_once(' ').unwrap();
            let d = match d {
                "L" => Direction::L,
                "R" => Direction::R,
                "U" => Direction::U,
                "D" => Direction::D,
                _ => unreachable!(),
            };
            (d, c.parse().unwrap())
        })
        .collect()
}

fn main() {
    let input_file_path = env::args().nth(1).unwrap_or("09/test_data.txt".into());
    let input = fs::read_to_string(&input_file_path)
        .expect(&format!("Error reading input file {input_file_path}"));
    let mut bridge_p1 = RopeBridge::<2>::new();
    for m in parse_movements(&input) {
        bridge_p1.motion(m.0, m.1);
    }
    println!("Head: {},{}", bridge_p1.head().0, bridge_p1.head().1);
    println!("Tail: {},{}", bridge_p1.tail().0, bridge_p1.tail().1);
    println!(
        "Number of positions visited by tail: {}",
        bridge_p1.count_visited_positions()
    );
}
