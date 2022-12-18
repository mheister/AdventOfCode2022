mod cave;

use std::{env, fs};
use cave::Cave;

fn main() {
    let input_file_path = env::args().nth(1).unwrap_or("14/input.txt".into());
    let input = fs::read_to_string(&input_file_path).unwrap();
    let mut cave :Cave= input.parse().unwrap();
    cave.fill_sand();
    let sand_count = cave.count_sand();
    // println!("{}", cave.to_string());
    println!("{sand_count} units of sand have come to rest");
}
