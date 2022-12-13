pub fn parse(input: &str) -> Option<Box<[crate::Rucksack]>> {
    input
        .is_ascii()
        .then(|| {
            input
                .lines()
                .map(str::as_bytes)
                .map(Some)
                .map(|line| line.filter(|line| line.len() % 2 == 0))
                .map(|line| line.filter(|line| line.iter().all(u8::is_ascii_alphabetic)))
                .map(|line| {
                    line.map(|line| {
                        let (first, second) = line.split_at(line.len() / 2);

                        let (first, second) = (
                            crate::Compartment::from(first),
                            crate::Compartment::from(second),
                        );

                        crate::Rucksack { first, second }
                    })
                })
                .collect::<Option<Box<_>>>()
        })
        .flatten()
}
