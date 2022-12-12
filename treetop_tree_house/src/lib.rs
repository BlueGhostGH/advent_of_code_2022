use std::cell::Cell;

pub const INPUT: &str = include_str!("./input.txt");
pub const DAY: usize = 8;

mod parse;
pub use parse::parse;

type Height = u8;
type Position = (isize, isize);
type Step = Position;

#[derive(Debug)]
pub struct Forest<'input> {
    width: usize,
    stride: usize,
    height: usize,
    trees: &'input [Height],
    seen: Box<[Cell<bool>]>,
}

pub fn part1(forest: &Forest) -> usize {
    let left = Lines::new((0, 0), (0, 1), forest.height, (1, 0), forest.width);
    let right = Lines::new(
        (forest.width as isize - 1, 0),
        (0, 1),
        forest.height,
        (-1, 0),
        forest.width,
    );
    let top = Lines::new((0, 0), (1, 0), forest.width, (0, 1), forest.height);
    let bottom = Lines::new(
        (0, forest.height as isize - 1),
        (1, 0),
        forest.width,
        (0, -1),
        forest.height,
    );

    left.count_visible_trees(forest)
        + right.count_visible_trees(forest)
        + top.count_visible_trees(forest)
        + bottom.count_visible_trees(forest)
}

pub fn part2(forest: &Forest) -> Option<usize> {
    Lines::new((0, 0), (1, 0), forest.width, (0, 1), forest.height)
        .flatten()
        .map(|position| {
            let left = Line::new(position, (-1, 0), position.0 as usize + 1);
            let right = Line::new(position, (1, 0), forest.width - position.0 as usize);
            let top = Line::new(position, (0, -1), position.1 as usize + 1);
            let down = Line::new(position, (0, 1), forest.height - position.1 as usize);

            let tree_height = forest.tree_height(position);

            let left = left
                .skip_first()
                .visible_distance_from_height(forest, tree_height);
            let right = right
                .skip_first()
                .visible_distance_from_height(forest, tree_height);
            let top = top
                .skip_first()
                .visible_distance_from_height(forest, tree_height);
            let down = down
                .skip_first()
                .visible_distance_from_height(forest, tree_height);

            left * right * top * down
        })
        .max()
}

impl<'input> Forest<'input> {
    fn position_as_index(&self, position: Position) -> usize {
        position.0 as usize + position.1 as usize * self.stride as usize
    }

    fn tree_height(&self, position: Position) -> Height {
        self.trees[self.position_as_index(position)] - b'0'
    }
}

#[derive(Debug)]
struct Line {
    position: Position,
    step: Step,
    length: usize,
}

impl Line {
    fn new(position: Position, step: Step, length: usize) -> Self {
        Line {
            position,
            step,
            length,
        }
    }

    fn count_visible_trees(self, forest: &Forest) -> usize {
        let mut count = 0;
        let mut tallest = -1;

        for position in self {
            let tree_height = forest.tree_height(position) as i8;
            let index = forest.position_as_index(position);

            if tree_height > tallest {
                tallest = tree_height;

                if !forest.seen[index].get() {
                    forest.seen[index].set(true);
                    count += 1;
                }
            }

            if tallest >= 9 {
                break;
            }
        }

        count
    }

    fn skip_first(mut self) -> Self {
        self.next();

        self
    }

    fn visible_distance_from_height(self, forest: &Forest, height: Height) -> usize {
        let mut distance = 0;

        for position in self {
            distance += 1;
            if forest.tree_height(position) >= height {
                break;
            }
        }

        distance
    }
}

impl Iterator for Line {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.length > 0 {
            let position = self.position;

            self.position.0 += self.step.0;
            self.position.1 += self.step.1;
            self.length -= 1;

            Some(position)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Lines {
    position: Position,
    step: Step,
    length: usize,

    line_step: Step,
    line_length: usize,
}

impl Lines {
    fn new(
        position: Position,
        step: Step,
        length: usize,
        line_step: Step,
        line_length: usize,
    ) -> Self {
        Lines {
            position,
            step,
            length,

            line_step,
            line_length,
        }
    }

    fn count_visible_trees(self, forest: &Forest) -> usize {
        self.map(|line| line.count_visible_trees(forest)).sum()
    }
}

impl Iterator for Lines {
    type Item = Line;

    fn next(&mut self) -> Option<Self::Item> {
        if self.length > 0 {
            let line = Line::new(self.position, self.line_step, self.line_length);

            self.position.0 += self.step.0;
            self.position.1 += self.step.1;
            self.length -= 1;

            Some(line)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part1() {
        assert_eq!(crate::part1(&crate::parse(INPUT).unwrap()), 21);
    }

    #[test]
    fn part2() {
        assert_eq!(crate::part2(&crate::parse(INPUT).unwrap()), Some(8));
    }
}
