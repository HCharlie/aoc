use anyhow::{Context, Result};
use crate::{Level, get_input_content};

/// Runs only the example input
pub fn run_example<T: std::fmt::Display>(
    level: Level,
    example_input: &str,
    p1: fn(&str) -> Result<T>,
    p2: fn(&str) -> Result<T>,
) {
    let sol_func = match level {
        Level::One => p1,
        Level::Two => p2,
    };

    match sol_func(example_input) {
        Ok(answer) => println!("Example result: {}", answer),
        Err(e) => eprintln!("Error processing example: {}", e),
    }
}

/// Runs the real input with the selected solution function and returns the answer
pub fn run_real<T: std::fmt::Display>(
    year: u16,
    day: u8,
    level: Level,
    p1: fn(&str) -> Result<T>,
    p2: fn(&str) -> Result<T>,
) -> Result<String> {
    let sol_func = match level {
        Level::One => p1,
        Level::Two => p2,
    };

    let content = get_input_content(year, day)
        .context(format!("Failed to get input for year {} day {}", year, day))?;
    
    let answer = sol_func(&content)
        .context("Failed to solve puzzle")?;

    Ok(answer.to_string())
}
