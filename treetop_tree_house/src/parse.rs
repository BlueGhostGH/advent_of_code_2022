use std::cell::Cell;

pub fn parse(input: &str) -> Option<crate::Forest> {
    if !input.is_ascii() {
        return None;
    }

    enum State {
        Text,
        Eol,
    }

    let mut width = 0;
    let mut stride = 0;
    let mut height = 0;

    let mut len = 0;
    let mut state = State::Text;

    for tree in input.trim().as_bytes() {
        match (&state, tree) {
            (State::Text, b'\r' | b'\n') => {
                if width < len {
                    width = len;
                }
                height += 1;
                state = State::Eol;
            }
            (State::Text, _) => len += 1,
            (State::Eol, b'\r' | b'\n') => len += 1,
            (State::Eol, _) => {
                if stride < len {
                    stride = len;
                }
                len = 0;
                state = State::Text;
            }
        }
    }

    let seen = vec![Cell::new(false); input.len()].into_boxed_slice();

    Some(crate::Forest {
        width,
        height: height + 1,
        stride: stride + 1,
        trees: input.as_bytes(),
        seen,
    })
}
