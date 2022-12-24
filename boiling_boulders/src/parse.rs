pub fn parse(input: &str) -> Option<crate::Field> {
    let positions = input
        .lines()
        .map(|line| {
            let mut coords = line.splitn(3, ',');

            let x = coords.next()?.parse().ok()?;
            let y = coords.next()?.parse().ok()?;
            let z = coords.next()?.parse().ok()?;

            Some(crate::Position { x, y, z })
        })
        .collect::<Option<Box<[_]>>>()?;

    let bounds = {
        let minimum = positions
            .iter()
            .copied()
            .reduce(|minimum, position| crate::Position {
                x: minimum.x.min(position.x),
                y: minimum.y.min(position.y),
                z: minimum.z.min(position.z),
            })?;
        let maximum = positions
            .iter()
            .copied()
            .reduce(|maximum, position| crate::Position {
                x: maximum.x.max(position.x),
                y: maximum.y.max(position.y),
                z: maximum.z.max(position.z),
            })?;

        let dimensions = maximum - minimum + crate::Position { x: 1, y: 1, z: 1 };

        let mut bounds = crate::Bounds {
            origin: minimum,
            dimensions,
        };
        bounds.grow();

        Some(bounds)
    }?;

    let occupied = {
        let mut occupied = vec![false; bounds.capacity()];
        for position in positions.iter().copied() {
            occupied[bounds.index(position)] = true;
        }

        occupied.into_boxed_slice()
    };

    Some(crate::Field {
        bounds,
        occupied,
        positions,
    })
}
