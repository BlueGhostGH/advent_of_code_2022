temp::day!(03);

type Item = u8;

#[derive(Debug)]
pub struct Compartment {
    items: ::std::collections::HashSet<Item>,
}

#[derive(Debug)]
pub struct Backpack {
    first: Compartment,
    second: Compartment,
}

fn item_priority(item: u8) -> u32 {
    match item {
        b'a'..=b'z' => item as u32 - 96,
        b'A'..=b'Z' => item as u32 - 38,
        _ => unreachable!(),
    }
}

impl temp::Solution<'_> for Day03 {
    type Input = Vec<Backpack>;
    type ParseError = Error;

    type P1 = u32;
    type P2 = u32;

    fn parse(input: &str) -> ::std::result::Result<Self::Input, Self::ParseError> {
        if !input.is_ascii() {
            return Err(Error::InputNotAscii);
        }

        let backpacks = input
            .lines()
            .map(|backpack| {
                if backpack.len() % 2 != 0 {
                    return Err(Error::OddNumberOfItems);
                };

                let backpack = backpack.as_bytes();
                let (first, second) = backpack.split_at(backpack.len() / 2);

                let first = Compartment {
                    items: first.iter().copied().collect(),
                };
                let second = Compartment {
                    items: second.iter().copied().collect(),
                };

                Ok(Backpack { first, second })
            })
            .collect::<::std::result::Result<Vec<_>, _>>()?;

        if backpacks.len() % 3 != 0 {
            return Err(Error::BackpacksNumberNotMultipleOfThree);
        }

        Ok(backpacks)
    }

    fn part1(input: &[Backpack]) -> Self::P1 {
        input
            .iter()
            .map(|Backpack { first, second }| {
                first
                    .items
                    .intersection(&second.items)
                    .copied()
                    .map(item_priority)
                    .sum::<u32>()
            })
            .sum()
    }

    fn part2(input: &[Backpack]) -> Self::P2 {
        input
            .array_chunks::<3>()
            .map(|group| {
                let [first_elf, second_elf, third_elf] = group;

                let first_elf = first_elf
                    .first
                    .items
                    .union(&first_elf.second.items)
                    .copied()
                    .collect::<::std::collections::HashSet<_>>();
                let second_elf = second_elf
                    .first
                    .items
                    .union(&second_elf.second.items)
                    .copied()
                    .collect();
                let third_elf = third_elf
                    .first
                    .items
                    .union(&third_elf.second.items)
                    .copied()
                    .collect();

                first_elf
                    .intersection(&second_elf)
                    .copied()
                    .collect::<::std::collections::HashSet<_>>()
                    .intersection(&third_elf)
                    .copied()
                    .map(item_priority)
                    .sum::<u32>()
            })
            .sum()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InputNotAscii,
    OddNumberOfItems,
    BackpacksNumberNotMultipleOfThree,
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Error::InputNotAscii => f.write_str("input is not ascii"),
            Error::OddNumberOfItems => f.write_str("backpack has an odd number of items"),
            Error::BackpacksNumberNotMultipleOfThree => {
                f.write_str("the number of backpacks is not a multiple of three")
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
    use temp::Solution;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test() {
        assert_eq!(super::Day03::solve(INPUT), Ok((157, 70)));
    }
}
