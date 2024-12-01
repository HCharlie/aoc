use super::utils::{get_input_content, submit_check_answer};

use crate::Level;

fn check_safe(items: &Vec<i32>) -> Result<bool, Box<dyn std::error::Error>> {
    if items.len() == 1 {
        return Err("items length is 1".into());
    }

    let diff = items[1] - items[0];
    if diff.abs() < 1 || diff.abs() > 3 {
        return Ok(false);
    }
    for idx in 2..items.len() {
        let diff = items[idx] - items[idx - 1];
        if diff.abs() < 1 || diff.abs() > 3 {
            return Ok(false);
        }
        if (items[idx] - items[idx - 1]) * (items[idx - 1] - items[idx - 2]) < 0 {
            return Ok(false);
        }
    }
    Ok(true)
}
fn p1(input_text: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut counter = 0;
    for line in input_text.lines() {
        if line.is_empty() {
            continue;
        }
        let items: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()?;
        if check_safe(&items)? {
            counter += 1;
        }
    }
    Ok(counter)
}

fn p2(input_text: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut counter = 0;
    for line in input_text.lines() {
        if line.is_empty() {
            continue;
        }
        let items: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()?;
        if check_safe(&items)? {
            counter += 1;
        } else {
            for i in 0..items.len() {
                let mut new_items = items.clone();
                new_items.remove(i);
                if check_safe(&new_items)? {
                    counter += 1;
                    break;
                }
            }
        }
    }
    Ok(counter)
}

pub fn run(day: u8, level: Level, debug: bool) -> () {
    let example_input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
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
