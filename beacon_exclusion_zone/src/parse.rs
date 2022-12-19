fn parse_position(position: &str) -> Option<crate::Position> {
    let (x, y) = &position[position.find("x=")? + 2..].split_once(", y=")?;

    Some(crate::Position {
        x: x.parse().ok()?,
        y: y.parse().ok()?,
    })
}

pub fn parse(input: &str) -> Option<Box<[crate::Sensor]>> {
    input
        .lines()
        .map(|line| {
            let (sensor, beacon) = line.split_once(": ")?;

            let sensor_position = parse_position(sensor)?;
            let beacon_position = parse_position(beacon)?;

            Some(crate::Sensor {
                position: sensor_position,
                beacon: beacon_position,
                distance: beacon_position.manhattan_distance(&sensor_position),
            })
        })
        .collect()
}
