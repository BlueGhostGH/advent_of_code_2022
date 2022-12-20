#![feature(array_windows, result_option_inspect)]

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
    solution::part1(sensors, 2000000)
}

pub fn part2(sensors: &[Sensor]) -> Option<u64> {
    solution::part2(
        sensors,
        Position {
            x: 4000000,
            y: 4000000,
        },
    )
}

mod solution {
    use std::{convert::identity, iter};

    use super::{
        count_beacons_in_range, merge_sorted_ranges, points_of_interest, sensors_edges, Position,
        Sensor,
    };

    pub(super) fn part1(sensors: &[Sensor], y: i64) -> u64 {
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

    pub(super) fn part2(sensors: &[Sensor], maximum_position: Position) -> Option<u64> {
        let edges = sensors_edges(sensors);

        let interesting_ys = edges
            .iter()
            .flat_map(|first_edge| {
                edges
                    .iter()
                    .map(move |second_edge| (first_edge, second_edge))
            })
            .map(|(first_edge, second_edge)| {
                points_of_interest(first_edge, second_edge)
                    .into_iter()
                    .filter_map(identity)
            })
            .flatten()
            .filter(|&y| 0 <= y && y <= maximum_position.y)
            .collect::<Box<[_]>>();

        if let Some(&first) = interesting_ys.first() {
            iter::once(&[first - 1, first])
                .chain(interesting_ys.array_windows::<2>())
                .filter(|[last_y, y]| y != last_y)
                .find_map(|&[_, y]| {
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
        } else {
            None
        }
    }
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

#[derive(Debug, PartialEq, Eq)]
struct Edge {
    from: Position,
    to: Position,
}

impl Edge {
    fn contains_y(&self, y: i64) -> bool {
        let (lower_bound, higher_bound) = if self.from.y < self.to.y {
            (self.from.y, self.to.y)
        } else {
            (self.to.y, self.from.y)
        };

        lower_bound <= y && y <= higher_bound
    }
}

fn sensors_edges(sensors: &[Sensor]) -> Box<[Edge]> {
    sensors
        .iter()
        .map(
            |&Sensor {
                 position: Position { x, y },
                 distance,
                 ..
             }| {
                let distance = distance as i64;

                let left = Position { x: x - distance, y };
                let right = Position { x: x + distance, y };
                let top = Position { x, y: y - distance };
                let bottom = Position { x, y: y + distance };

                [
                    Edge {
                        from: left,
                        to: top,
                    },
                    Edge {
                        from: bottom,
                        to: right,
                    },
                    Edge {
                        from: top,
                        to: right,
                    },
                    Edge {
                        from: left,
                        to: bottom,
                    },
                ]
            },
        )
        .flatten()
        .collect()
}

#[derive(Debug)]
struct Line {
    slope: i64,
    y_intercept: i64,
}

impl Line {
    fn from_points(first: Position, second: Position) -> Self {
        let slope = (second.y - first.y) / (second.x - first.x);
        let y_intercept = first.y - slope * first.x;

        Line { slope, y_intercept }
    }

    fn intersection_y(&self, other: &Line) -> (i64, Option<i64>) {
        let numerator = self.slope * other.y_intercept - other.slope * self.y_intercept;
        let denominator = self.slope - other.slope;

        if numerator % denominator == 0 {
            (numerator / denominator, None)
        } else {
            (numerator / denominator, Some(numerator / denominator + 1))
        }
    }
}

fn points_of_interest(first_edge: &Edge, second_edge: &Edge) -> [Option<i64>; 2] {
    if first_edge == second_edge {
        return [None, None];
    }

    let first_line = Line::from_points(first_edge.from, first_edge.to);
    let second_line = Line::from_points(second_edge.from, second_edge.to);
    if first_line.slope == second_line.slope {
        return [None, None];
    }

    let (y1, y2) = first_line.intersection_y(&second_line);
    let y1 = {
        let contained_by_first = first_edge.contains_y(y1);
        let contained_by_second = second_edge.contains_y(y1);

        contained_by_first && contained_by_second
    }
    .then_some(y1);
    let y2 = y2.and_then(|y2| {
        let contained_by_first = first_edge.contains_y(y2);
        let contained_by_second = second_edge.contains_y(y2);

        (contained_by_first && contained_by_second).then_some(y2)
    });

    [y1, y2]
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
        assert_eq!(
            crate::solution::part1(&crate::parse(INPUT).unwrap(), 10),
            26
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            crate::solution::part2(
                &crate::parse(INPUT).unwrap(),
                crate::Position { x: 20, y: 20 }
            ),
            Some(56000011)
        );
    }
}
