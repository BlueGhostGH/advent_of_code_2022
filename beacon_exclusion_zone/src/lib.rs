#![feature(result_option_inspect)]

pub const INPUT: &str = include_str!("./input.txt");
pub const DAY: usize = 15;

mod parse;
pub use parse::parse;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone)]
pub struct Sensor {
    position: Position,
    beacon: Position,
    distance: u64,
}

pub fn part1(sensors: &[Sensor]) -> u64 {
    solve_part1(sensors, 2000000)
}

pub fn part2(sensors: &[Sensor]) -> Option<u64> {
    solve_part2(
        sensors,
        Position {
            x: 4000000,
            y: 4000000,
        },
    )
}

fn solve_part1(sensors: &[Sensor], y: i64) -> u64 {
    let mut ranges = sensors
        .iter()
        .filter_map(|sensor| sensor.range_at_line(y))
        .collect::<Box<[_]>>();
    ranges.sort_unstable_by_key(|range| range.start);
    let merged = merge_sorted_ranges(&ranges);

    let mut seen = Vec::with_capacity(sensors.len());
    merged
        .iter()
        .map(|range| {
            let beacon_count = count_beacons_in_range(sensors, range, y, &mut seen);

            range.length() - beacon_count
        })
        .sum()
}

fn solve_part2(sensors: &[Sensor], maximum_position: Position) -> Option<u64> {
    (0..=maximum_position.y)
        .find_map(|y| {
            let mut ranges = sensors
                .iter()
                .filter_map(|sensor| sensor.range_at_line(y))
                .collect::<Box<[_]>>();
            ranges.sort_unstable_by_key(|range| range.start);
            let merged = merge_sorted_ranges(&ranges);

            if let Some([first, _]) = merged.get(..2) {
                let x = first.end + 1;

                Some((x, y))
            } else {
                None
            }
        })
        .map(|(x, y)| (x as u64) * 4000000 + y as u64)
}

impl Position {
    fn manhattan_distance(&self, other: &Position) -> u64 {
        (self.x - other.x).unsigned_abs() + (self.y - other.y).unsigned_abs()
    }
}

#[derive(Debug, Clone, Copy)]
struct Range {
    start: i64,
    end: i64,
}

impl Sensor {
    fn range_at_line(&self, y: i64) -> Option<Range> {
        let half_size = self.distance as i64 - (y - self.position.y).abs();
        let range_size = half_size * 2 + 1;

        if range_size > 0 {
            Some(Range {
                start: self.position.x - half_size,
                end: self.position.x + half_size,
            })
        } else {
            None
        }
    }
}

impl Range {
    fn length(&self) -> u64 {
        (self.end - self.start).unsigned_abs() + 1
    }

    fn contains(&self, value: i64) -> bool {
        self.start <= value && value <= self.end
    }

    fn merge(&self, other: &Range) -> (Self, Option<Self>) {
        if self.contains(other.start) {
            (
                Range {
                    start: self.start,
                    end: self.end.max(other.end),
                },
                None,
            )
        } else if self.end + 1 == other.start {
            (
                Range {
                    start: self.start,
                    end: other.end,
                },
                None,
            )
        } else {
            (*self, Some(*other))
        }
    }
}

fn merge_sorted_ranges(ranges: &[Range]) -> Box<[Range]> {
    let mut merged_ranges = Vec::with_capacity(ranges.len());

    ranges
        .iter()
        .copied()
        .reduce(|range, other| {
            let last = merged_ranges.last().unwrap_or(&range);

            match last.merge(&other) {
                (range, Some(other)) => {
                    merged_ranges.push(range);

                    other
                }
                (range, _) => range,
            }
        })
        .inspect(|&last_range| merged_ranges.push(last_range));

    merged_ranges.into_boxed_slice()
}

fn count_beacons_in_range(
    sensors: &[Sensor],
    range: &Range,
    y: i64,
    seen: &mut Vec<Position>,
) -> u64 {
    seen.clear();

    for sensor in sensors {
        let at_right_level = sensor.beacon.y == y;
        let within_range = range.contains(sensor.beacon.x);
        let not_already_seen = !seen.contains(&sensor.beacon);

        if at_right_level && within_range && not_already_seen {
            seen.push(sensor.beacon);
        }
    }

    seen.len() as u64
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn part1() {
        assert_eq!(crate::solve_part1(&crate::parse(INPUT).unwrap(), 10), 26);
    }

    #[test]
    fn part2() {
        assert_eq!(
            crate::solve_part2(
                &crate::parse(INPUT).unwrap(),
                crate::Position { x: 20, y: 20 }
            ),
            Some(56000011)
        );
    }
}
