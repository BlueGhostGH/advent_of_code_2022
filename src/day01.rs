use std::num::ParseIntError;

advent_of_code::day!(01);

type ParseError = ParseIntError;
type Num = u32;

impl advent_of_code::Solution<'_> for Day01 {
    type Input = Vec<Num>;
    type ParseError = ParseError;

    type P1 = usize;
    type P2 = u32;

    fn parse(input: &str) -> Result<Self::Input, Self::ParseError> {
        input.lines().map(str::parse).collect()
    }

    fn part1(input: &[Num]) -> Self::P1 {
        input.iter().count()
    }

    fn part2(input: &[Num]) -> Self::P2 {
        input.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use advent_of_code::Solution;

    const INPUT: &str = "1
2
3";

    #[test]
    fn test() {
        assert_eq!(super::Day01::solve(INPUT), Ok((3, 6)));
    }
}
