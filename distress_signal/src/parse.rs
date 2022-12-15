use std::iter::Peekable;

fn parse_tokens<Bs>(mut bytes: Peekable<Bs>) -> Option<Box<[crate::Token]>>
where
    Bs: Iterator<Item = u8>,
{
    let mut tokens = Vec::new();

    while let Some(byte) = bytes.next() {
        match byte {
            b'[' => tokens.push(crate::Token::Start),
            b']' => tokens.push(crate::Token::End),
            b',' => {}
            digit @ b'0'..=b'9' => {
                let mut value = digit - b'0';

                while let Some(&digit @ b'0'..=b'9') = bytes.peek() {
                    bytes.next();

                    value = value * 10 + digit - b'0';
                }

                tokens.push(crate::Token::Value { value });
            }
            _ => return None,
        }
    }

    Some(tokens.into_boxed_slice())
}

pub(crate) fn parse_packet(input: &str) -> Option<crate::Packet> {
    parse_tokens(input.as_bytes().iter().copied().peekable()).map(|tokens| crate::Packet { tokens })
}

fn parse_pair([left, right]: [&str; 2]) -> Option<crate::Pair> {
    Some(crate::Pair {
        left: parse_packet(left)?,
        right: parse_packet(right)?,
    })
}

pub fn parse(input: &str) -> Option<Box<[crate::Pair]>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .array_chunks::<2>()
        .map(parse_pair)
        .collect()
}
