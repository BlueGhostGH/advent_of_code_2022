pub fn parse(input: &str) -> Option<crate::Heightmap> {
    if !input.is_ascii() {
        return None;
    }

    let width = input.lines().map(|line| line.len()).max()?;
    let height = input.lines().count();

    let mut start = None;
    let mut end = None;

    let mut heights = Vec::new();
    for (x, y, height) in input.lines().enumerate().flat_map(|(y, line)| {
        line.as_bytes()
            .iter()
            .enumerate()
            .map(move |(x, height)| (x as isize, y as isize, height))
    }) {
        heights.push(match height {
            b'S' => {
                start = Some(crate::Position { x, y });
                0
            }
            b'E' => {
                end = Some(crate::Position { x, y });
                b'z' - b'a'
            }
            height => height - b'a',
        })
    }

    Some(crate::Heightmap {
        width,
        height,

        start: start?,
        end: end?,

        heights,
    })
}
