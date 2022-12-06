use std::collections::HashSet;

pub const INPUT: &str = include_str!("./input.txt");
pub const DAY: usize = 4;

mod parse;
pub use parse::parse;

type Assignment = HashSet<u64>;
type Pair = (Assignment, Assignment);

pub fn part1(pairs: &[Pair]) -> usize {
    pairs
        .iter()
        .filter(|(first, second)| first.is_subset(second) || second.is_subset(first))
        .count()
}

pub fn part2(pairs: &[Pair]) -> usize {
    pairs
        .iter()
        .filter(|(first, second)| !first.is_disjoint(second))
        .count()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1() {
        assert_eq!(crate::part1(&crate::parse(INPUT).unwrap()), 2);
    }

    #[test]
    fn part2() {
        assert_eq!(crate::part2(&crate::parse(INPUT).unwrap()), 4);
    }
}
