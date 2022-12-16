use std::ops::{Add, AddAssign, Sub};

pub const INPUT: &str = include_str!("./input.txt");
pub const DAY: usize = 14;

mod parse;
pub use parse::parse;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: isize,
    y: isize,
}
type Size = Position;

#[derive(Debug, Clone)]
pub struct Reservoir {
    occupancy: Box<[bool]>,

    top_left: Position,
    bottom_right: Position,

    size: Size,
}

pub fn part1(reservoir: &Reservoir) -> u64 {
    let mut reservoir = reservoir.clone();

    reservoir
        .depth_first_search(Position { x: 500, y: 0 })
        .count
}

pub fn part2(reservoir: &Reservoir) -> u64 {
    let mut reservoir = reservoir.clone();
    reservoir.with_bottom();

    reservoir
        .depth_first_search(Position { x: 500, y: 0 })
        .count
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

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug)]
struct Search {
    found: bool,
    count: u64,
}

impl Reservoir {
    fn with_bottom(&mut self) {
        let y = (self.bottom_right.y - self.top_left.y) * self.size.x;

        for x in self.top_left.x..=self.bottom_right.x {
            let index = (x - self.top_left.x + y) as usize;

            self.occupancy[index] = true;
        }
    }

    fn position_as_index(&self, position: Position) -> usize {
        let with_offset = position - self.top_left;

        (with_offset.x + with_offset.y * self.size.x) as usize
    }

    fn in_bounds(&self, position: Position) -> bool {
        let in_bounds_x = self.top_left.x <= position.x && position.x <= self.bottom_right.x;
        let in_bounds_y = self.top_left.y <= position.y && position.y <= self.bottom_right.y;

        in_bounds_x && in_bounds_y
    }

    fn is_occupied(&self, position: Position) -> bool {
        self.in_bounds(position) && self.occupancy[self.position_as_index(position)]
    }

    fn depth_first_search(&mut self, position: Position) -> Search {
        if self.in_bounds(position) {
            let mut children_count = 0;

            for direction in [
                Position { x: 0, y: 1 },
                Position { x: -1, y: 1 },
                Position { x: 1, y: 1 },
            ] {
                let new_position = position + direction;

                if !self.is_occupied(new_position) {
                    let Search { found, count } = self.depth_first_search(new_position);
                    children_count += count;

                    if found {
                        return Search {
                            found: true,
                            count: children_count,
                        };
                    }
                }
            }

            self.occupancy[self.position_as_index(position)] = true;

            Search {
                found: false,
                count: children_count + 1,
            }
        } else {
            Search {
                found: true,
                count: 0,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn part1() {
        assert_eq!(crate::part1(&crate::parse(INPUT).unwrap()), 24);
    }

    #[test]
    fn part2() {
        assert_eq!(crate::part2(&crate::parse(INPUT).unwrap()), 93);
    }
}
