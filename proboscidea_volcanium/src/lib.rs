use std::{cmp::Reverse, slice::Iter};

pub const INPUT: &str = include_str!("./input.txt");
pub const DAY: usize = 16;

mod parse;
pub use parse::parse;

type FlowRate = u8;
type PathLengths = Box<[Box<[u8]>]>;

#[derive(Debug)]
pub struct Input {
    starting_valve: usize,

    flow_rates: Box<[FlowRate]>,
    flow_rate_indices: Box<[usize]>,

    path_lengths: PathLengths,
}

pub fn part1(input: &Input) -> u16 {
    let mut best = 0;

    branch_and_bound(
        &input.flow_rates,
        &input.flow_rate_indices,
        &input.path_lengths,
        State::new(input.starting_valve as u8, 30),
        &mut [],
        &mut best,
        |bound, best| bound > best,
    );

    best
}

pub fn part2(_input: &Input) -> u64 {
    todo!()
}

#[derive(Debug, Clone, Copy)]
struct State {
    visited: u16,
    avoided: u16,

    pressure_released: u16,
    minutes_remaining: u8,

    position: u8,
}

#[derive(Debug)]
struct Branch<'st, 'frs> {
    state: &'st State,
    flow_rates: &'frs [FlowRate],
    path_lengths: Iter<'frs, u8>,

    destination: usize,
}

impl State {
    fn new(position: u8, minutes_remaining: u8) -> Self {
        State {
            visited: 0,
            avoided: 1 << position,

            pressure_released: 0,
            minutes_remaining,

            position,
        }
    }

    fn can_visit(&self, index: usize) -> bool {
        (self.visited | self.avoided) & (1 << index) == 0
    }

    fn bound(self, flow_rates: &[FlowRate], flow_rate_indices: &[usize]) -> u16 {
        self.pressure_released
            + (0..=self.minutes_remaining)
                .rev()
                .step_by(2)
                .skip(1)
                .zip(
                    flow_rate_indices
                        .iter()
                        .filter(|&&index| self.can_visit(index))
                        .map(|&i| flow_rates[i]),
                )
                .map(|(minutes, flow)| minutes as u16 * flow as u16)
                .sum::<u16>()
    }

    fn branch<'st, 'frs>(
        &'st self,
        flow_rates: &'frs [FlowRate],
        path_lengths: &'frs PathLengths,
    ) -> Branch<'st, 'frs> {
        Branch {
            state: &self,
            flow_rates,
            path_lengths: path_lengths[self.position as usize].iter(),

            destination: 0,
        }
    }
}

impl Iterator for Branch<'_, '_> {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(&distance) = self.path_lengths.next() {
            if self.state.can_visit(self.destination) {
                let minutes_remaining = match self.state.minutes_remaining.checked_sub(distance + 1)
                {
                    Some(minutes_remaining) => minutes_remaining,
                    None => {
                        self.destination += 1;
                        continue;
                    }
                };

                let flow_rate = self.flow_rates[self.destination];
                let pressure_to_be_released = minutes_remaining as u16 * flow_rate as u16;

                let state = State {
                    visited: self.state.visited | (1 << self.destination),
                    avoided: self.state.avoided,

                    pressure_released: self.state.pressure_released + pressure_to_be_released,
                    minutes_remaining,

                    position: self.destination as u8,
                };

                self.destination += 1;

                return Some(state);
            } else {
                self.destination += 1;
            }
        }

        None
    }
}

fn branch_and_bound<FB>(
    flow_rates: &[FlowRate],
    flow_rate_indices: &[usize],
    path_lengths: &PathLengths,
    state: State,

    best_for_visited: &mut [u16],
    best: &mut u16,

    filter_bound: FB,
) where
    FB: Fn(u16, u16) -> bool + Copy,
{
    if let Some(current_best) = best_for_visited.get_mut(state.visited as usize) {
        *current_best = state.pressure_released.max(*current_best);
    }

    *best = state.pressure_released.max(*best);

    let mut bound_branch_pairs = state
        .branch(flow_rates, path_lengths)
        .map(|state| (state.bound(flow_rates, flow_rate_indices), state))
        .filter(|&(bound, _)| filter_bound(bound, *best))
        .collect::<Box<[_]>>();
    bound_branch_pairs.sort_unstable_by_key(|(bound, _)| Reverse(*bound));

    // https://github.com/rust-lang/rust/issues/59878.
    for (bound, branch) in bound_branch_pairs.into_vec() {
        if filter_bound(bound, *best) {
            branch_and_bound(
                flow_rates,
                flow_rate_indices,
                path_lengths,
                branch,
                best_for_visited,
                best,
                filter_bound,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn part1() {
        assert_eq!(crate::part1(&crate::parse(INPUT).unwrap()), 1651);
    }

    #[test]
    fn part2() {
        assert_eq!(crate::part2(&crate::parse(INPUT).unwrap()), 2216);
    }
}
