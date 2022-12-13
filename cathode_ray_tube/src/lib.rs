use std::convert::identity;

pub const INPUT: &str = include_str!("./input.txt");
pub const DAY: usize = 10;

mod parse;

pub use parse::parse;

type Cycle = i64;

pub fn part1(cycles: &[Cycle]) -> i64 {
    cycles
        .iter()
        .copied()
        .enumerate()
        .map(|(index, cycle)| (index as i64 + 1) * cycle)
        .skip(19)
        .step_by(40)
        .take(6)
        .sum()
}

pub fn part2(cycles: &[Cycle]) -> String {
    const SCREEN_WIDTH: i64 = 40;

    let mut screen = cycles
        .iter()
        .copied()
        .map(|x| [x - 1, x, x + 1])
        .enumerate()
        .flat_map(|(index, cursor)| {
            let index = index as i64 % SCREEN_WIDTH;

            [
                Some(if cursor.contains(&index) { '#' } else { '.' }),
                if index == SCREEN_WIDTH - 1 {
                    Some('\n')
                } else {
                    None
                },
            ]
        })
        .filter_map(identity)
        .collect::<String>();
    screen.pop();

    screen
}

#[cfg(test)]
mod tests {

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn part1() {
        assert_eq!(crate::part1(&crate::parse(INPUT).unwrap()), 13140);
    }

    #[test]
    fn part2() {
        assert_eq!(
            crate::part2(&crate::parse(INPUT).unwrap()),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }
}
