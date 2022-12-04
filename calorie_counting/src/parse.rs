use chumsky::{
    primitive::end,
    text::{int, newline, TextParser},
    Parser,
};

fn calories() -> core::BParser<crate::Calories> {
    int(10).from_str().unwrapped().boxed()
}

fn elf() -> core::BParser<crate::Elf> {
    calories()
        .separated_by(newline())
        .map(Vec::into_boxed_slice)
        .boxed()
}

fn parser() -> core::BParser<Vec<crate::Elf>> {
    elf()
        .separated_by(newline())
        .padded()
        .then_ignore(end())
        .boxed()
}

pub fn parse(input: &str) -> Result<Vec<crate::Elf>, Vec<core::Error>> {
    parser().parse(input)
}
