mod cave;

use cave::Cave;
use std::{env, fs};

fn main() {
    let input_file_path = env::args().nth(1).unwrap_or("14/input.txt".into());
    let input = fs::read_to_string(&input_file_path).unwrap();
    let mut cave: Cave = input.parse().unwrap();
    cave.fill_sand();
    let sand_count = cave.count_sand();
    // println!("{}", cave.to_string());
    println!("{sand_count} units of sand have come to rest");
    let mut cave2 = Cave::from_str_with_bottom(&input).unwrap();
    cave2.fill_sand();
    let sand_count = cave2.count_sand();
    println!("{sand_count} units of sand have come to rest in the cave with bottom");
}
