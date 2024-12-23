#! [deny(clippy::unwrap_used)]
#! [deny(clippy::expect_used)]
#! [deny(clippy::panic)]
#! [deny(unused_must_use)]

use std::time::Instant;

mod days;

#[derive(Debug)]
pub enum Level {
    One = 1,
    Two = 2,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("args: {:?}", args);
    if args.len() != 3 && args.len() != 4 {
        eprintln!("Usage: cargo run <day> <level> [<debug>] or cargo run -- <day> <level> [<debug>]");
        return;
    }

    let day: u8 = match args[1].parse() {
        Ok(d) if d >= 1 && d <= 25 => d,
        _ => {
            eprintln!("Day must be between 1 and 25");
            return;
        }
    };
    let level: Level = match args[2].parse() {
        Ok(1) => Level::One,
        Ok(2) => Level::Two,
        _ => {
            eprintln!("Level must be 1 or 2");
            return;
        }
    };
    let debug: bool = if args.len() == 4 {
        match args[3].parse() {
            Ok(0) => false,
            Ok(1) => true,
            _ => {
                eprintln!("Debug must be 0 (false) or 1 (true)");
                return;
            }
        }
    } else {
        false
    };

    let start = Instant::now();

    match day {
        1 => days::day01::run(day, level, debug),
        2 => days::day02::run(day, level, debug),
        3 => days::day03::run(day, level, debug),
        4 => days::day04::run(day, level, debug),
        5 => days::day05::run(day, level, debug),
        6 => days::day06::run(day, level, debug),
        7 => days::day07::run(day, level, debug),
        8 => days::day08::run(day, level, debug),
        9 => days::day09::run(day, level, debug),
        10 => days::day10::run(day, level, debug),
        11 => days::day11::run(day, level, debug),
        12 => days::day12::run(day, level, debug),
        13 => days::day13::run(day, level, debug),
        14 => days::day14::run(day, level, debug),
        15 => days::day15::run(day, level, debug),
        16 => days::day16::run(day, level, debug),
        17 => days::day17::run(day, level, debug),
        18 => days::day18::run(day, level, debug),
        19 => days::day19::run(day, level, debug),
        20 => days::day20::run(day, level, debug),
        21 => days::day21::run(day, level, debug),
        22 => days::day22::run(day, level, debug),
        23 => days::day23::run(day, level, debug),
        // Add other days here
        _ => eprintln!("Invalid day or level specified! Usage: day level debug"),
    }

    let duration = start.elapsed();
    println!("Command executed in: {:?}", duration);
}
