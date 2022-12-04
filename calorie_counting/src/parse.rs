use chumsky::{text::TextParser, Parser};

fn calories() -> ::core::BParser<crate::Calories> {
    ::chumsky::text::int(10).from_str().unwrapped().boxed()
}

fn elf() -> ::core::BParser<crate::Elf> {
    calories()
        .separated_by(::chumsky::text::newline())
        .map(Vec::into_boxed_slice)
        .boxed()
}

fn parser() -> ::core::BParser<Vec<crate::Elf>> {
    elf()
        .separated_by(::chumsky::text::newline())
        .padded()
        .then_ignore(chumsky::primitive::end())
        .boxed()
}

pub fn parse(input: &str) -> ::std::result::Result<Vec<crate::Elf>, Vec<core::Error>> {
    parser().parse(input)
}
