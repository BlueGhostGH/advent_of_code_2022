pub fn parse(input: &str) -> Option<Box<[crate::Rucksack]>> {
    input
        .is_ascii()
        .then(|| {
            input
                .lines()
                .map(str::as_bytes)
                .map(|line| {
                    if line.len() % 2 == 0 {
                        let (first, second) = line.split_at(line.len() / 2);

                        let (first, second) = (
                            crate::Compartment::from(first),
                            crate::Compartment::from(second),
                        );

                        Some(crate::Rucksack { first, second })
                    } else {
                        None
                    }
                })
                .collect::<Option<Vec<_>>>()
                .map(Vec::into_boxed_slice)
        })
        .flatten()
}
