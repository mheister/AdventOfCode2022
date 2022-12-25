use std::{env, fs};

use crate::pathfinder::find_pressure_release_potential;

mod input;
mod preprocessing;
mod pathfinder;

fn main() {
    let input_file_path = env::args().nth(1).unwrap_or("16/test_data.txt".into());
    let input = fs::read_to_string(&input_file_path).unwrap();
    let cave: input::Cave = input.parse().unwrap();
    let cave = preprocessing::Cave::from(&cave);
    let p = find_pressure_release_potential(cave).unwrap();
    println!("Part1: We can potentially release {p} units of pressure")
}
