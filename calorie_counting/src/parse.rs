pub fn parse(input: &str) -> Option<Box<[crate::Elf]>> {
    input
        .lines()
        .collect::<Vec<_>>()
        .split(|line| line.is_empty())
        .map(|elf| {
            elf.iter()
                .copied()
                .map(str::parse::<crate::Calories>)
                .map(Result::ok)
                .collect::<Option<Vec<_>>>()
                .map(Vec::into_boxed_slice)
        })
        .collect::<Option<Vec<_>>>()
        .map(Vec::into_boxed_slice)
}
