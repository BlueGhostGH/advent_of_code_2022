pub fn parse(input: &str) -> Option<Box<[crate::Pair]>> {
    input
        .lines()
        .map(|line| {
            let (first, second) = line.split_once(',')?;

            let [first, second] = [first, second].map(|assignment| {
                let (start, end) = assignment.split_once('-')?;

                let (start, end) = (start.parse().ok()?, end.parse().ok()?);

                Some(crate::Assignment { start, end })
            });

            Some((first?, second?))
        })
        .collect::<Option<Vec<_>>>()
        .map(Vec::into_boxed_slice)
}
