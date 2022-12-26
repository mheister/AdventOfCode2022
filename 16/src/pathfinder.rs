use std::collections::{HashMap, VecDeque};

use anyhow::{anyhow, Context};
use local_vec::LocalVec;

use crate::{input::ValveLabel, preprocessing::*};

/// A node in the solution search tree
#[derive(Clone, Debug)]
struct State {
    positions: LocalVec<ValveIdx, 2>,
    closed_valves: ValveBitMask,
    /// Amount of pressure that would be released if we did nothing more
    score: u32,
    time_left: u32,
}

#[derive(Debug)]
enum StateCmp {
    StrictlyBetter,
    StrictlyWorseOrEqual,
    Unknown,
}

impl State {
    /// Compare two states given that the actors are in the same positions
    ///
    /// Note: self's positions has to be a permutation of other's positions, this
    /// condition is not checked
    fn compare_to(&self, other: &State) -> StateCmp {
        if self.score <= other.score
            && self.time_left <= other.time_left
            && self.closed_valves.is_subset(other.closed_valves)
        {
            return StateCmp::StrictlyWorseOrEqual;
        }
        if self.score >= other.score
            && self.time_left >= other.time_left
            && self.closed_valves.is_superset(other.closed_valves)
        {
            // We know here that the states are not equally as good due to the previous
            // check
            return StateCmp::StrictlyBetter;
        }
        // Extra pruning: If we have more time left, but our score is still lower after an
        // ideal run of opening valves (assuming the biggest valves are still open), then
        // we are still worse or equal
        //
        // FIXME: I am not sure why we do not need to compare the closed_valves for this
        // condition; the program produces the right answer for both example and real
        // input, however, maybe this is just a very good heuristic
        if self.score < other.score {
            let score_upper_bound: u32 = self.score
                + (self.time_left / 2..=other.time_left / 2)
                    .zip(20..0)
                    .zip(19..0)
                    .map(|((t_left, v1), v2)| t_left * (v1 + v2))
                    .sum::<u32>();
            if score_upper_bound <= other.score {
                return StateCmp::StrictlyWorseOrEqual;
            }
        }
        return StateCmp::Unknown;
    }
}

struct StateMemoizer {
    /// Best known state for a combination (bit mask) of positions
    best_known: HashMap<ValveBitMask, Vec<State>>,
}

enum StateMemoizationResult {
    SeenBetter,
    PotentiallyBest,
}

impl StateMemoizer {
    fn new() -> Self {
        Self {
            best_known: HashMap::new(),
        }
    }

    fn memoize(&mut self, s: &State) -> StateMemoizationResult {
        let states_at_valve = self
            .best_known
            .entry(s.positions.iter().cloned().collect())
            .or_insert(vec![]);
        for known in states_at_valve.iter_mut() {
            match s.compare_to(known) {
                StateCmp::StrictlyBetter => {
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
            .flat_map(|(_, v)| v.iter())
            .map(|s| s.score)
            .max()
    }
}

pub fn find_pressure_release_potential(
    cave: Cave,
    starting_positions: Vec<ValveLabel>,
    time: u32,
) -> anyhow::Result<u32> {
    let mut states: VecDeque<State> = VecDeque::new();
    let closed_valves = cave
        .valves
        .iter()
        .enumerate()
        // valves with zero potential flow rate might as well be considered open from the start
        .filter(|(_, v)| v.flow_rate > 0)
        .map(|(idx, _)| idx as ValveIdx)
        .collect();
    let starting_positions = starting_positions
        .iter()
        .map(|label| {
            cave.valve_labels
                .iter()
                .position(|&p| p == *label)
                .ok_or(anyhow!("Could not find starting valve {label}"))
                .and_then(|p| {
                    ValveIdx::try_from(p)
                        .context(format!("Index of {label} valve out of bounds"))
                })
        })
        .fold(Ok(LocalVec::<_, 2>::new()), |acc, p| match p {
            Ok(p) => acc.map(|mut v| {
                v.push(p);
                v
            }),
            Err(e) => Err(e),
        })?;
    states.push_back(State {
        positions: starting_positions,
        closed_valves,
        score: 0,
        time_left: time,
    });
    let mut mem = StateMemoizer::new();
    let mut state_cnt = 0usize;
    let mut prune_cnt = 0usize;
    while !states.is_empty() {
        state_cnt += 1;
        let s = states.pop_front().unwrap();

        // Check if leaf node
        if s.time_left == 0 || *s.closed_valves == 0 {
            continue;
        }

        let mut follow_states_0 = LocalVec::<State, 100>::new();
        let mut follow_states_1 = LocalVec::<State, 100>::new();

        let follow_states = &mut follow_states_0;
        let follow_states_next = &mut follow_states_1;

        follow_states.push(State {
            time_left: s.time_left - 1,
            ..s.clone()
        });

        for (actor, pos) in s.positions.iter().enumerate() {
            while let Some(s) = follow_states.pop() {
                // Open valve
                if s.closed_valves.contains(*pos) {
                    let mut closed_valves = s.closed_valves.clone();
                    closed_valves.remove(*pos);
                    let s_prime = State {
                        closed_valves,
                        score: s.score + s.time_left * cave[*pos].flow_rate,
                        ..s.clone()
                    };
                    follow_states_next.push(s_prime);
                }
                // Move on
                let valve = &cave[*pos];
                for target in valve.tunnels.iter() {
                    let mut positions = s.positions.clone();
                    positions[actor] = target;
                    let s_prime = State {
                        positions,
                        ..s.clone()
                    };
                    follow_states_next.push(s_prime);
                }
            }
            std::mem::swap(follow_states, follow_states_next);
        }

        let mut prune = |state: &State| match mem.memoize(state) {
            StateMemoizationResult::SeenBetter => {
                prune_cnt += 1;
                true
            }
            StateMemoizationResult::PotentiallyBest => false,
        };

        // Prune
        follow_states_next
            .extend(follow_states.iter().filter(|state| !prune(state)).cloned());

        // Enqueue follow states, best scores first
        follow_states_next.sort_by(|a, b| b.score.cmp(&a.score));
        states.extend(follow_states_next.iter().cloned());
    }

    println!("(INFO) During the solve, {prune_cnt} states were pruned and {state_cnt} states were visited");
    mem.get_best_score().ok_or(anyhow!(
        "Did not find anything! Are there any reachable valves with positive flow rate?"
    ))
}
