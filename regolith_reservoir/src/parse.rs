use std::collections::VecDeque;

fn parse_vertices(line: &str) -> Option<Box<[crate::Position]>> {
    line.split(" -> ")
        .map(|vertex| {
            let (x, y) = vertex.split_once(',')?;

            Some(crate::Position {
                x: x.parse().ok()?,
                y: y.parse().ok()?,
            })
        })
        .collect::<Option<Box<_>>>()
}

#[derive(Debug)]
struct Path<'vertices> {
    first: Option<crate::Position>,
    current: Option<crate::Position>,
    vertices: &'vertices [crate::Position],

    inner: VecDeque<crate::Position>,
}

impl Iterator for Path<'_> {
    type Item = crate::Position;

    fn next(&mut self) -> Option<Self::Item> {
        self.first.take().or_else(|| {
            self.inner.pop_front().or_else(|| {
                if let Some(&next_vertex) = self.vertices.first() {
                    self.vertices = &self.vertices[1..];
                    let current = self.current.as_mut().unwrap();

                    let direction = crate::Position {
                        x: (next_vertex.x - current.x).signum(),
                        y: (next_vertex.y - current.y).signum(),
                    };

                    let mut current_edge = *current;
                    *current = next_vertex;

                    loop {
                        current_edge += direction;
                        self.inner.push_back(current_edge);

                        if current_edge == next_vertex {
                            break;
                        }
                    }

                    self.inner.pop_front()
                } else {
                    None
                }
            })
        })
    }
}

fn parse_path(line_vertices: &[crate::Position]) -> Path {
    Path {
        first: line_vertices.first().copied(),
        current: line_vertices.first().copied(),
        vertices: &line_vertices[1..],

        inner: VecDeque::new(),
    }
}

pub fn parse(input: &str) -> Option<crate::Reservoir> {
    let vertices = input
        .lines()
        .map(parse_vertices)
        .collect::<Option<Box<_>>>()?;

    let (mut top_left, mut bottom_right) = vertices
        .iter()
        .flat_map(|line_vertices| line_vertices.iter())
        .fold(
            (
                crate::Position { x: 500, y: 0 },
                crate::Position { x: 500, y: 0 },
            ),
            |(top_left, bottom_right), vertex| {
                (
                    crate::Position {
                        x: top_left.x.min(vertex.x),
                        y: top_left.y.min(vertex.y),
                    },
                    crate::Position {
                        x: bottom_right.x.max(vertex.x),
                        y: bottom_right.y.max(vertex.y),
                    },
                )
            },
        );
    bottom_right.y += 2;
    top_left.x = 500 - bottom_right.y - 1;
    bottom_right.x = 500 + bottom_right.y + 1;

    let size = (bottom_right - top_left) + crate::Size { x: 1, y: 1 };

    let mut occupancy = vec![false; size.x as usize * size.y as usize].into_boxed_slice();

    for position in vertices
        .iter()
        .flat_map(|line_vertices| parse_path(line_vertices))
    {
        let index = {
            let with_offset = position - top_left;
            (with_offset.x + with_offset.y * size.x) as usize
        };

        occupancy[index] = true;
    }

    Some(crate::Reservoir {
        occupancy,

        top_left,
        bottom_right,

        size,
    })
}
