use super::utils::{get_input_content, submit_check_answer};

use crate::Level;
use regex::Regex;

fn p1(input_text: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)")?;
    let mut sum = 0;

    for cap in re.captures_iter(input_text) {
        let x: i32 = cap[1].parse()?;
        let y: i32 = cap[2].parse()?;
        sum += x * y;
    }

    Ok(sum)
}

fn p2(input_text: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut sum = 0;
    let mut enabled = true;

    let re_combined = Regex::new(r"(do\(\)|don't\(\)|mul\((\d+),(\d+)\))")?;

    for cap in re_combined.captures_iter(input_text) {
        if let Some(matched_str) = cap.get(0) {
            let s = matched_str.as_str();
            if s == "do()" {
                enabled = true;
            } else if s == "don't()" {
                enabled = false;
            } else if enabled {
                if let (Some(x_match), Some(y_match)) = (cap.get(2), cap.get(3)) {
                    let x: i32 = x_match.as_str().parse()?;
                    let y: i32 = y_match.as_str().parse()?;
                    sum += x * y;
                }
            }
        }
    }

    Ok(sum)
}

pub fn run(day: u8, level: Level, debug:    bool) -> () {
    let example_input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    let sol_func = match level {
        Level::One => p1,
        Level::Two => p2,
    };

    match sol_func(example_input) {
        Ok(answer) => println!("Example result: {}", answer),
        Err(e) => eprintln!("Error processing example: {}", e),
    }

    let content = match get_input_content(day) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            return;
        }
    };

    let answer = match sol_func(&content) {
        Ok(answer) => answer,
        Err(e) => {
            eprintln!("Error processing input: {}", e);
            return;
        }
    };
    if debug {
        println!("Answer: {}", answer);
        return ();
    }

    match submit_check_answer(day, level as u8, &answer.to_string()) {
        Ok(is_correct) => println!(
            "Answer {} is {}",
            answer,
            if is_correct { "correct" } else { "wrong" }
        ),
        Err(e) => {
            eprintln!("Error submitting answer: {}", e);
            return;
        }
    }
}
