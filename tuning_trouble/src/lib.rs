pub const INPUT: &str = include_str!("./input.txt");
pub const DAY: usize = 6;

mod parse;
pub use parse::parse;

const PACKET_MARKER_SIZE: usize = 4;
const MESSAGE_MARKER_SIZE: usize = 14;

pub type Input<'i> = &'i [u8];

pub fn part1(input: &Input) -> Option<usize> {
    solve::<PACKET_MARKER_SIZE>(input)
}

pub fn part2(input: &Input) -> Option<usize> {
    solve::<MESSAGE_MARKER_SIZE>(input)
}

fn solve<const MARKER_SIZE: usize>(input: &Input) -> Option<usize> {
    let mut slider = Slider::<MARKER_SIZE>::new();

    for &byte in input.iter() {
        slider.push(byte);

        if slider.is_unique() {
            return Some(slider.index);
        }
    }

    None
}

#[derive(Debug, Clone)]
struct Slider<const S: usize> {
    circular_window: [u8; S],
    occurences: [u8; 256],
    duplicate_count: usize,
    index: usize,
}

impl<const S: usize> Slider<S> {
    fn new() -> Self {
        Slider {
            circular_window: [0; S],
            occurences: [0; 256],
            duplicate_count: 0,
            index: 0,
        }
    }

    fn is_unique(&self) -> bool {
        self.duplicate_count == 0 && self.index > S
    }

    fn push(&mut self, byte: u8) {
        self.remove();
        self.add(byte);
    }

    fn remove(&mut self) {
        let current_byte = &mut self.circular_window[self.index % S];

        if self.index >= S {
            let occurences = &mut self.occurences[*current_byte as usize];

            if *occurences > 1 {
                self.duplicate_count -= 1;
            }

            *occurences -= 1;
        }
    }

    fn add(&mut self, byte: u8) {
        self.circular_window[self.index % S] = byte;

        let occurences = &mut self.occurences[byte as usize];
        *occurences += 1;

        if *occurences > 1 {
            self.duplicate_count += 1;
        }

        self.index += 1;
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn part1() {
        assert_eq!(crate::part1(&crate::parse(INPUT).unwrap()), Some(7));
    }

    #[test]
    fn part2() {
        assert_eq!(crate::part2(&crate::parse(INPUT).unwrap()), Some(19));
    }
}
