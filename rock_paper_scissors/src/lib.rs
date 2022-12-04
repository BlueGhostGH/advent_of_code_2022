pub const INPUT: &str = include_str!("./input.txt");
pub const DAY: usize = 2;

mod parse;
pub use parse::parse;

#[derive(Debug)]
pub struct Line {
    pub(crate) first: char,
    pub(crate) second: char,
}

pub fn part1(rounds: &[Line]) -> u64 {
    rounds.iter().map(line_score::<Shape, Shape, Round>).sum()
}

pub fn part2(rounds: &[Line]) -> u64 {
    rounds.iter().map(line_score::<Shape, Outcome, Round>).sum()
}

fn line_score<Fst, Snd, Rnd>(Line { first, second }: &Line) -> u64
where
    Fst: From<char>,
    Snd: From<char>,
    Rnd: From<(Fst, Snd)> + Score,
{
    let first = (*first).into();
    let second = (*second).into();

    Rnd::from((first, second)).score()
}

trait Score {
    fn score(&self) -> u64;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Shape {
    fn wins_against(&self, other: &Shape) -> bool {
        self == &other.to_winner()
    }

    fn to_winner(self) -> Self {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }

    fn to_loser(self) -> Self {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

    fn for_outcome(self, outcome: &Outcome) -> Self {
        match outcome {
            Outcome::Win => self.to_winner(),
            Outcome::Loss => self.to_loser(),
            Outcome::Draw => self,
        }
    }
}

impl From<char> for Shape {
    fn from(ch: char) -> Self {
        match ch {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissors,
            _ => unreachable!(),
        }
    }
}

impl Score for Shape {
    fn score(&self) -> u64 {
        *self as u64
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl From<char> for Outcome {
    fn from(ch: char) -> Self {
        match ch {
            'X' => Outcome::Loss,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => unreachable!(),
        }
    }
}

impl Score for Outcome {
    fn score(&self) -> u64 {
        *self as u64
    }
}

#[derive(Debug)]
struct Round {
    player: Shape,
    enemy: Shape,
}

impl Round {
    fn outcome(&self) -> Outcome {
        if self.player.wins_against(&self.enemy) {
            Outcome::Win
        } else if self.enemy.wins_against(&self.player) {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }
}

impl From<(Shape, Shape)> for Round {
    fn from((enemy, player): (Shape, Shape)) -> Self {
        Self { player, enemy }
    }
}

impl From<(Shape, Outcome)> for Round {
    fn from((enemy, outcome): (Shape, Outcome)) -> Self {
        Self {
            player: enemy.for_outcome(&outcome),
            enemy,
        }
    }
}

impl Score for Round {
    fn score(&self) -> u64 {
        self.player.score() + self.outcome().score()
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part1() {
        assert_eq!(crate::part1(&crate::parse(INPUT).unwrap()), 15);
    }

    #[test]
    fn part2() {
        assert_eq!(crate::part2(&crate::parse(INPUT).unwrap()), 12);
    }
}
