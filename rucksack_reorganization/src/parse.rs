use chumsky::{
    primitive::{end, filter},
    text::{newline, TextParser},
    Parser,
};

fn rucksack() -> core::BParser<crate::Rucksack> {
    filter(char::is_ascii_alphabetic)
        .repeated()
        .at_least(1)
        .collect::<String>()
        .map(|line| {
            let line = line.as_bytes();
            let (first, second) = line.split_at(line.len() / 2);

            let first = crate::Compartment {
                items: first.iter().copied().collect(),
            };

            let second = crate::Compartment {
                items: second.iter().copied().collect(),
            };

            crate::Rucksack { first, second }
        })
        .boxed()
}

fn parser() -> core::BParser<Box<[crate::Rucksack]>> {
    rucksack()
        .separated_by(newline())
        .map(Vec::into_boxed_slice)
        .padded()
        .then_ignore(end())
        .boxed()
}

pub fn parse(input: &str) -> Result<Box<[crate::Rucksack]>, Vec<core::Error>> {
    parser().parse(input)
}
