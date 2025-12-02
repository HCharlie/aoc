#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

use anyhow::Result;
use aoc_common::{run_real, Level};

mod day01;
mod day02;

pub fn run_day_real(day: u8, level: Level) -> Result<String> {
    match day {
        1 => run_real(2025, day, level, day01::p1, day01::p2),
        2 => run_real(2025, day, level, day02::p1, day02::p2),
        _ => anyhow::bail!("Invalid day specified: {}", day),
    }
}
