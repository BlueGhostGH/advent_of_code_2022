use chumsky::{
    primitive::{choice, end, just},
    text::{newline, TextParser},
    Parser,
};

fn round() -> core::BParser<crate::Line> {
    choice((just('A'), just('B'), just('C')))
        .then_ignore(just(' '))
        .then(choice((just('X'), just('Y'), just('Z'))))
        .map(|(first, second)| crate::Line { first, second })
        .boxed()
}

fn parser() -> core::BParser<Box<[crate::Line]>> {
    round()
        .separated_by(newline())
        .map(Vec::into_boxed_slice)
        .padded()
        .then_ignore(end())
        .boxed()
}

pub fn parse(input: &str) -> Option<Box<[crate::Line]>> {
    parser().parse(input).ok()
}
