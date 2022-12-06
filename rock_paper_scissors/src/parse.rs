pub fn parse(input: &str) -> Option<Box<[crate::Line]>> {
    input
        .lines()
        .map(|line| {
            if line.len() != 3 {
                None
            } else {
                let mut chars = line.chars();

                match (
                    chars.next().unwrap(),
                    chars.next().unwrap(),
                    chars.next().unwrap(),
                ) {
                    (first @ ('A' | 'B' | 'C'), ' ', second @ ('X' | 'Y' | 'Z')) => {
                        Some(crate::Line { first, second })
                    }
                    _ => None,
                }
            }
        })
        .collect::<Option<Vec<_>>>()
        .map(Vec::into_boxed_slice)
}
