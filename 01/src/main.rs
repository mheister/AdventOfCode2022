use std::{
    env,
    fs::File,
    io::{self, BufRead},
};

fn get_calories_by_elve(
    input_lines: &mut dyn Iterator<Item = Option<String>>,
) -> Vec<u32> {
    input_lines.fold(vec![0u32], |mut acc, line| {
        let line = line.unwrap();
        if line.is_empty() {
            acc.push(0);
        } else {
            let item = line
                .parse::<u32>()
                .expect(format!("Error parsing {line}").as_str());
            *acc.last_mut().unwrap() += item;
        }
        acc
    })
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input_file_path = &args[1];
    let file = File::open(input_file_path)
        .expect(format!("Could not open file '{input_file_path}'").as_str());
    let mut calories =
        get_calories_by_elve(&mut io::BufReader::new(file).lines().map(|x| match x {
            Ok(line) => Some(line),
            Err(_) => None,
        }));
    calories.sort();
    calories.reverse();

    // let max_calories = calories.iter().cloned().fold(0u32, u32::max);
    let max_calories = calories[0];
    println!("Max calories: {max_calories}");

    // let max3_calories = calories.iter().cloned().fold([0u32; 3], |acc, elve| acc);
    let max3_calories : Vec<u32> = calories.iter().cloned().take(3).collect();
    let max3_sum:u32 = max3_calories.iter().sum();
    println!("Max 3 calories: {max3_calories:?}, {max3_sum} total.");
}
