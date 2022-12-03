use phf::phf_map;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Round {
    their_hand: Hand,
    my_hand: Hand,
}

static HAND_BY_CODE: phf::Map<&'static str, Hand> = phf_map! {
    "A" => Hand::Rock, "B" => Hand::Paper, "C" => Hand::Scissors,
    "X" => Hand::Rock, "Y" => Hand::Paper, "Z" => Hand::Scissors,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

static OUTCOME_BY_CODE: phf::Map<&'static str, Outcome> = phf_map! {
    "X" => Outcome::Loss, "Y" => Outcome::Draw, "Z" => Outcome::Win,
};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input_file_path = &args[1];
    let file = File::open(input_file_path)
        .expect(format!("Could not open file '{input_file_path}'").as_str());
    let rounds_part1: Vec<Round> = BufReader::new(file)
        .lines()
        .map(|ln| ln.unwrap())
        .map(|ln| -> Round {
            let mut sp = ln.split(" ");
            let their_hand_code = sp.next().unwrap();
            let my = sp.next().unwrap();
            assert!(sp.next().is_none());
            Round {
                their_hand: HAND_BY_CODE.get(their_hand_code).unwrap().clone(),
                my_hand: HAND_BY_CODE.get(my).unwrap().clone(),
            }
        })
        .collect();

    let score: u32 = rounds_part1
        .iter()
        .cloned()
        .map(|round| shape_score(round.my_hand) + outcome_score(round))
        .sum();
    println!("Score (Part 1): {score}");

    let file = File::open(input_file_path)
        .expect(format!("Could not open file '{input_file_path}'").as_str());
    let rounds_part2: Vec<Round> = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|ln| -> Round {
            let mut sp = ln.split(" ");
            let their_hand = HAND_BY_CODE.get(sp.next().unwrap()).unwrap();
            let outcome = OUTCOME_BY_CODE.get(sp.next().unwrap()).unwrap();
            Round {
                their_hand: their_hand.clone(),
                my_hand: hand_for_outcome(*their_hand, *outcome),
            }
        })
        .collect();

    let score: u32 = rounds_part2
        .iter()
        .cloned()
        .map(|round| shape_score(round.my_hand) + outcome_score(round))
        .sum();
    println!("Score (Part 2): {score}");
}

fn shape_score(my_hand: Hand) -> u32 {
    match my_hand {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
    }
}

fn winning_hand_against(hand: Hand) -> Hand {
    use Hand::*;
    match hand {
        Rock => Paper,
        Paper => Scissors,
        Scissors => Rock,
    }
}

fn losing_hand_against(hand: Hand) -> Hand {
    use Hand::*;
    match hand {
        Rock => Scissors,
        Paper => Rock,
        Scissors => Paper,
    }
}

fn outcome_score(round: Round) -> u32 {
    if round.my_hand == winning_hand_against(round.their_hand) {
        return 6;
    };
    if round.my_hand == round.their_hand {
        return 3;
    };
    return 0;
}

fn hand_for_outcome(their_hand: Hand, outcome: Outcome) -> Hand {
    match outcome {
        Outcome::Loss => losing_hand_against(their_hand),
        Outcome::Draw => their_hand,
        Outcome::Win => winning_hand_against(their_hand),
    }
}
