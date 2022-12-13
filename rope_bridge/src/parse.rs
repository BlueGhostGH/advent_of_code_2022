use std::iter;

pub fn parse(input: &str) -> Option<Box<[crate::Move]>> {
    if !input.is_ascii() {
        return None;
    }

    let mut moves = Vec::new();

    for line in input.lines() {
        let mut tokens = line.split_ascii_whitespace();

        let mv = match tokens.next()? {
            "L" => crate::Position { x: -1, y: 0 },
            "R" => crate::Position { x: 1, y: 0 },
            "U" => crate::Position { x: 0, y: -1 },
            "D" => crate::Position { x: 0, y: 1 },
            _ => return None,
        };

        let count = tokens.next()?.parse().ok()?;

        moves.extend(iter::repeat(mv).take(count));
    }

    let moves = moves.into_boxed_slice();

    Some(moves)
}
