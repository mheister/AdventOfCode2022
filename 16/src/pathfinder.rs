use anyhow::{anyhow, Context};

use crate::preprocessing::*;

/// A node in the solution search tree
#[derive(Clone)]
struct State {
    pos: ValveIdx,
    closed_valves: ValveBitMask,
    /// Amount of pressure that would be released if we did nothing more
    score: u32,
    time_left: u32,
}

enum StateCmp {
    StrictlyBetterOrEqual,
    StrictlyWorseOrEqual,
    Unknown,
}

impl State {
    fn compare_to(&self, other: &State) -> StateCmp {
        if self.pos != other.pos {
            return StateCmp::Unknown;
        }
        if self.score >= other.score
            && self.time_left >= other.time_left
            && self.closed_valves.is_superset(other.closed_valves)
        {
            return StateCmp::StrictlyBetterOrEqual;
        }
        if self.score <= other.score
            && self.time_left <= other.time_left
            && self.closed_valves.is_subset(other.closed_valves)
        {
            return StateCmp::StrictlyWorseOrEqual;
        }
        return StateCmp::Unknown;
    }
}

struct StateMemoizer {
    best_known: Vec<Vec<State>>,
}

enum StateMemoizationResult {
    SeenBetter,
    PotentiallyBest,
}

impl StateMemoizer {
    fn new(n: usize) -> Self {
        Self {
            best_known: vec![vec![]; n],
        }
    }

    fn memoize(&mut self, s: &State) -> StateMemoizationResult {
        let states_at_valve = &mut self.best_known[s.pos as usize];
        for known in states_at_valve.iter_mut() {
            match s.compare_to(known) {
                StateCmp::StrictlyBetterOrEqual => {
                    *known = s.clone();
                    return StateMemoizationResult::PotentiallyBest;
                }
                StateCmp::StrictlyWorseOrEqual => {
                    return StateMemoizationResult::SeenBetter;
                }
                StateCmp::Unknown => (),
            }
        }
        states_at_valve.push(s.clone());
        StateMemoizationResult::PotentiallyBest
    }

    fn get_best_score(&self) -> Option<u32> {
        self.best_known
            .iter()
            .flat_map(|v| v.iter())
            .map(|s| s.score)
            .max()
    }
}

pub fn find_pressure_release_potential(cave: Cave) -> anyhow::Result<u32> {
    let mut states: Vec<State> = vec![];
    let closed_valves = cave
        .valves
        .iter()
        .enumerate()
        // valves with zero potential flow rate might as well be considered open from the start
        .filter(|(_, v)| v.flow_rate > 0)
        .map(|(idx, _)| idx as ValveIdx)
        .collect();
    states.push(State {
        pos: cave
            .valve_labels
            .iter()
            .position(|&p| p == "AA".parse().unwrap())
            .ok_or(anyhow!("Could not find starting valve AA"))
            .and_then(|p| u8::try_from(p).context("Index of AA valve out of bounds"))?,
        closed_valves,
        score: 0,
        time_left: 30,
    });
    let mut mem = StateMemoizer::new(cave.valves.len());
    let mut state_cnt = 0usize;
    let mut prune_cnt = 0usize;
    while !states.is_empty() {
        state_cnt += 1;

        let s = states.pop().unwrap();

        // Check if leaf node
        if s.time_left == 0 || *s.closed_valves == 0 {
            continue;
        }

        // Open valve
        if s.closed_valves.contains(s.pos) {
            let mut closed_valves = s.closed_valves.clone();
            closed_valves.remove(s.pos);
            let s_prime = State {
                closed_valves,
                score: s.score + (s.time_left - 1) * cave[s.pos].flow_rate,
                time_left: s.time_left - 1,
                ..s
            };
            match mem.memoize(&s_prime) {
                StateMemoizationResult::SeenBetter => {
                    prune_cnt += 1;
                }
                StateMemoizationResult::PotentiallyBest => {
                    states.push(s_prime);
                }
            }
        }

        // Move on
        let valve = &cave[s.pos];
        for target in valve.tunnels.iter() {
            let s_prime = State {
                pos: target,
                time_left: s.time_left - 1,
                ..s.clone()
            };
            match mem.memoize(&s_prime) {
                StateMemoizationResult::SeenBetter => {
                    prune_cnt += 1;
                }
                StateMemoizationResult::PotentiallyBest => {
                    states.push(s_prime);
                }
            }
        }
    }
    println!("(INFO) During the solve, {prune_cnt} states were pruned and {state_cnt} states were visited");
    mem.get_best_score().ok_or(anyhow!(
        "Did not find anything! Are there any reachable valves with positive flow rate?"
    ))
}
