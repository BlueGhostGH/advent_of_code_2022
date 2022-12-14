fn parse_compartment(items: &[crate::Item]) -> crate::Compartment {
    let items = items
        .iter()
        .copied()
        .fold([false; 26 * 2], |mut items, item| {
            let index = match item {
                b'A'..=b'Z' => (item as usize) - 65,
                b'a'..=b'z' => (item as usize) - 71,
                _ => unreachable!(),
            };

            items[index] = true;
            items
        });

    crate::Compartment { items }
}

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

                        let (first, second) = (parse_compartment(first), parse_compartment(second));

                        crate::Rucksack { first, second }
                    })
                })
                .collect::<Option<Box<_>>>()
        })
        .flatten()
}
