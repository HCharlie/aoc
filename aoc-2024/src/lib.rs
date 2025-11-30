#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

pub use aoc_common::{Level, prelude::*};
use anyhow::Result;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

pub fn run_day_example(day: u8, level: Level) {
    match day {
        1 => run_example(level, day01::EXAMPLE_INPUT, day01::p1, day01::p2),
        2 => run_example(level, day02::EXAMPLE_INPUT, day02::p1, day02::p2),
        3 => run_example(level, day03::EXAMPLE_INPUT, day03::p1, day03::p2),
        4 => run_example(level, day04::EXAMPLE_INPUT, day04::p1, day04::p2),
        5 => run_example(level, day05::EXAMPLE_INPUT, day05::p1, day05::p2),
        6 => run_example(level, day06::EXAMPLE_INPUT, day06::p1, day06::p2),
        7 => run_example(level, day07::EXAMPLE_INPUT, day07::p1, day07::p2),
        8 => run_example(level, day08::EXAMPLE_INPUT, day08::p1, day08::p2),
        9 => run_example(level, day09::EXAMPLE_INPUT, day09::p1, day09::p2),
        10 => run_example(level, day10::EXAMPLE_INPUT, day10::p1, day10::p2),
        11 => run_example(level, day11::EXAMPLE_INPUT, day11::p1, day11::p2),
        12 => run_example(level, day12::EXAMPLE_INPUT, day12::p1, day12::p2),
        13 => run_example(level, day13::EXAMPLE_INPUT, day13::p1, day13::p2),
        14 => run_example(level, day14::EXAMPLE_INPUT, day14::p1, day14::p2),
        15 => run_example(level, day15::EXAMPLE_INPUT, day15::p1, day15::p2),
        16 => run_example(level, day16::EXAMPLE_INPUT, day16::p1, day16::p2),
        17 => run_example(level, day17::EXAMPLE_INPUT, day17::p1, day17::p2),
        18 => run_example(level, day18::EXAMPLE_INPUT, day18::p1, day18::p2),
        19 => run_example(level, day19::EXAMPLE_INPUT, day19::p1, day19::p2),
        20 => run_example(level, day20::EXAMPLE_INPUT, day20::p1, day20::p2),
        21 => run_example(level, day21::EXAMPLE_INPUT, day21::p1, day21::p2),
        22 => run_example(level, day22::EXAMPLE_INPUT, day22::p1, day22::p2),
        23 => run_example(level, day23::EXAMPLE_INPUT, day23::p1, day23::p2),
        24 => run_example(level, day24::EXAMPLE_INPUT, day24::p1, day24::p2),
        25 => run_example(level, day25::EXAMPLE_INPUT, day25::p1, day25::p2),
        _ => eprintln!("Invalid day specified!"),
    }
}

pub fn run_day_real(day: u8, level: Level) -> Result<String> {
    match day {
        1 => run_real(2024, day, level, day01::p1, day01::p2),
        2 => run_real(2024, day, level, day02::p1, day02::p2),
        3 => run_real(2024, day, level, day03::p1, day03::p2),
        4 => run_real(2024, day, level, day04::p1, day04::p2),
        5 => run_real(2024, day, level, day05::p1, day05::p2),
        6 => run_real(2024, day, level, day06::p1, day06::p2),
        7 => run_real(2024, day, level, day07::p1, day07::p2),
        8 => run_real(2024, day, level, day08::p1, day08::p2),
        9 => run_real(2024, day, level, day09::p1, day09::p2),
        10 => run_real(2024, day, level, day10::p1, day10::p2),
        11 => run_real(2024, day, level, day11::p1, day11::p2),
        12 => run_real(2024, day, level, day12::p1, day12::p2),
        13 => run_real(2024, day, level, day13::p1, day13::p2),
        14 => run_real(2024, day, level, day14::p1, day14::p2),
        15 => run_real(2024, day, level, day15::p1, day15::p2),
        16 => run_real(2024, day, level, day16::p1, day16::p2),
        17 => run_real(2024, day, level, day17::p1, day17::p2),
        18 => run_real(2024, day, level, day18::p1, day18::p2),
        19 => run_real(2024, day, level, day19::p1, day19::p2),
        20 => run_real(2024, day, level, day20::p1, day20::p2),
        21 => run_real(2024, day, level, day21::p1, day21::p2),
        22 => run_real(2024, day, level, day22::p1, day22::p2),
        23 => run_real(2024, day, level, day23::p1, day23::p2),
        24 => run_real(2024, day, level, day24::p1, day24::p2),
        25 => run_real(2024, day, level, day25::p1, day25::p2),
        _ => anyhow::bail!("Invalid day specified: {}", day),
    }
}