use std::{env, fs};

use crate::pathfinder::find_pressure_release_potential;

mod input;
mod pathfinder;
mod preprocessing;

fn main() {
    let input_file_path = env::args().nth(1).unwrap_or("16/test_data.txt".into());
    let input = fs::read_to_string(&input_file_path).unwrap();
    let cave: input::Cave = input.parse().unwrap();
    let cave = preprocessing::Cave::from(&cave);

    let starting_positions: Vec<input::ValveLabel> = vec!["AA".parse().unwrap()];
    let p = find_pressure_release_potential(cave.clone(), starting_positions, 30).unwrap();
    println!("Part1: We can potentially release {p} units of pressure");

    let starting_positions: Vec<input::ValveLabel> =
        vec!["AA".parse().unwrap(), "AA".parse().unwrap()];
    let p = find_pressure_release_potential(cave, starting_positions, 26).unwrap();
    println!("Part2: We can potentially release {p} units of pressure");
}
