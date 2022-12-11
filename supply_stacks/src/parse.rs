fn parse_stacks(input: &str) -> Option<(Box<[crate::Stack]>, &str)> {
    let (stacks, rest) = input.split_at(input.find("\n\n").or_else(|| input.find("\r\n\r\n"))?);

    let mut stacks = stacks.lines();
    let count = stacks.next_back()?.trim().split("   ").count();
    let max_height = stacks.clone().count();

    let input = stacks;
    let mut stacks = vec![Vec::with_capacity(max_height); count];

    for line in input.map(str::as_bytes) {
        for (position, container) in (0..=(line.len() - 2) / 4)
            .map(|position| line[position * 4 + 1])
            .enumerate()
        {
            match container {
                b'A'..=b'Z' => stacks[position].push(container),
                _ => {}
            }
        }
    }

    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    let stacks = stacks.into_boxed_slice();
    let rest = rest.trim();

    Some((stacks, rest))
}

fn parse_instructions(input: &str) -> Option<Box<[crate::Instruction]>> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut tokens = line.split_ascii_whitespace();

            let count = tokens.nth(1)?.parse().ok()?;
            let from = tokens.nth(1)?.parse().ok()?;
            let to = tokens.nth(1)?.parse().ok()?;

            Some(crate::Instruction { count, from, to })
        })
        .collect::<Option<Box<[_]>>>()
}

pub fn parse(input: &str) -> Option<crate::Input> {
    let (stacks, input) = parse_stacks(input)?;
    let instructions = parse_instructions(input)?;

    Some(crate::Input {
        stacks,
        instructions,
    })
}
