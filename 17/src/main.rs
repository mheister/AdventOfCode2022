use std::{env, fs};

use chamber::Chamber;

mod chamber;

fn main() {
    let input_file_path = env::args().nth(1).unwrap_or("17/input.txt".into());
    let input = fs::read_to_string(&input_file_path).unwrap();
    let mut chamber = Chamber::new(input.parse().unwrap());
    chamber.rumble(2022);
    println!("Tower height after 2022 rocks: {}", chamber.tower_height());
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
