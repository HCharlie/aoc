#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

use anyhow::Result;
use aoc_common::{run_real, Level};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

pub fn run_day_real(day: u8, level: Level) -> Result<String> {
    match day {
        1 => run_real(2025, day, level, day01::p1, day01::p2),
        2 => run_real(2025, day, level, day02::p1, day02::p2),
        3 => run_real(2025, day, level, day03::p1, day03::p2),
        4 => run_real(2025, day, level, day04::p1, day04::p2),
        5 => run_real(2025, day, level, day05::p1, day05::p2),
        6 => run_real(2025, day, level, day06::p1, day06::p2),
        7 => run_real(2025, day, level, day07::p1, day07::p2),
        8 => run_real(2025, day, level, day08::p1, day08::p2),
        _ => anyhow::bail!("Invalid day specified: {}", day),
    }
}
