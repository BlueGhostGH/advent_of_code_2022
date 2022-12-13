fn is_valid_line(line: &str) -> bool {
    let check = |line: &str| {
        let mut tokens = line.split_ascii_whitespace();

        match tokens.next()? {
            "L" => {}
            "R" => {}
            "U" => {}
            "D" => {}
            _ => None?,
        };

        tokens.next()?.parse::<usize>().ok()?;

        Some(())
    };

    check(line).is_some()
}

pub fn parse(input: &str) -> Option<crate::Directions> {
    (input.is_ascii() && input.lines().all(is_valid_line)).then(|| crate::Directions {
        mv: None,
        count: 0,
        input,
    })
}
