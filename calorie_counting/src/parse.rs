pub fn parse(input: &str) -> Option<Box<[crate::Elf]>> {
    input
        .lines()
        .collect::<Vec<_>>()
        .split(|line| line.is_empty())
        .map(|elf| {
            let sum = elf
                .iter()
                .copied()
                .map(str::parse::<crate::Calories>)
                .map(Result::ok)
                .sum::<Option<_>>()?;

            Some(crate::Elf { sum })
        })
        .collect::<Option<Box<_>>>()
}
