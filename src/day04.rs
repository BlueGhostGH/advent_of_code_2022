advent_of_code::day!(04);

type Assignment = ::std::collections::HashSet<u64>;
type Pair = (Assignment, Assignment);

impl advent_of_code::Solution<'_> for Day04 {
    type Input = Vec<Pair>;
    type ParseError = Error;

    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> ::std::result::Result<Self::Input, Self::ParseError> {
        input
            .lines()
            .map(|pair| {
                let (first, second) = pair.split_once(',').ok_or(Error::MissingCommaSeparator)?;

                let (first_start, first_end) =
                    first.split_once('-').ok_or(Error::MissingDashSeparator)?;
                let (second_start, second_end) =
                    second.split_once('-').ok_or(Error::MissingDashSeparator)?;

                let first = first_start.parse::<u64>()?..=first_end.parse()?;
                let second = second_start.parse::<u64>()?..=second_end.parse()?;

                Ok((first.collect(), second.collect()))
            })
            .collect()
    }

    fn part1(input: &[Pair]) -> Self::P1 {
        input
            .iter()
            .filter(|(first, second)| first.is_subset(second) || second.is_subset(first))
            .count()
    }

    fn part2(input: &[Pair]) -> Self::P2 {
        input
            .iter()
            .filter(|(first, second)| !first.is_disjoint(second))
            .count()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    MissingCommaSeparator,
    MissingDashSeparator,
    ParseInt(::std::num::ParseIntError),
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Error::MissingCommaSeparator => f.write_str("line is missing comma separator"),
            Error::MissingDashSeparator => f.write_str("assignment is missing dash separator"),
            Error::ParseInt(parse_int_err) => f.write_fmt(format_args!("{parse_int_err}")),
        }
    }
}

impl ::std::error::Error for Error {
    fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Error::ParseInt(parse_int_err) => Some(parse_int_err),
            _ => None,
        }
    }
}

impl From<::std::num::ParseIntError> for Error {
    fn from(parse_int_err: ::std::num::ParseIntError) -> Self {
        Error::ParseInt(parse_int_err)
    }
}

#[cfg(test)]
mod tests {
    use advent_of_code::Solution;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test() {
        assert_eq!(super::Day04::solve(INPUT), Ok((2, 4)));
    }
}
