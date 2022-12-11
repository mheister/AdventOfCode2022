use log::info;
use std::{collections::VecDeque, env, fs};

mod notes;

struct Monkey {
    items: VecDeque<i32>,
    inspection_count: usize,
}

fn main() {
    // Run with RUST_LOG=INFO for logs
    env_logger::init();
    let input_file_path = env::args().nth(1).unwrap_or("11/test_data.txt".into());
    let input = fs::read_to_string(&input_file_path).unwrap();
    let notes = notes::parse_notes(&input).unwrap();
    let mut monkeys: Vec<_> = notes
        .iter()
        .enumerate()
        .map(|(idx, monkey_note)| {
            assert_eq!(idx, monkey_note.idx);
            Monkey {
                items: monkey_note.starting_items.clone().into(),
                inspection_count: 0,
            }
        })
        .collect();
    let mut items_to_catch: Vec<VecDeque<i32>> = vec![VecDeque::new(); monkeys.len()];
    for _ in 0..20 {
        for note in notes.iter() {
            info!("Monkey {}", note.idx);
            let monkey = &mut monkeys[note.idx];
            while let Some(item) = monkey.items.pop_front() {
                info!("  Monkey inspects an item with a worry level of {item}.");
                monkey.inspection_count += 1;
                use notes::Operand::*;
                let get_operand = |o: notes::Operand| match o {
                    Old => item.clone(),
                    Constant(x) => x,
                };
                let item = match &note.operation {
                    notes::Operation::Add(a, b) => {
                        let (a, b) = (get_operand(a.clone()), get_operand(b.clone()));
                        let new = a + b;
                        info!("    Worry level increases by {a} to {new}.");
                        new
                    }
                    notes::Operation::Multiply(a, b) => {
                        let (a, b) = (get_operand(a.clone()), get_operand(b.clone()));
                        let new = a * b;
                        info!("    Worry level is multiplied by {a} to {new}.");
                        new
                    }
                };
                let item = item / 3;
                info!("    Monkey gets bored with item. Worry level is divided by 3 to {item}.");
                let target = if (item % note.test.divisor as i32) == 0 {
                    info!(
                        "    Current worry level is divisible by {}.",
                        note.test.divisor
                    );
                    note.test.true_target
                } else {
                    info!(
                        "    Current worry level is not divisible by {}.",
                        note.test.divisor
                    );
                    note.test.false_target
                };
                info!("    Item with worry level {item} is thrown to monkey {target}.");
                items_to_catch[target].push_back(item);
            }
            for (monkey, new_items) in monkeys.iter_mut().zip(items_to_catch.iter_mut()) {
                while let Some(item) = new_items.pop_front() {
                    monkey.items.push_back(item);
                }
            }
        }
    }
    for (idx, monkey) in monkeys.iter().enumerate() {
        println!(
            "Monkey {idx} inspected items {} times",
            monkey.inspection_count
        );
    }
    let most_active: [usize; 2] = {
        let mut res: Vec<usize> = monkeys.iter().map(|m| m.inspection_count).collect();
        res.sort();
        [res[res.len() - 2], res[res.len() - 1]]
    };
    let monkey_business_lvl: usize = most_active.iter().product();
    println!("Monkey business level is at {monkey_business_lvl}");
}
