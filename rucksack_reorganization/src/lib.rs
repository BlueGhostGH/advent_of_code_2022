#![feature(array_chunks, array_zip, exclusive_range_pattern)]

pub const INPUT: &str = include_str!("./input.txt");
pub const DAY: usize = 3;

mod parse;
pub use parse::parse;

type Item = u8;

#[derive(Debug)]
struct Compartment {
    items: [bool; 26 * 2],
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
        .map(|[first, second, third]| [first, second, third].map(Rucksack::compartments_union))
        .map(|[first_elf, second_elf, third_elf]| {
            first_elf
                .intersection(&second_elf)
                .intersection(&third_elf)
                .items()
                .map(item_priority)
                .sum::<u64>()
        })
        .sum()
}

impl Compartment {
    fn items(&self) -> impl Iterator<Item = u8> {
        self.items
            .into_iter()
            .enumerate()
            .filter_map(|(index, present)| {
                if present {
                    let item = match index {
                        0..26 => (index as u8) + 65,
                        26..52 => (index as u8) + 71,
                        _ => unreachable!(),
                    };

                    Some(item)
                } else {
                    None
                }
            })
    }

    fn intersection(&self, other: &Compartment) -> Self {
        let items = self
            .items
            .zip(other.items)
            .map(|(in_self, in_other)| in_self && in_other);

        Compartment { items }
    }

    fn union(&self, other: &Compartment) -> Self {
        let items = self
            .items
            .zip(other.items)
            .map(|(in_self, in_other)| in_self || in_other);

        Compartment { items }
    }
}

impl Rucksack {
    fn compute_common_sum(&self) -> u64 {
        let Rucksack { first, second } = self;

        first.intersection(second).items().map(item_priority).sum()
    }

    fn compartments_union(&self) -> Compartment {
        let Rucksack { first, second } = self;

        first.union(second)
    }
}

fn item_priority(item: Item) -> u64 {
    match item {
        b'A'..=b'Z' => (item as u64) - 38,
        b'a'..=b'z' => (item as u64) - 96,
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
