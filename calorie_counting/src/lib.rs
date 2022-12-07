pub const INPUT: &str = include_str!("./input.txt");
pub const DAY: usize = 1;

mod parse;
pub use parse::parse;

type Calories = u64;

#[derive(Debug)]
pub struct Elf {
    sum: Calories,
}

pub fn part1(elves: &[Elf]) -> Option<Calories> {
    elves.iter().map(|elf| elf.sum).max()
}

pub fn part2(elves: &[Elf]) -> Option<Calories> {
    let mut elves = elves.iter().map(|elf| elf.sum);

    let top = [elves.next()?, elves.next()?, elves.next()?];
    let top = elves.fold(top, |mut top, elf| {
        ordered_insert::<3, u64>(&mut top, elf);

        top
    });

    Some(top.into_iter().sum())
}

fn ordered_insert<const C: usize, T>(slice: &mut [T; C], value: T)
where
    T: Copy + PartialOrd,
{
    for i in 0..C {
        if slice[i] < value {
            slice.copy_within(i..C - 1, i + 1);
            slice[i] = value;
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part1() {
        assert_eq!(crate::part1(&crate::parse(INPUT).unwrap()), Some(24000));
    }

    #[test]
    fn part2() {
        assert_eq!(crate::part2(&crate::parse(INPUT).unwrap()), Some(24000));
    }
}
