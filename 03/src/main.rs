use std::{collections::HashSet, env, fs::File, io::BufRead, io::BufReader};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input_file_path = &args[1];
    let file = File::open(input_file_path)
        .expect(format!("Could not open file '{input_file_path}'").as_str());
    let priority_total: u32 = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|ln| find_duplicate_item_type(&ln))
        .map(|item_type| get_item_type_priority(item_type))
        .map(Result::unwrap)
        .sum();
    println!("Priority total: {priority_total}");
    let file = File::open(input_file_path)
        .expect(format!("Could not open file '{input_file_path}'").as_str());
    let badge_priority_total: u32 = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<String>>()
        .chunks(3)
        .map(|group| get_badge_item(&[&group[0], &group[1], &group[2]]))
        .map(|item_type| get_item_type_priority(item_type))
        .map(Result::unwrap)
        .sum();
    println!("Badge item priority total: {badge_priority_total}");
}

fn find_duplicate_item_type(rucksack: &str) -> char {
    let compartment_size = rucksack.chars().count() / 2;
    let compartment1_item_types: HashSet<char> =
        HashSet::from_iter(rucksack.chars().take(compartment_size));
    rucksack
        .chars()
        .skip(compartment_size)
        .find(|c| compartment1_item_types.contains(c))
        .unwrap()
}

fn get_item_type_priority(item_type: char) -> Result<u32, String> {
    if item_type >= 'a' && item_type <= 'z' {
        return Ok(item_type as u32 - 'a' as u32 + 1);
    }
    if item_type >= 'A' && item_type <= 'Z' {
        return Ok(item_type as u32 - 'A' as u32 + 27);
    }
    Err(format!(
        "Could not determine priority for item type {item_type}"
    ))
}

fn get_badge_item(group: &[&str; 3]) -> char {
    let rucksack_0_types = group[0].chars().collect::<HashSet<char>>();
    let rucksack_1_types = group[1].chars().collect::<HashSet<char>>();
    group[2]
        .chars()
        .find(|c| rucksack_0_types.contains(&c) && rucksack_1_types.contains(&c))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_duplicate_item_types() {
        let rucksacks = vec![
            ("vJrwpWtwJgWrhcsFMMfFFhFp", 'p'),
            ("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", 'L'),
            ("PmmdzqPrVvPwwTWBwg", 'P'),
            ("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", 'v'),
            ("ttgJtRGJQctTZtZT", 't'),
            ("CrZsJsPPZsGzwwsLwLmpwMDw", 's'),
        ];
        for (rucksack, dup_item) in rucksacks {
            assert_eq!(find_duplicate_item_type(rucksack), dup_item);
        }
    }

    #[test]
    fn get_item_type_priorities() {
        let types = vec![
            ('a', 1),
            ('e', 5),
            ('z', 26),
            ('A', 27),
            ('X', 50),
            ('Z', 52),
        ];
        for (item_type, priority) in types {
            assert_eq!(get_item_type_priority(item_type).unwrap(), priority);
        }
    }
}
