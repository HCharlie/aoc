use anyhow::{Context, Result};
use aoc_common::{Level, submit_check_answer};
use clap::Parser;
use std::time::Instant;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(name = "aoc")]
#[command(about = "Advent of Code Solutions Runner", long_about = None)]
#[command(version)]
struct Args {
    /// Year (2015-2025)
    year: u16,
    
    /// Day (1-25)
    day: u8,
    
    /// Level: 1 or 2
    level: String,
    
    /// Submit the answer to Advent of Code
    #[arg(short, long)]
    submit: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Validate year
    if args.year < 2015 || args.year > 2025 {
        anyhow::bail!("Year {} is out of range (2015-2025)", args.year);
    }
    
    // Validate day
    if args.day < 1 || args.day > 25 {
        anyhow::bail!("Day {} is out of range (1-25)", args.day);
    }
    
    // Parse level
    let level = Level::from_str(&args.level)
        .context("Invalid level specified (use '1' or '2')")?;

    match args.year {
        2024 => {
            aoc2024::run_day_example(args.day, level);
        }
        _ => {
            anyhow::bail!("Year {} is not implemented yet", args.year);
        }
    }

    // Benchmark the solution
    let start = Instant::now();
    
    // Run real input and optionally submit
    let answer = match args.year {
        2024 => aoc2024::run_day_real(args.day, level)?,
        _ => anyhow::bail!("Year {} is not implemented yet", args.year),
    };

    let duration = start.elapsed();
    println!("Total execution time: {:?}", duration);
    println!("Answer: {}", answer);
    
    if args.submit {
        let is_correct = submit_check_answer(args.year, args.day, level as u8, &answer)
            .context("Failed to submit answer")?;
        
        println!(
            "Submission result: {}",
            if is_correct { "correct" } else { "wrong" }
        );
    }
    
    Ok(())
}
