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

fn parser() -> core::BParser<Box<[crate::Elf]>> {
    elf()
        .separated_by(newline())
        .map(Vec::into_boxed_slice)
        .padded()
        .then_ignore(end())
        .boxed()
}

pub fn parse(input: &str) -> Option<Box<[crate::Elf]>> {
    parser().parse(input).ok()
}
