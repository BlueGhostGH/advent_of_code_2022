#[macro_export]
macro_rules! days {
    ($($day: ident => ($part1: expr, $part2: expr)),+) => {
        paste::paste! {
            let mut total_days = ::std::time::Duration::ZERO;
            let total_real = ::std::time::Instant::now();

            $(
                {
                    let day = [<$day>]::DAY;

                    let time = ::std::time::Instant::now();
                    let input = [<$day>]::parse([<$day>]::INPUT);
                    let input_time = time.elapsed();
                    let input = match input {
                        Some(input) => input,
                        None => {
                            println!(
                                "Day {day}: could not parse input",
                            );

                            return;
                        }
                    };


                    let time = ::std::time::Instant::now();
                    let part1 = [<$day>]::part1(&input);
                    let part1_time = time.elapsed();
                    let part1 = if part1 == $part1 {
                        part1
                    } else {
                        return println!(
                            "Day {day}:\n    Parsing: {input_time:?}\n    Part 1 ({part1_time:?}): expected answer {:?} but instead got {part1:?}", $part1,
                        );
                    };

                    let time = ::std::time::Instant::now();
                    let part2 = [<$day>]::part2(&input);
                    let part2_time = time.elapsed();
                    let part2 = if part2 == $part2 {
                        part2
                    } else {
                        return println!(
                            "Day {day}:\n    Parsing: {input_time:?}\n    Part 1 ({part1_time:?}): {part1:?}\n    Part 2 ({part2_time:?}): expected answer {:?} but instead got {part2:?}", $part2,
                        );
                    };

                    let total_time = input_time + part1_time + part2_time;
                    total_days += total_time;

                    println!(
                        "Day {day} ({total_time:?}):\n    Parsing: {input_time:?}\n    Part 1 ({part1_time:?}): {part1:?}\n    Part 2 ({part2_time:?}): {part2:?}",
                    );
                }
            )+

            let total_real = total_real.elapsed();

            println!("\nTotal times:\n    Days: {total_days:?}\n    Real: {total_real:?}")
        }
    };
}
