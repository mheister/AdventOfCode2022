use std::{env, fs, str::FromStr};

use anyhow::{anyhow, Context};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Value {
    List(Vec<Value>),
    Int(i32),
}

fn int_to_list(i: i32) -> Value {
    Value::List(vec![Value::Int(i)])
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Int(lhs), Value::Int(rhs)) => lhs.partial_cmp(rhs),
            (Value::List(lhs), Value::List(rhs)) => lhs.partial_cmp(rhs), // Vec's lexicographical ordering should work
            (lhs_list, Value::Int(rhs)) => lhs_list.partial_cmp(&int_to_list(*rhs)),
            (Value::Int(lhs), rhs_list) => int_to_list(*lhs).partial_cmp(rhs_list),
        }
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Value::Int(lhs), Value::Int(rhs)) => lhs.cmp(rhs),
            (Value::List(lhs), Value::List(rhs)) => lhs.cmp(rhs), // Vec's lexicographical ordering should work
            (lhs_list, Value::Int(rhs)) => lhs_list.cmp(&int_to_list(*rhs)),
            (Value::Int(lhs), rhs_list) => int_to_list(*lhs).cmp(rhs_list),
        }
    }
}

impl FromStr for Value {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn read_list_item(s: &str) -> anyhow::Result<(Value, &str)> {
            match s.chars().next() {
                Some('[') => {
                    let end = s
                        .chars()
                        .skip(1)
                        .scan(1, |depth, ch| {
                            match ch {
                                '[' => *depth += 1,
                                ']' => *depth -= 1,
                                _ => (),
                            };
                            Some(*depth)
                        })
                        .position(|depth| depth == 0)
                        .ok_or(anyhow!("no matching closing bracket for {s}"))?
                        + 1;
                    let rest_start = if let Some(',') = s.chars().nth(end + 1) {
                        end + 2
                    } else {
                        end + 1
                    };
                    Ok((Value::from_str(&s[0..=end])?, &s[rest_start..]))
                }
                Some(c) => {
                    if c == ']' {
                        return Err(anyhow!("no more items; looking at ']'"));
                    }
                    let (end, rest) = match s.chars().position(|ch| ch == ',' || ch == ']')
                    {
                        Some(pos) => (pos, &s[pos + 1..]),
                        None => (s.len(), ""),
                    };
                    Ok((Value::from_str(&s[0..end])?, rest))
                }
                None => Err(anyhow!("no more items")),
            }
        }

        match s.chars().next() {
            Some('[') => {
                let mut res = vec![];
                let mut items = &s[1..s.len() - 1];
                while !items.is_empty() {
                    let (item, rest) = read_list_item(items)?;
                    res.push(item);
                    items = rest;
                }
                Ok(Value::List(res))
            }
            Some(_) => Ok(Value::Int(
                i32::from_str(s).context("Error parsing int").unwrap(),
            )),
            None => Err(anyhow!("no more input")),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let input_file_path = env::args().nth(1).unwrap_or("13/test_data.txt".into());
    let input = fs::read_to_string(&input_file_path).unwrap();
    let pairs = input
        .split("\n\n")
        .map(|pair| -> anyhow::Result<_> {
            let pair = pair
                .trim()
                .split_once('\n')
                .ok_or(anyhow!("Error splitting pair"))?;
            Ok((Value::from_str(&pair.0)?, Value::from_str(&pair.1)?))
        })
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();
    let sum_of_1based_indices_right_order: usize = pairs
        .iter()
        .enumerate()
        .map(|(idx, (a, b))| if a < b { idx + 1 } else { 0 })
        .sum();
    println!("Part1 sum is {sum_of_1based_indices_right_order}");

    let mut items: Vec<_> = pairs.iter().cloned().fold(vec![], |mut acc, pair| {
        acc.push(pair.0);
        acc.push(pair.1);
        acc
    });
    let sep1: Value = "[[2]]".parse()?;
    let sep2: Value = "[[6]]".parse()?;
    items.push(sep1.clone());
    items.push(sep2.clone());
    items.sort();
    let code = (items
        .iter()
        .position(|v| *v == sep1)
        .ok_or(anyhow!("Lost separator ([[2]])"))?
        + 1)
        * (items
            .iter()
            .position(|v| *v == sep2)
            .ok_or(anyhow!("Lost separator ([[2]])"))?
            + 1);
    println!("Code value for Part2: {code}");
    Ok(())
}
