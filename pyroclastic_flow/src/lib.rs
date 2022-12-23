use std::array::IntoIter;

pub const INPUT: &str = include_str!("./input.txt");
pub const DAY: usize = 17;

mod parse;
pub use parse::parse;

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

type Directions = Box<[Direction]>;

pub fn part1(directions: &Directions) -> u64 {
    todo!()
}

pub fn part2(directions: &Directions) -> u64 {
    todo!()
}

#[derive(Debug)]
enum Cell {
    Emty,
    Frst,
    Scnd,
    Thrd,
    Frth,
    Ffth,
}
type Row = [Cell; 7];

mod rocks {
    use crate::{Cell::*, Row};

    pub(crate) const HORIZONTAL: [Row; 1] = [[Emty, Emty, Frst, Frst, Frst, Frst, Emty]];
    pub(crate) const CROSS: [Row; 3] = [
        [Emty, Emty, Emty, Scnd, Emty, Emty, Emty],
        [Emty, Emty, Scnd, Scnd, Scnd, Emty, Emty],
        [Emty, Emty, Emty, Scnd, Emty, Emty, Emty],
    ];
    pub(crate) const L: [Row; 3] = [
        [Emty, Emty, Emty, Emty, Thrd, Emty, Emty],
        [Emty, Emty, Emty, Emty, Thrd, Emty, Emty],
        [Emty, Emty, Thrd, Thrd, Thrd, Emty, Emty],
    ];
    pub(crate) const VERTICAL: [Row; 4] = [
        [Emty, Emty, Frth, Emty, Emty, Emty, Emty],
        [Emty, Emty, Frth, Emty, Emty, Emty, Emty],
        [Emty, Emty, Frth, Emty, Emty, Emty, Emty],
        [Emty, Emty, Frth, Emty, Emty, Emty, Emty],
    ];
    pub(crate) const SQUARE: [Row; 2] = [
        [Emty, Emty, Ffth, Ffth, Emty, Emty, Emty],
        [Emty, Emty, Ffth, Ffth, Emty, Emty, Emty],
    ];
}

fn rocks() -> Rocks<'static> {
    let rocks = [
        &rocks::VERTICAL[..],
        &rocks::CROSS[..],
        &rocks::L[..],
        &rocks::VERTICAL[..],
        &rocks::SQUARE[..],
    ];

    Rocks {
        iterator: rocks.into_iter(),
        original: rocks.into_iter(),
    }
}

#[derive(Debug)]
struct Rocks<'rocks> {
    iterator: IntoIter<&'rocks [Row], 5>,
    original: IntoIter<&'rocks [Row], 5>,
}

impl<'rocks> Iterator for Rocks<'rocks> {
    type Item = &'rocks [Row];

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator.next() {
            Some(rock) => Some(rock),
            None => {
                self.iterator = self.original.clone();

                self.iterator.next()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn part1() {
        assert_eq!(crate::part1(&crate::parse(INPUT).unwrap()), 3068);
    }

    #[test]
    fn part2() {
        assert_eq!(crate::part2(&crate::parse(INPUT).unwrap()), 1514285714288);
    }
}
