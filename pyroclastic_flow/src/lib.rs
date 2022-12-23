#![feature(array_windows)]

use std::array::IntoIter;

pub const INPUT: &str = include_str!("./input.txt");
pub const DAY: usize = 17;

mod parse;
pub use parse::parse;

#[derive(Debug)]
pub enum Direction {
    Right,
    Left,
}

type Directions = Box<[Direction]>;

pub fn part1(directions: &Directions) -> usize {
    simulate(directions, 2022).len()
}

pub fn part2(directions: &Directions) -> Option<u64> {
    let chamber = simulate(directions, 3000);

    let (pattern_start, pattern_length) = combinations(chamber.array_windows::<50>().enumerate())
        .find(|((_, a), (_, b))| a == b)
        .map(|((i, _), (j, _))| (i, j - i))?;

    let (rocks_before_pattern, rocks_generated_in_pattern) = (
        count_rocks_in_chamber(&chamber[..pattern_start]),
        count_rocks_in_chamber(&chamber[pattern_start..pattern_start + pattern_length]),
    );

    let (number_of_pattern_repetitions, leftover_rocks) = (
        (1_000_000_000_000 - rocks_before_pattern as u64) / rocks_generated_in_pattern as u64,
        (1_000_000_000_000 - rocks_before_pattern as u64) % rocks_generated_in_pattern as u64,
    );

    let leftover_rocks_height = (0..=pattern_length).find(|&i| {
        count_rocks_in_chamber(&chamber[pattern_start..pattern_start + i])
            >= leftover_rocks as usize
    })?;

    let height = number_of_pattern_repetitions * pattern_length as u64
        + pattern_start as u64
        + leftover_rocks_height as u64;

    Some(height)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    #[derive(Debug, Clone, Copy)]
    pub(crate) enum Rock {
        One { rows: [Row; 1] },
        Two { rows: [Row; 2] },
        Three { rows: [Row; 3] },
        Four { rows: [Row; 4] },
    }

    impl Rock {
        pub(crate) fn as_mut_slice(&mut self) -> &mut [Row] {
            match self {
                Rock::One { rows } => rows,
                Rock::Two { rows } => rows,
                Rock::Three { rows } => rows,
                Rock::Four { rows } => rows,
            }
        }
    }
}

fn rocks() -> Rocks {
    let rocks = [
        rocks::Rock::One {
            rows: rocks::HORIZONTAL,
        },
        rocks::Rock::Three { rows: rocks::CROSS },
        rocks::Rock::Three { rows: rocks::L },
        rocks::Rock::Four {
            rows: rocks::VERTICAL,
        },
        rocks::Rock::Two {
            rows: rocks::SQUARE,
        },
    ];

    Rocks {
        iterator: rocks.into_iter(),
        original: rocks.into_iter(),
    }
}

#[derive(Debug)]
struct Rocks {
    iterator: IntoIter<rocks::Rock, 5>,
    original: IntoIter<rocks::Rock, 5>,
}

impl Iterator for Rocks {
    type Item = rocks::Rock;

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

fn simulate(directions: &Directions, rocks_count: usize) -> Vec<Row> {
    let mut chamber = Vec::<Row>::new();
    let mut directions = directions.iter().cycle();

    for mut rock in rocks().take(rocks_count) {
        let rock = rock.as_mut_slice();
        let mut rock_top = chamber.len() + 3 + rock.len();

        loop {
            rock_top -= 1;
            if let Some(direction) = directions.next() {
                let air_push_successful = rock
                    .iter()
                    .enumerate()
                    .map(|(height, row)| (row, chamber.get(rock_top - height)))
                    .all(|(row, chamber_row)| {
                        if match direction {
                            Direction::Right => !matches!(row.last().unwrap(), Cell::Emty),
                            Direction::Left => !matches!(row.first().unwrap(), Cell::Emty),
                        } {
                            return false;
                        }

                        let Some(chamber_row) = chamber_row else {
                            return true;
                        };

                        match direction {
                            Direction::Right => row[0..6]
                                .iter()
                                .zip(chamber_row[1..7].iter())
                                .all(|(a, b)| matches!(a, Cell::Emty) || matches!(b, Cell::Emty)),
                            Direction::Left => row[1..7]
                                .iter()
                                .zip(chamber_row[0..6].iter())
                                .all(|(a, b)| matches!(a, Cell::Emty) || matches!(b, Cell::Emty)),
                        }
                    });
                if air_push_successful {
                    rock.iter_mut().for_each(|row| {
                        row.rotate_right(match direction {
                            Direction::Right => 1,
                            Direction::Left => 6,
                        })
                    })
                }

                if rock_top < rock.len() {
                    break;
                }

                let move_down_successful = rock.iter().enumerate().all(|(row_index, row)| {
                    if let Some(chamber_row) = chamber.get(rock_top - row_index - 1) {
                        row.iter()
                            .zip(chamber_row.iter())
                            .all(|(a, b)| matches!(a, Cell::Emty) || matches!(b, Cell::Emty))
                    } else {
                        true
                    }
                });
                if !move_down_successful {
                    break;
                }
            } else {
                break;
            }
        }

        chamber.resize(chamber.len().max(rock_top + 1), [Cell::Emty; 7]);

        for (row_index, row) in rock.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if !matches!(cell, Cell::Emty) {
                    chamber[rock_top - row_index][x] = cell;
                }
            }
        }
    }

    chamber
}

#[derive(Debug)]
struct Combinations<I: Iterator> {
    item: Option<I::Item>,

    iter1: I,
    iter2: I,
}

impl<I, A> Iterator for Combinations<I>
where
    I: Iterator<Item = A> + Clone,
    I::Item: Clone,
{
    type Item = (A, A);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(b) = self.iter2.next() {
            let a = self.item.clone().unwrap();

            Some((a, b))
        } else {
            self.item = self.iter1.next();

            self.item.clone().and_then(|a| {
                self.iter2 = self.iter1.clone();

                self.iter2.next().map(|b| (a, b))
            })
        }
    }
}

fn combinations<I>(mut iter: I) -> Combinations<I>
where
    I: Iterator + Clone,
{
    Combinations {
        item: iter.next(),

        iter1: iter.clone(),
        iter2: iter,
    }
}

fn count_rocks_in_chamber(chamber: &[Row]) -> usize {
    chamber
        .iter()
        .flatten()
        .filter(|&&cell| !matches!(cell, Cell::Emty))
        .count()
        * 5
        / 22
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
        assert_eq!(
            crate::part2(&crate::parse(INPUT).unwrap()),
            Some(1514285714288)
        );
    }
}
