#[macro_export]
macro_rules! days {
    ($($day: ident => ($part1: expr, $part2: expr)),+) => {
        paste::paste! {
            $(
                {
                    let day = [<$day>]::DAY;

                    let time = ::std::time::Instant::now();
                    let input = [<$day>]::parse([<$day>]::INPUT);
                    let input_time = time.elapsed();
                    let input = match input {
                        Ok(input) => input,
                        Err(errs) => {
                            let err = &errs[0]; // We don't care about displaying all errors
                            let source = &[<$day>]::INPUT[err.span()];

                            println!(
                                "Day {day}: could not parse \"{source}\"",
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

                    println!(
                        "Day {day} ({total_time:?}):\n    Parsing: {input_time:?}\n    Part 1 ({part1_time:?}): {part1:?}\n    Part 2 ({part2_time:?}): {part2:?}",
                    );
                }
            )+
            // $(
            //     let [<day_ $day>] = [<day $day>]::[<Day $day>]::run([<day $day>]::[<Day $day>]::input(), $ans_1, $ans_2).unwrap();
            // )+

            // $(
            //     println!(
            //         "Day {}({:?}):\n    Part 1({:?}): {:?}\n    Part 2({:?}): {:?}",
            //         $day, [<day_ $day>].total, [<day_ $day>].part_1.1, [<day_ $day>].part_1.0, [<day_ $day>].part_2.1, [<day_ $day>].part_2.0
            //     );
            // )+

            // let total: ::std::time::Duration = [
            //     $(
            //         [<day_ $day>].total
            //     ),+
            // ].iter().sum();

            // println!("\nTotal: {:?}", total);
        }
    };
}
