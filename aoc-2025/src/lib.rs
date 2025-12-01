#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

use aoc_common::{Level, run_example, run_real};
use anyhow::Result;

mod day01;


pub fn run_day_example(day: u8, level: Level) {
    match day {
        1 => run_example(level, day01::EXAMPLE_INPUT, day01::p1, day01::p2),
        _ => eprintln!("Invalid day specified!"),
    }
}

pub fn run_day_real(day: u8, level: Level) -> Result<String> {
    match day {
        1 => run_real(2025, day, level, day01::p1, day01::p2),
        _ => anyhow::bail!("Invalid day specified: {}", day),
    }
}