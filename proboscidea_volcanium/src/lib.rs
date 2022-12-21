pub const INPUT: &str = include_str!("./input.txt");
pub const DAY: usize = 16;

mod parse;
pub use parse::parse;

type FlowRate = u8;
type PathLengths = Box<[Box<[usize]>]>;

#[derive(Debug)]
pub struct Input {
    starting_valve: usize,

    flow_rates: Box<[FlowRate]>,
    flow_rate_indices: Box<[usize]>,

    shortest_path_lengths: PathLengths,
}

pub fn part1(input: &Input) -> u64 {
    todo!()
}

pub fn part2(input: &Input) -> u64 {
    todo!()
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
