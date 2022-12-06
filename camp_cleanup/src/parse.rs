use chumsky::{
    primitive::{end, just},
    text::{int, newline, TextParser},
    Parser,
};

fn assignment() -> core::BParser<crate::Assignment> {
    int(10)
        .from_str::<u64>()
        .unwrapped()
        .then_ignore(just('-'))
        .then(int(10).from_str().unwrapped())
        .map(|(start, end)| (start..=end).collect())
        .boxed()
}

fn pair() -> core::BParser<crate::Pair> {
    assignment()
        .then_ignore(just(','))
        .then(assignment())
        .boxed()
}

fn parser() -> core::BParser<Box<[crate::Pair]>> {
    pair()
        .separated_by(newline())
        .map(Vec::into_boxed_slice)
        .padded()
        .then_ignore(end())
        .boxed()
}

pub fn parse(input: &str) -> Result<Box<[crate::Pair]>, Vec<core::Error>> {
    parser().parse(input)
}
