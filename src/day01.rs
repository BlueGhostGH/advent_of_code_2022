advent_of_code::day!(01);

type ParseError = ::std::num::ParseIntError;
type Calories = u32;
type Elf = Vec<Calories>;

impl advent_of_code::Solution<'_> for Day01 {
    type Input = Vec<Elf>;
    type ParseError = ParseError;

    type P1 = u32;
    type P2 = u32;

    fn parse(input: &str) -> ::std::result::Result<Self::Input, Self::ParseError> {
        input
            .lines()
            .collect::<Vec<_>>()
            .split(|line| line.is_empty())
            .map(|elf| elf.iter().copied().map(str::parse).collect())
            .collect()
    }

    fn part1(input: &[Elf]) -> Self::P1 {
        input
            .iter()
            .map(|elf| elf.iter().sum())
            .max()
            .unwrap_or_default()
    }

    fn part2(input: &[Elf]) -> Self::P2 {
        let mut input = input
            .iter()
            .map(|elf| elf.iter().sum::<u32>())
            .collect::<Vec<_>>();
        input.sort_by(|first, second| second.cmp(first));

        input[..3].iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use advent_of_code::Solution;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test() {
        assert_eq!(super::Day01::solve(INPUT), Ok((24000, 45000)));
    }
}
