#![feature(array_windows, never_type, result_flattening, option_result_contains)]

use advent_of_code::{Day, Solution};

mod day01;
mod day02;

fn main() {
    advent_of_code::days!(
        01 => (68467, 203420),
        02 => (11475, 16862)
    );
}
