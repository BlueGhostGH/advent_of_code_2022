#![feature(iter_array_chunks, split_array)]

use std::{cmp::Reverse, slice::Iter};

pub const INPUT: &str = include_str!("./input.txt");
pub const DAY: usize = 11;

mod parse;
pub use parse::parse;

type Item = u64;
type Move = (Item, usize);
type Worry = Item;

#[derive(Debug, Clone, Copy)]
enum Value {
    Old,
    Lit { value: Worry },
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add { lhs: Value, rhs: Value },
    Mul { lhs: Value, rhs: Value },
}

#[derive(Debug, Clone, Copy)]
struct Test {
    divisor: u64,
    true_monkey: usize,
    false_monkey: usize,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    test: Test,

    inspection_count: u64,
}

#[derive(Debug, Clone)]
pub struct Game {
    monkeys: Vec<Monkey>,
    moves: Vec<Move>,

    max_worry: Worry,
}

pub fn part1(game: &Game) -> Option<u64> {
    solve::<20, true>(&mut game.clone())
}

pub fn part2(game: &Game) -> Option<u64> {
    solve::<10000, false>(&mut game.clone())
}

impl Value {
    fn get(&self, old: Worry) -> Worry {
        match self {
            Value::Old => old,
            &Value::Lit { value } => value,
        }
    }
}

impl Operation {
    fn apply(&self, old: Worry) -> Worry {
        match self {
            Operation::Add { lhs, rhs } => lhs.get(old) + rhs.get(old),
            Operation::Mul { lhs, rhs } => lhs.get(old) * rhs.get(old),
        }
    }
}

impl Test {
    fn check(&self, worry: Worry) -> usize {
        if worry % self.divisor == 0 {
            self.true_monkey
        } else {
            self.false_monkey
        }
    }
}

#[derive(Debug)]
struct Moves<'monkey> {
    inspection_count: &'monkey mut u64,

    calm: bool,

    items: Iter<'monkey, Item>,
    operation: &'monkey Operation,
    test: &'monkey Test,
}

impl Iterator for Moves<'_> {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        let worry = self.items.next()?;
        *self.inspection_count += 1;

        let worry = if self.calm {
            self.operation.apply(*worry) / 3
        } else {
            self.operation.apply(*worry)
        };

        let next_monkey = self.test.check(worry);

        Some((worry, next_monkey))
    }
}

impl Monkey {
    fn moves(&mut self, calm: bool) -> Moves {
        Moves {
            inspection_count: &mut self.inspection_count,

            calm,

            items: self.items.iter(),
            operation: &self.operation,
            test: &self.test,
        }
    }
}

impl Game {
    fn run_round(&mut self, calm: bool) {
        for index in 0..self.monkeys.len() {
            let source_monkey = &mut self.monkeys[index];

            self.moves.clear();
            for mv in source_monkey.moves(calm) {
                self.moves.push(mv);
            }
            source_monkey.items.clear();

            for &(worry, monkey_index) in &self.moves {
                self.monkeys[monkey_index]
                    .items
                    .push(worry % self.max_worry)
            }
        }
    }
}

fn solve<const ROUNDS_COUNT: usize, const CALM: bool>(game: &mut Game) -> Option<u64> {
    for _ in 0..ROUNDS_COUNT {
        game.run_round(CALM);
    }

    game.monkeys
        .sort_unstable_by_key(|monkey| Reverse(monkey.inspection_count));
    let ([first, second], _) = game.monkeys.get(..2)?.split_array_ref::<2>();

    Some(first.inspection_count * second.inspection_count)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn part1() {
        assert_eq!(crate::part1(&crate::parse(INPUT).unwrap()), Some(10605));
    }

    #[test]
    fn part2() {
        assert_eq!(
            crate::part2(&crate::parse(INPUT).unwrap()),
            Some(2713310158)
        );
    }
}
