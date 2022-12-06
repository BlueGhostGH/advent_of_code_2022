#![feature(array_chunks, iter_intersperse)]

use std::mem;

pub const INPUT: &str = include_str!("./input.txt");
pub const DAY: usize = 5;

mod parse;
pub use parse::parse;

type Container = char;
type Stack = Vec<Container>;

#[derive(Debug, Clone, Copy)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

#[derive(Debug, Clone)]
pub struct Input {
    stacks: Box<[Stack]>,
    instructions: Box<[Instruction]>,
}

pub fn part1(input: &Input) -> String {
    let mut input = input.clone();

    input.execute_moves::<Model9000>();

    input.collect_top_containers()
}

pub fn part2(input: &Input) -> String {
    let mut input = input.clone();

    input.execute_moves::<Model9001>();

    input.collect_top_containers()
}

impl Input {
    fn execute_moves<C>(&mut self)
    where
        C: Crane,
    {
        self.instructions.iter().for_each(|instruction| {
            let stacks = &mut self.stacks;

            let mut from = mem::take(&mut stacks[instruction.from - 1]);
            let mut to = mem::take(&mut stacks[instruction.to - 1]);

            C::move_containers(instruction.count, &mut from, &mut to);

            stacks[instruction.from - 1] = from;
            stacks[instruction.to - 1] = to;
        });
    }

    fn collect_top_containers(&self) -> String {
        self.stacks
            .iter()
            .filter_map(|stack| stack.last())
            .collect()
    }
}

trait Crane {
    fn move_containers(count: usize, from: &mut Stack, to: &mut Stack);
}

struct Model9000;

impl Crane for Model9000 {
    fn move_containers(count: usize, from: &mut Stack, to: &mut Stack) {
        to.extend((1..=count).filter_map(|_| from.pop()))
    }
}

struct Model9001;

impl Crane for Model9001 {
    fn move_containers(count: usize, from: &mut Stack, to: &mut Stack) {
        let containers = (1..=count).filter_map(|_| from.pop()).collect::<Vec<_>>();

        to.extend(containers.into_iter().rev());
    }
}

#[cfg(test)]
mod tests {
    #[rustfmt::skip]
    const INPUT: &str = 
"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1() {
        assert_eq!(
            crate::part1(&crate::parse(INPUT).unwrap()),
            String::from("CMZ")
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            crate::part2(&crate::parse(INPUT).unwrap()),
            String::from("MCD")
        );
    }
}
