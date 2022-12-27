use std::{env, fs};

use chamber::Chamber;

mod chamber;

fn main() {
    let input_file_path = env::args().nth(1).unwrap_or("17/input.txt".into());
    let input = fs::read_to_string(&input_file_path).unwrap();
    let mut chamber = Chamber::new(input.parse().unwrap());
    let n_rocks_part1 = 2022;
    let n_rocks_part2 = 1_000_000_000_000;
    chamber.rumble(n_rocks_part1);
    println!(
        "Tower height after {n_rocks_part1} rocks: {}",
        chamber.tower_height()
    );
    chamber.rumble(n_rocks_part2 - n_rocks_part1);
    println!(
        "Tower height after {n_rocks_part2} rocks: {}",
        chamber.tower_height()
    );
}

#[cfg(test)]
mod example_input {
    use super::*;

    #[test]
    fn test_10_rocks() {
        let example_inp = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let mut chamber = Chamber::new(example_inp.parse().unwrap());
        chamber.rumble(10);
        assert_eq!(chamber.tower_height(), 17);
    }

    #[test]
    fn test_2022_rocks() {
        let example_inp = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let mut chamber = Chamber::new(example_inp.parse().unwrap());
        chamber.rumble(2022);
        assert_eq!(chamber.tower_height(), 3068);
    }
}
