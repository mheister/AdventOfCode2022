use std::{env, fs};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Move {
    n: usize,
    from: usize,
    to: usize,
}

fn main() {
    let input_file_path = env::args()
        .nth(1)
        .unwrap_or("../../05/test_data.txt".into());
    let input = fs::read_to_string(&input_file_path)
        .expect(&format!("Error reading input file {input_file_path}"));
    let (stacks, procedure) = input.split_once("\n\n").expect(
        "Expecting two paragraphs in input: intial stacks and rearrangement procedure",
    );
    let stacks = {
        let mut result: Vec<Vec<char>> = vec![];
        for line in stacks.lines().rev().skip(1) {
            for (idx, crate_id) in line.chars().skip(1).step_by(4).enumerate() {
                if result.len() <= idx {
                    result.push(vec![]);
                }
                if crate_id == ' ' {
                    continue;
                }
                result[idx].push(crate_id);
            }
        }
        result
    };
    let procedure = procedure
        .lines()
        .map(|ln| {
            let mut words = ln.split_whitespace();
            Move {
                n: words
                    .nth(1)
                    .and_then(|n| n.parse().ok())
                    .expect("Unable to parse amount of crates"),
                from: words
                    .nth(1)
                    .and_then(|n| n.parse().ok())
                    .expect("Unable to parse source stack index"),
                to: words
                    .nth(1)
                    .and_then(|n| n.parse().ok())
                    .expect("Unable to parse target stack index"),
            }
        })
        .collect::<Vec<_>>();
    {
        let mut stacks_part1 = stacks.clone();
        for mov in &procedure {
            assert!(mov.to != mov.from);
            let from = &stacks_part1[mov.from - 1];
            let moved_crates = from[from.len() - mov.n..].to_owned();
            moved_crates.iter().rev().for_each(|&c| stacks_part1[mov.to - 1].push(c));
            let new_source_stack_len = stacks_part1[mov.from - 1].len() - mov.n;
            stacks_part1[mov.from - 1].resize(new_source_stack_len, 'X');
        }
        let tops = stacks_part1.iter().filter_map(|stack| stack.last()).collect::<String>();
        println!("Top crates: {}", tops);
    }
    {
        let mut stacks_part2 = stacks.clone();
        for mov in &procedure {
            assert!(mov.to != mov.from);
            let from = &stacks_part2[mov.from - 1];
            let moved_crates = from[from.len() - mov.n..].to_owned();
            moved_crates.iter().for_each(|&c| stacks_part2[mov.to - 1].push(c));
            let new_source_stack_len = stacks_part2[mov.from - 1].len() - mov.n;
            stacks_part2[mov.from - 1].resize(new_source_stack_len, 'X');
        }
        let tops = stacks_part2.iter().filter_map(|stack| stack.last()).collect::<String>();
        println!("Top crates: {}", tops);
    }
}
