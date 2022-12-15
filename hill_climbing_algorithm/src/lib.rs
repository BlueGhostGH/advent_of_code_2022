use std::collections::VecDeque;

pub const INPUT: &str = include_str!("./input.txt");
pub const DAY: usize = 12;

mod parse;
pub use parse::parse;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    x: isize,
    y: isize,
}

#[derive(Debug)]
pub struct Heightmap {
    width: usize,
    height: usize,

    start: Position,
    end: Position,

    heights: Vec<u8>,
}

pub fn part1(heightmap: &Heightmap) -> Option<u64> {
    heightmap.breadth_first_search(|position| position == heightmap.start)
}

pub fn part2(heightmap: &Heightmap) -> Option<u64> {
    heightmap.breadth_first_search(|position| {
        heightmap.heights[heightmap.position_as_index(position)] == 0
    })
}

impl Heightmap {
    fn position_as_index(&self, position: Position) -> usize {
        position.x as usize + position.y as usize * self.width
    }

    fn breadth_first_search<EndFn>(&self, is_end: EndFn) -> Option<u64>
    where
        EndFn: Fn(Position) -> bool,
    {
        let mut distances = vec![u64::MAX; self.heights.len()];
        distances[self.position_as_index(self.end)] = 0;

        let mut queue = VecDeque::with_capacity(self.heights.len());
        queue.push_back(self.end);

        while let Some(position) = queue.pop_front() {
            let index = self.position_as_index(position);
            let distance = distances[index];
            let height = self.heights[index];

            if is_end(position) {
                return Some(distance);
            }

            for neighbour in
                [(-1, 0), (1, 0), (0, -1), (0, 1)]
                    .into_iter()
                    .filter_map(|(delta_x, delta_y)| {
                        let x = position.x + delta_x;
                        let y = position.y + delta_y;

                        if 0 <= x && x < self.width as isize && 0 <= y && y < self.height as isize {
                            Some(Position { x, y })
                        } else {
                            None
                        }
                    })
            {
                let neighbour_index = self.position_as_index(neighbour);
                let neighbour_distance = &mut distances[neighbour_index];
                let neighbour_height = self.heights[neighbour_index];

                if height as i8 - 1 <= neighbour_height as i8 {
                    if distance + 1 < *neighbour_distance {
                        *neighbour_distance = distance + 1;

                        queue.push_back(neighbour);
                    }
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn part1() {
        assert_eq!(crate::part1(&crate::parse(INPUT).unwrap()), Some(31));
    }

    #[test]
    fn part2() {
        assert_eq!(crate::part2(&crate::parse(INPUT).unwrap()), Some(29));
    }
}
