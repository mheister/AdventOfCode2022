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

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input_file_path = &args[1];
    let file = File::open(input_file_path)
        .expect(format!("Could not open file '{input_file_path}'").as_str());
    let rounds: Vec<Round> = BufReader::new(file)
        .lines()
        .map(|ln| ln.unwrap())
        .map(|ln| -> Round {
            let mut sp = ln.split(" ");
            let opp = sp.next().unwrap();
            let my = sp.next().unwrap();
            assert!(sp.next().is_none());
            Round {
                their_hand: HAND_BY_CODE.get(opp).unwrap().clone(),
                my_hand: HAND_BY_CODE.get(my).unwrap().clone(),
            }
        })
        .collect();
    // println!("Games: {rounds:?}");

    let score : u32 = rounds.iter().cloned()
        .map(|round| shape_score(round.my_hand) + outcome_score(round))
        .sum();
    println!("Score: {score}");
}

fn shape_score(my_hand: Hand) -> u32 {
    match my_hand {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
    }
}

fn outcome_score(round: Round) -> u32 {
    use Hand::*;
    match round {
        Round {
            their_hand: Paper,
            my_hand: Scissors,
        } => 6,
        Round {
            their_hand: Rock,
            my_hand: Paper,
        } => 6,
        Round {
            their_hand: Scissors,
            my_hand: Rock,
        } => 6,
        Round {
            their_hand,
            my_hand,
        } => {
            if their_hand == my_hand {
                3
            } else {
                0
            }
        }
    }
}
