use log::info;
use std::{collections::VecDeque, env, fs};

mod notes;

struct Monkey {
    items: VecDeque<usize>,
    inspection_count: usize,
}

struct Part1;
struct Part2 {
    gcd: usize,
}

trait PuzzlePart {
    fn title(&self) -> &str;
    fn number_of_iterations(&self) -> usize;
    fn managy_worries(&self, item: usize) -> usize;
}

impl PuzzlePart for Part1 {
    fn title(&self) -> &str {
        "--- Part One ---"
    }
    fn number_of_iterations(&self) -> usize {
        20
    }
    fn managy_worries(&self, item: usize) -> usize {
        item / 3
    }
}
impl PuzzlePart for Part2 {
    fn title(&self) -> &str {
        "--- Part Two ---"
    }
    fn number_of_iterations(&self) -> usize {
        10000
    }
    fn managy_worries(&self, item: usize) -> usize {
        item % self.gcd
    }
}

fn main() {
    // Run with RUST_LOG=INFO for logs
    env_logger::init();
    let input_file_path = env::args().nth(1).unwrap_or("11/test_data.txt".into());
    let input = fs::read_to_string(&input_file_path).unwrap();
    let notes = notes::parse_notes(&input).unwrap();
    let divisor_product: usize = notes.iter().map(|m| m.test.divisor).product(); // for part2
    let part1: Box<dyn PuzzlePart> = Box::new(Part1 {});
    let part2: Box<dyn PuzzlePart> = Box::new(Part2 {
        gcd: divisor_product,
    });
    for part in [part1, part2].iter() {
        println!("{}", part.title());
        println!();
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
        let mut items_to_catch: Vec<VecDeque<usize>> =
            vec![VecDeque::new(); monkeys.len()];
        for round in 0..part.number_of_iterations() {
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
                    let item = part.managy_worries(item);
                    info!("    Monkey gets bored with item. Worry level is divided by 3 to {item}.");
                    let target = if (item % note.test.divisor as usize) == 0 {
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
                    info!(
                        "    Item with worry level {item} is thrown to monkey {target}."
                    );
                    items_to_catch[target].push_back(item);
                }
                for (monkey, new_items) in
                    monkeys.iter_mut().zip(items_to_catch.iter_mut())
                {
                    while let Some(item) = new_items.pop_front() {
                        monkey.items.push_back(item);
                    }
                }
            }
            if round == 0 || round == 19 || ((round + 1) % 1000) == 0 {
                println!("== After round {} ==", round + 1);
                for (idx, monkey) in monkeys.iter().enumerate() {
                    println!(
                        "Monkey {idx} inspected items {} times",
                        monkey.inspection_count
                    );
                }
            }
        }
        let most_active: [usize; 2] = {
            let mut res: Vec<usize> = monkeys.iter().map(|m| m.inspection_count).collect();
            res.sort();
            [res[res.len() - 2], res[res.len() - 1]]
        };
        let monkey_business_lvl: usize = most_active.iter().product();
        println!("\nMonkey business level is at {monkey_business_lvl}\n");
    }
}
