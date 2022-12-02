advent_of_code::day!(02);

#[derive(Debug)]
pub enum Column {
    X,
    Y,
    Z,
}

pub(crate) mod round {

    #[derive(Debug, Clone, Copy)]
    pub enum Play {
        Rock,
        Paper,
        Scissors,
    }

    impl Play {
        pub(super) fn score(&self) -> u32 {
            match self {
                Play::Rock => 1,
                Play::Paper => 2,
                Play::Scissors => 3,
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Result {
        Win,
        Draw,
        Loss,
    }

    impl Result {
        pub(super) fn score(&self) -> u32 {
            match self {
                Result::Win => 6,
                Result::Draw => 3,
                Result::Loss => 0,
            }
        }

        pub(super) fn compute(own: Play, opponent: Play) -> Self {
            match (own, opponent) {
                (Play::Rock, Play::Scissors)
                | (Play::Paper, Play::Rock)
                | (Play::Scissors, Play::Paper) => Result::Win,
                (Play::Rock, Play::Rock)
                | (Play::Paper, Play::Paper)
                | (Play::Scissors, Play::Scissors) => Result::Draw,
                (Play::Rock, Play::Paper)
                | (Play::Paper, Play::Scissors)
                | (Play::Scissors, Play::Rock) => Result::Loss,
            }
        }
    }

    pub(super) fn play_such_that(opponent: Play, result: Result) -> Play {
        match (opponent, result) {
            (Play::Scissors, Result::Win)
            | (Play::Rock, Result::Draw)
            | (Play::Paper, Result::Loss) => Play::Rock,
            (Play::Rock, Result::Win)
            | (Play::Paper, Result::Draw)
            | (Play::Scissors, Result::Loss) => Play::Paper,
            (Play::Paper, Result::Win)
            | (Play::Scissors, Result::Draw)
            | (Play::Rock, Result::Loss) => Play::Scissors,
        }
    }
}

impl advent_of_code::Solution<'_> for Day02 {
    type Input = Vec<(round::Play, Column)>;
    type ParseError = Error;

    type P1 = u32;
    type P2 = u32;

    fn parse(input: &str) -> ::std::result::Result<Self::Input, Self::ParseError> {
        if !input.is_ascii() {
            return Err(Error::InputNotAscii);
        }

        input
            .lines()
            .map(|round| {
                let (opponent_play, column) =
                    round.split_once(' ').ok_or(Error::MissingSpaceSeparator)?;

                let opponent_play = match opponent_play.len() {
                    1 => opponent_play.chars().next().unwrap(),
                    length => return Err(Error::ColumnNotOneCharacter { instead: length }),
                };
                let opponent_play = match opponent_play {
                    'A' => round::Play::Rock,
                    'B' => round::Play::Paper,
                    'C' => round::Play::Scissors,
                    character => return Err(Error::UnexpectedCharacter { character }),
                };

                let column = match column.len() {
                    1 => column.chars().next().unwrap(),
                    length => return Err(Error::ColumnNotOneCharacter { instead: length }),
                };
                let column = match column {
                    'X' => Column::X,
                    'Y' => Column::Y,
                    'Z' => Column::Z,
                    character => return Err(Error::UnexpectedCharacter { character }),
                };

                Ok((opponent_play, column))
            })
            .collect()
    }

    fn part1(input: &[(round::Play, Column)]) -> Self::P1 {
        input
            .iter()
            .map(|(opponent, column)| {
                let own = match column {
                    Column::X => round::Play::Rock,
                    Column::Y => round::Play::Paper,
                    Column::Z => round::Play::Scissors,
                };

                let result = round::Result::compute(own, *opponent);

                own.score() + result.score()
            })
            .sum()
    }

    fn part2(input: &[(round::Play, Column)]) -> Self::P2 {
        input
            .iter()
            .map(|(opponent, column)| {
                let result = match column {
                    Column::X => round::Result::Loss,
                    Column::Y => round::Result::Draw,
                    Column::Z => round::Result::Win,
                };

                let own = round::play_such_that(*opponent, result);

                own.score() + result.score()
            })
            .sum()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InputNotAscii,
    MissingSpaceSeparator,
    ColumnNotOneCharacter { instead: usize },
    UnexpectedCharacter { character: char },
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Error::InputNotAscii => f.write_str("input is not ascii"),
            Error::MissingSpaceSeparator => f.write_str("missing space separator between columns"),
            Error::ColumnNotOneCharacter { instead } => f.write_fmt(format_args!(
                "column should be one character but is instead {instead} characters"
            )),
            Error::UnexpectedCharacter { character } => {
                f.write_fmt(format_args!("unexpected character {character} in input"))
            }
        }
    }
}

impl ::std::error::Error for Error {
    fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)> {
        None
    }
}

#[cfg(test)]
mod tests {
    use advent_of_code::Solution;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test() {
        assert_eq!(super::Day02::solve(INPUT), Ok((15, 12)));
    }
}
