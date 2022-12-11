use std::{env, fs};

mod notes;
// use notes::*;

fn main() {
    let input_file_path = env::args().nth(1).unwrap_or("11/test_data.txt".into());
    let input = fs::read_to_string(&input_file_path).unwrap();
    let notes = notes::parse_notes(&input).unwrap();
    dbg!(&notes);
}
