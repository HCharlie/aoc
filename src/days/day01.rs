use super::utils::{get_input_content, submit_check_answer};

use crate::Level;
use std::collections::HashMap;

fn p1(input_text: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input_text.lines() {
        if line.is_empty() {
            continue;
        }
        let items: Vec<&str> = line.split_whitespace().collect();
        if items.len() < 2 {
            return Err("Invalid line format: not enough numbers".into());
        }
        left.push(items[0].parse::<i32>()?);
        right.push(items[items.len() - 1].parse::<i32>()?);
    }

    left.sort();
    right.sort();

    Ok(left
        .iter()
        .zip(right.iter())
        .map(|(i, j)| (i - j).abs())
        .sum())
}

fn p2(input_text: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut left: HashMap<i32, i32> = HashMap::new();
    let mut right: HashMap<i32, i32> = HashMap::new();

    for line in input_text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let items: Vec<&str> = line.split_whitespace().collect();
        if items.len() != 2 {
            return Err("Invalid line format: expected two numbers".into());
        }
        let left_key = items[0].parse::<i32>()?;
        let right_key = items[1].parse::<i32>()?;
        *left.entry(left_key).or_insert(0) += 1;
        *right.entry(right_key).or_insert(0) += 1;
    }

    Ok(left
        .iter()
        .map(|(&key, &left_count)| right.get(&key).unwrap_or(&0) * key * left_count)
        .sum())
}

pub fn run(day: u8, level: Level, debug: bool) -> () {
    let example_input = "3   4
4   3
2   5
1   3
3   9
3   3
";

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
