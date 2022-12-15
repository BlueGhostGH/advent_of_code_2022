#![feature(array_windows)]

use std::{array, ops::Add};

pub const INPUT: &str = include_str!("./input.txt");
pub const DAY: usize = 9;

mod parse;
pub use parse::parse;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    x: isize,
    y: isize,
}

type Move = Position;

pub fn part1(moves: &[Move]) -> u32 {
    solve::<2>(moves)
}

pub fn part2(moves: &[Move]) -> u32 {
    solve::<10>(moves)
}

impl Position {
    fn follow(self, head: Position) -> Self {
        let delta_x = head.x - self.x;
        let delta_y = head.y - self.y;

        if delta_x.abs() > 1 || delta_y.abs() > 1 {
            self + Position {
                x: delta_x.signum(),
                y: delta_y.signum(),
            }
        } else {
            self
        }
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[inline(always)]
fn set_bit(position: Position, slice: &mut [usize]) {
    let index = ((position.x + 512) + (position.y + 512) * 1024) as usize;

    let word = &mut slice[index >> 6];
    let shift = index & 0b111111;

    *word |= 1 << shift;
}

pub fn solve<const LENGTH: usize>(moves: &[Move]) -> u32 {
    let mut knots = [Position::default(); LENGTH];
    let indices = array::from_fn::<_, LENGTH, _>(|i| i);

    let mut visited = [0usize; 1024 * 1024 / 64];
    set_bit(Position { x: 0, y: 0 }, &mut visited);

    for &direction in moves {
        let head = &mut knots[0];
        *head = *head + direction;

        for &[first, second] in indices.array_windows::<2>() {
            let head = knots[first];
            let tail = &mut knots[second];

            let old_tail = *tail;
            *tail = tail.follow(head);

            if old_tail == *tail {
                break;
            }
        }

        let tail = knots[LENGTH - 1];
        set_bit(tail, &mut visited);
    }

    visited.iter().map(|u| u.count_ones()).sum::<u32>()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn part1() {
        assert_eq!(crate::part1(&crate::parse(INPUT).unwrap()), 13);
    }

    #[test]
    fn part2() {
        assert_eq!(crate::part2(&crate::parse(INPUT).unwrap()), 1);
    }
}
