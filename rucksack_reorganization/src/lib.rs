#![feature(array_chunks)]

use std::collections::HashSet;

pub const INPUT: &str = include_str!("./input.txt");
pub const DAY: usize = 3;

mod parse;
pub use parse::parse;

type Item = u8;

#[derive(Debug)]
struct Compartment {
    items: HashSet<Item>,
}

#[derive(Debug)]
pub struct Rucksack {
    first: Compartment,
    second: Compartment,
}

pub fn part1(rucksacks: &[Rucksack]) -> u64 {
    rucksacks.iter().map(Rucksack::compute_common_sum).sum()
}

pub fn part2(rucksacks: &[Rucksack]) -> u64 {
    rucksacks
        .array_chunks::<3>()
        .map(|[first, second, third]| {
            [
                first.compartments_union(),
                second.compartments_union(),
                third.compartments_union(),
            ]
        })
        .map(|[first_elf, second_elf, third_elf]| {
            first_elf
                .intersection(&second_elf)
                .copied()
                .collect::<HashSet<_>>()
                .intersection(&third_elf)
                .copied()
                .collect::<HashSet<_>>()
                .into_iter()
                .map(item_priority)
                .sum::<u64>()
        })
        .sum()
}

impl From<&[Item]> for Compartment {
    fn from(items: &[Item]) -> Self {
        Compartment {
            items: items.iter().copied().collect(),
        }
    }
}

impl Rucksack {
    fn compute_common_sum(&self) -> u64 {
        let Rucksack { first, second } = self;

        first
            .items
            .intersection(&second.items)
            .copied()
            .map(item_priority)
            .sum()
    }

    fn compartments_union(&self) -> HashSet<Item> {
        let Rucksack { first, second } = self;

        first.items.union(&second.items).copied().collect()
    }
}

fn item_priority(item: Item) -> u64 {
    match item {
        b'a'..=b'z' => u64::from(item) - 96,
        b'A'..=b'Z' => u64::from(item) - 38,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1() {
        assert_eq!(crate::part1(&crate::parse(INPUT).unwrap()), 157);
    }

    #[test]
    fn part2() {
        assert_eq!(crate::part2(&crate::parse(INPUT).unwrap()), 70);
    }
}
