pub fn parse(input: &str) -> Option<crate::Directions> {
    input
        .bytes()
        .map(|direction| match direction {
            b'<' => Some(crate::Direction::Left),
            b'>' => Some(crate::Direction::Right),
            _ => None,
        })
        .collect()
}
