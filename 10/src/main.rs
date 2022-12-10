use std::{env, fs};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Instruction {
    Addx(i64),
    Noop,
}

fn main() {
    let input_file_path = env::args().nth(1).unwrap_or("10/test_data.txt".into());
    let input = fs::read_to_string(&input_file_path)
        .expect(&format!("Error reading input file {input_file_path}"));
    let prog: Vec<_> = input
        .lines()
        .map(|ln| {
            if ln == "noop" {
                Instruction::Noop
            } else {
                let v = ln
                    .split_once(' ')
                    .and_then(|(_, v)| v.parse().ok())
                    .expect("Failed reading addx");
                Instruction::Addx(v)
            }
        })
        .collect();
    let mut x_value_iter = prog.iter().scan((0, 1i64), |(start_cycle, x), &inst| {
        let current_x = *x;
        (*start_cycle, *x) = match inst {
            Instruction::Addx(v) => (*start_cycle + 2, *x + v),
            Instruction::Noop => (*start_cycle + 1, *x),
        };
        Some((*start_cycle, current_x))
    });
    let signal_strength: i64 = (20..=220)
        .step_by(40)
        .map(|cycle| {
            cycle
                * x_value_iter
                    .find(|(x_value_cycle, _)| *x_value_cycle >= cycle)
                    .expect("Missing some X values")
                    .1
        })
        .sum();
    print!("{signal_strength}");
}
