#![feature(iter_array_chunks, box_syntax)]

pub const INPUT: &str = include_str!("./input.txt");
pub const DAY: usize = 13;

mod parse;
pub use parse::parse;

#[derive(Debug, Clone, Copy)]
enum Token {
    Start,
    End,
    Value { value: u8 },
}

#[derive(Debug)]
struct Packet {
    tokens: Box<[Token]>,
}

#[derive(Debug)]
pub struct Pair {
    left: Packet,
    right: Packet,
}

pub fn part1(pairs: &[Pair]) -> usize {
    pairs
        .iter()
        .enumerate()
        .filter_map(|(index, Pair { left, right })| {
            are_ordered(
                &mut left.tokens.iter().copied(),
                &mut right.tokens.iter().copied(),
            )
            .then_some(index + 1)
        })
        .sum()
}

pub fn part2(pairs: &[Pair]) -> usize {
    let mut packets = pairs.iter().flat_map(|pair| [&pair.left, &pair.right]);

    fn index_of_divider(packets: &mut (dyn Iterator<Item = &Packet>), divider: &Packet) -> usize {
        1 + packets
            .filter(|packet| {
                are_ordered(
                    &mut packet.tokens.iter().copied(),
                    &mut divider.tokens.iter().copied(),
                )
            })
            .count()
    }

    let two = Packet {
        tokens: box [
            Token::Start,
            Token::Start,
            Token::Value { value: 2 },
            Token::End,
            Token::End,
        ],
    };
    let six = Packet {
        tokens: box [
            Token::Start,
            Token::Start,
            Token::Value { value: 6 },
            Token::End,
            Token::End,
        ],
    };

    let index_of_two = index_of_divider(&mut packets.clone(), &two);
    let index_of_six = index_of_divider(&mut packets, &six) + 1;

    index_of_two * index_of_six
}

fn are_ordered(
    left_packet: &mut (dyn Iterator<Item = Token>),
    right_packet: &mut (dyn Iterator<Item = Token>),
) -> bool {
    while let (Some(left), Some(right)) = (left_packet.next(), right_packet.next()) {
        match (left, right) {
            (Token::Start, Token::Start) => {}
            (Token::Start, Token::End) => return false,
            (Token::Start, value @ Token::Value { .. }) => {
                return are_ordered(
                    left_packet,
                    &mut [value, Token::End].into_iter().chain(right_packet),
                )
            }

            (Token::End, Token::Start) => return true,
            (Token::End, Token::End) => {}
            (Token::End, Token::Value { .. }) => return true,

            (value @ Token::Value { .. }, Token::Start) => {
                return are_ordered(
                    &mut [value, Token::End].into_iter().chain(left_packet),
                    right_packet,
                )
            }
            (Token::Value { .. }, Token::End) => return false,
            (Token::Value { value: left }, Token::Value { value: right }) => {
                if left < right {
                    return true;
                } else if right < left {
                    return false;
                }
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn part1() {
        assert_eq!(crate::part1(&crate::parse(INPUT).unwrap()), 13);
    }

    #[test]
    fn part2() {
        assert_eq!(crate::part2(&crate::parse(INPUT).unwrap()), 140);
    }
}
