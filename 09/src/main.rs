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
    let mut bridge = RopeBridge::new();
    for m in parse_movements(&input) {
        bridge.motion(m.0, m.1);
    }
    println!("Head: {},{}", bridge.head().0, bridge.head().1);
    println!("Tail: {},{}", bridge.tail().0, bridge.tail().1);
    println!(
        "Number of positions visited by tail: {}",
        bridge.count_visited_positions()
    );
}
