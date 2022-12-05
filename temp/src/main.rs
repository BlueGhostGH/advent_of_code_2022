#![feature(
    array_chunks,
    array_windows,
    never_type,
    result_flattening,
    option_result_contains
)]

use temp::{Day, Solution};

mod day04;

fn main() {
    temp::days!(
        04 => (471, 888)
    );
}
