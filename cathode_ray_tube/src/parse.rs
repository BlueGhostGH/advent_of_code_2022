use std::iter;

pub fn parse(input: &str) -> Option<Box<[crate::Cycle]>> {
    if !input.is_ascii() {
        return None;
    }

    let mut x = 1;
    let mut cycles = Vec::new();

    for line in input.lines() {
        let mut tokens = line.split_ascii_whitespace();

        match tokens.next()? {
            "noop" => cycles.extend(iter::repeat(x).take(1)),
            "addx" => {
                let new_cycles = iter::repeat(x).take(2);

                let argument = tokens.next()?.parse::<i64>().ok()?;
                x += argument;

                cycles.extend(new_cycles)
            }
            _ => return None,
        }
    }

    let cycles = cycles.into_boxed_slice();

    Some(cycles)
}
