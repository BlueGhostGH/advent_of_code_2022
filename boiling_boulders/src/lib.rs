use std::{
    array::IntoIter,
    collections::VecDeque,
    ops::{Add, AddAssign, Sub, SubAssign},
};

pub const INPUT: &str = include_str!("./input.txt");
pub const DAY: usize = 18;

mod parse;
pub use parse::parse;

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, Clone, Copy)]
struct Bounds {
    origin: Position,
    dimensions: Position,
}

#[derive(Debug)]
pub struct Field {
    bounds: Bounds,
    occupied: Box<[bool]>,
    positions: Box<[Position]>,
}

pub fn part1(field: &Field) -> usize {
    field.free_face_count()
}

pub fn part2(field: &Field) -> usize {
    field.exterior_face_count()
}

#[derive(Debug)]
struct Neighbours {
    position: Position,
    neighbours: IntoIter<Position, 6>,
}

impl Iterator for Neighbours {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        self.neighbours
            .next()
            .map(|neighbour| neighbour + self.position)
    }
}

impl Position {
    fn neighbours(self) -> Neighbours {
        Neighbours {
            position: self,
            neighbours: [
                Position { x: -1, y: 0, z: 0 },
                Position { x: 1, y: 0, z: 0 },
                Position { x: 0, y: -1, z: 0 },
                Position { x: 0, y: 1, z: 0 },
                Position { x: 0, y: 0, z: -1 },
                Position { x: 0, y: 0, z: 1 },
            ]
            .into_iter(),
        }
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Self) -> Self::Output {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, other: Self) -> Self::Output {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Position {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Bounds {
    fn capacity(&self) -> usize {
        (self.dimensions.x * self.dimensions.y * self.dimensions.z) as _
    }

    fn grow(&mut self) {
        self.origin -= Position { x: 1, y: 1, z: 1 };
        self.dimensions += Position { x: 2, y: 2, z: 2 };
    }

    fn index(&self, position: Position) -> usize {
        let position = position - self.origin;

        (position.x
            + position.y * self.dimensions.x
            + position.z * self.dimensions.x * self.dimensions.y) as _
    }

    fn is_in_bounds(&self, position: Position) -> bool {
        let (x_in_bounds, y_in_bounds, z_in_bounds) = (
            self.origin.x <= position.x && position.x < self.origin.x + self.dimensions.x,
            self.origin.y <= position.y && position.y < self.origin.y + self.dimensions.y,
            self.origin.z <= position.z && position.z < self.origin.z + self.dimensions.z,
        );

        x_in_bounds && y_in_bounds && z_in_bounds
    }
}

impl Field {
    fn contains(&self, position: Position) -> bool {
        self.occupied[self.bounds.index(position)]
    }

    fn free_face_count(&self) -> usize {
        self.positions
            .iter()
            .map(|&position| {
                position
                    .neighbours()
                    .filter(|&neighbour| !self.contains(neighbour))
                    .count()
            })
            .sum()
    }

    fn enclosing_space(&self) -> Self {
        let mut occupied = vec![false; self.bounds.capacity()].into_boxed_slice();
        let mut positions = Vec::with_capacity(self.bounds.capacity());
        let mut queue = VecDeque::new();

        queue.push_back(self.bounds.origin);
        while let Some(position) = queue.pop_front() {
            position.neighbours().for_each(|position| {
                if self.bounds.is_in_bounds(position)
                    && !self.contains(position)
                    && !occupied[self.bounds.index(position)]
                {
                    occupied[self.bounds.index(position)] = true;
                    positions.push(position);

                    queue.push_back(position);
                }
            })
        }

        Field {
            bounds: self.bounds,
            occupied,
            positions: positions.into_boxed_slice(),
        }
    }

    fn exterior_face_count(&self) -> usize {
        let enclosing = self.enclosing_space();

        self.positions
            .iter()
            .map(|position| {
                position
                    .neighbours()
                    .filter(|&neighbour| enclosing.contains(neighbour))
                    .count()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn part1() {
        assert_eq!(crate::part1(&crate::parse(INPUT).unwrap()), 64);
    }

    #[test]
    fn part2() {
        assert_eq!(crate::part2(&crate::parse(INPUT).unwrap()), 58);
    }
}
