use std::{env, fs};

mod cave;

fn main() {
    let input_file_path = env::args().nth(1).unwrap_or("16/input.txt".into());
    let input = fs::read_to_string(&input_file_path).unwrap();
    let cave: cave::Cave = input.parse().unwrap();
    dbg!(cave);
}
