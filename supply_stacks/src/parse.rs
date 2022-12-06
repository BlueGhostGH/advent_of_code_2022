use chumsky::{
    primitive::{filter, just},
    text::{int, keyword, newline},
    Parser,
};

fn container() -> core::BParser<Option<crate::Container>> {
    just('[')
        .ignore_then(filter(char::is_ascii_uppercase))
        .then_ignore(just(']'))
        .map(Some)
        .or(just("   ").to(None))
        .boxed()
}

fn stacks() -> core::BParser<Box<[crate::Stack]>> {
    container()
        .separated_by(just(' '))
        .separated_by(newline())
        .map(|drawing| {
            let stacks_count = drawing
                .iter()
                .map(|line| line.len())
                .max()
                .unwrap_or_default();
            let mut stacks = vec![Vec::new(); stacks_count].into_boxed_slice();

            drawing.iter().rev().for_each(|line| {
                line.iter().enumerate().for_each(|(column, container)| {
                    if let Some(container) = container {
                        stacks[column].push(*container);
                    }
                })
            });

            stacks
        })
        .boxed()
}

fn instruction() -> core::BParser<crate::Instruction> {
    let mv = keyword("move")
        .ignore_then(just(' '))
        .ignore_then(int(10).from_str::<usize>().unwrapped())
        .boxed();
    let from = keyword("from")
        .ignore_then(just(' '))
        .ignore_then(int(10).from_str::<usize>().unwrapped())
        .boxed();
    let to = keyword("to")
        .ignore_then(just(' '))
        .ignore_then(int(10).from_str::<usize>().unwrapped())
        .boxed();

    mv.then_ignore(just(' '))
        .then(from)
        .then_ignore(just(' '))
        .then(to)
        .map(|((count, source), destination)| crate::Instruction {
            count,
            from: source,
            to: destination,
        })
        .boxed()
}

fn instructions() -> core::BParser<Box<[crate::Instruction]>> {
    instruction()
        .separated_by(newline())
        .map(Vec::into_boxed_slice)
        .boxed()
}

pub fn parse(input: &str) -> Option<crate::Input> {
    let stack_numbers_position = input
        .lines()
        .enumerate()
        .find(|(_, line)| {
            line.chars()
                .nth(1)
                .as_ref()
                .map(char::is_ascii_digit)
                .unwrap_or_default()
        })
        .map(|(index, _)| index)?;
    let drawing_input = input
        .lines()
        .take(stack_numbers_position)
        .intersperse("\n")
        .collect::<String>();
    let instructions_input = input
        .lines()
        .skip(stack_numbers_position + 2)
        .intersperse("\n")
        .collect::<String>();

    let drawing = stacks().parse(&drawing_input[..]).ok()?;
    let instructions = instructions().parse(&instructions_input[..]).ok()?;

    Some(crate::Input {
        stacks: drawing,
        instructions,
    })
}
