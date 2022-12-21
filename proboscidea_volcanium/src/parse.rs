use std::{cmp::Reverse, collections::HashMap, iter};

#[derive(Debug)]
struct Valve<'i> {
    name: &'i str,
    flow_rate: u8,
    tunnels: Vec<&'i str>,
}

fn parse_valve(valve: &str) -> Option<Valve<'_>> {
    let mut tokens = valve.split_ascii_whitespace();

    let name = tokens.nth(1)?;

    let flow_rate = {
        let (_, flow_rate) = tokens.nth(2)?.split_once('=')?;
        let mut bytes = flow_rate.as_bytes().iter().peekable();
        bytes.next_back()?;

        let mut flow_rate = bytes.next()? - &b'0';
        while let Some(&digit @ b'0'..=b'9') = bytes.peek() {
            bytes.next();

            flow_rate = flow_rate * 10 + digit - b'0';
        }

        Some(flow_rate)
    }?;

    let tunnels = tokens
        .skip(4)
        .map(|name| name.strip_suffix(',').unwrap_or(name))
        .collect();

    Some(Valve {
        name,
        flow_rate,
        tunnels,
    })
}

fn floyd_warshall(valves: &[Valve]) -> crate::PathLengths {
    let valve_name_to_index = valves
        .iter()
        .enumerate()
        .map(|(index, &Valve { name, .. })| (name, index))
        .collect::<HashMap<_, _>>();

    let distance = vec![u8::MAX; valves.len()].into_boxed_slice();
    let mut distances = vec![distance; valves.len()].into_boxed_slice();
    for (i, Valve { tunnels, .. }) in valves.iter().enumerate() {
        for tunnel in tunnels {
            let j = valve_name_to_index[tunnel];

            distances[i][j] = 1;
        }
    }
    for i in 0..distances.len() {
        distances[i][i] = 0;
    }

    for k in 0..distances.len() {
        for i in 0..distances.len() {
            for j in 0..distances.len() {
                let (potential, overflow) = distances[i][k].overflowing_add(distances[k][j]);
                let current = &mut distances[i][j];

                if !overflow && *current > potential {
                    *current = potential;
                }
            }
        }
    }

    distances
}

pub fn parse(input: &str) -> Option<crate::Input> {
    let valves = input
        .lines()
        .map(parse_valve)
        .collect::<Option<Box<[_]>>>()?;
    let shortest_path_lengths = floyd_warshall(&valves);

    let interesting_valve_indices = valves
        .iter()
        .enumerate()
        .filter(
            |(
                _,
                &Valve {
                    name, flow_rate, ..
                },
            )| name == "AA" || flow_rate > 0,
        )
        .map(|(index, _)| index)
        .collect::<Box<[_]>>();
    let starting_valve = interesting_valve_indices
        .iter()
        .position(|&i| valves[i].name == "AA")?;

    let flow_rates = interesting_valve_indices
        .iter()
        .map(|&index| valves[index].flow_rate)
        .collect::<Box<[_]>>();
    let flow_rate_indices = {
        let mut indices = flow_rates.iter().enumerate().collect::<Box<[_]>>();
        indices.sort_unstable_by_key(|&(_, &flow)| Reverse(flow));

        indices.into_iter().map(|&(index, _)| index).collect()
    };

    let path_lengths = interesting_valve_indices
        .iter()
        .map(|&i| {
            interesting_valve_indices
                .iter()
                .map(|&j| shortest_path_lengths[i][j])
                .collect()
        })
        .collect();

    Some(crate::Input {
        starting_valve,

        flow_rates,
        flow_rate_indices,

        path_lengths,
    })
}
