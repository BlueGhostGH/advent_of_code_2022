#![feature(box_syntax)]

fn bench<F, R>(fun: F) -> (R, ::std::time::Duration)
where
    F: FnOnce() -> R,
{
    let now = ::std::time::Instant::now();
    let res = fun();

    (res, now.elapsed())
}

pub trait Day {
    fn day() -> usize;

    fn input() -> &'static str;
}

#[macro_export]
macro_rules! day {
    ($day: expr) => {
        paste::paste! {
            pub struct [<Day $day>];

            impl advent_of_code::Day for [<Day $day>] {
                fn day() -> usize {
                    $day
                }

                fn input() -> &'static str {
                    include_str!(concat!("../input/", stringify!($day), ".txt"))
                }
            }
        }
    };
}

pub trait Result: ::std::fmt::Debug + ::std::cmp::PartialEq + 'static {}

impl<T> Result for T where T: ::std::fmt::Debug + ::std::cmp::PartialEq + 'static {}

#[derive(Debug)]
pub struct Bench<P1, P2>
where
    P1: Result,
    P2: Result,
{
    pub part_1: (P1, ::std::time::Duration),
    pub part_2: (P2, ::std::time::Duration),
    pub total: ::std::time::Duration,
}

#[derive(Debug)]
pub struct WrongResult {
    expected: Box<dyn ::std::fmt::Debug>,
    actual: Box<dyn ::std::fmt::Debug>,
}

impl ::std::fmt::Display for WrongResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let WrongResult { expected, actual } = self;

        f.write_fmt(format_args!(
            "expected answer {expected:?} but instead got {actual:?}"
        ))
    }
}

impl ::std::error::Error for WrongResult {}

pub trait Solution<'a>: Day {
    type Input: ::std::ops::Deref;
    type ParseError: ::std::error::Error;

    type P1: Result;
    type P2: Result;

    fn parse(input: &'a str) -> ::std::result::Result<Self::Input, Self::ParseError>;

    fn part1(input: &<Self::Input as ::std::ops::Deref>::Target) -> Self::P1;

    fn part2(input: &<Self::Input as ::std::ops::Deref>::Target) -> Self::P2;

    fn solve(input: &'a str) -> ::std::result::Result<(Self::P1, Self::P2), Self::ParseError> {
        let input = Self::parse(input)?;

        let p1 = Self::part1(&input);
        let p2 = Self::part2(&input);

        Ok((p1, p2))
    }

    fn run(
        input: &'static str,
        ans_1: Self::P1,
        ans_2: Self::P2,
    ) -> ::std::result::Result<Bench<Self::P1, Self::P2>, WrongResult> {
        let input = Self::parse(input).unwrap();

        let (p1, p1_dur) = bench(|| Self::part1(&input));
        let (p2, p2_dur) = bench(|| Self::part2(&input));
        let total_dur = p1_dur + p2_dur;

        let res = if p1 != ans_1 {
            Err(WrongResult {
                expected: box ans_1,
                actual: box p1,
            })
        } else if p2 != ans_2 {
            Err(WrongResult {
                expected: box ans_2,
                actual: box p2,
            })
        } else {
            Ok(Bench {
                part_1: (p1, p1_dur),
                part_2: (p2, p2_dur),
                total: total_dur,
            })
        };

        res
    }
}

#[macro_export]
macro_rules! days {
    ($($day: expr => ($ans_1: expr, $ans_2: expr)),+) => {
        paste::paste! {
            $(
                let [<day_ $day>] = [<day $day>]::[<Day $day>]::run([<day $day>]::[<Day $day>]::input(), $ans_1, $ans_2).unwrap();
            )+

            $(
                println!(
                    "Day {}({:?}):\n    Part 1({:?}): {:?}\n    Part 2({:?}): {:?}",
                    $day, [<day_ $day>].total, [<day_ $day>].part_1.1, [<day_ $day>].part_1.0, [<day_ $day>].part_2.1, [<day_ $day>].part_2.0
                );
            )+

            let total: ::std::time::Duration = [
                $(
                    [<day_ $day>].total
                ),+
            ].iter().sum();

            println!("\nTotal: {:?}", total);
        }
    };
}
