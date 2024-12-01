use super::utils::{get_input_content, submit_check_answer};

use crate::Level;


use std::error::Error;



fn p1(input_text: &str) -> Result<i64, Box<dyn Error>> {
    
    let blinks = 25;

    let mut initial_stones: Vec<i64> = input_text
        .split_whitespace()
        .map(|s| s.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;
    for _ in 0..blinks {
        let mut new_stones: Vec<i64> = Vec::new();
        for stone in initial_stones {
            match stone {
                0 => {
                    new_stones.push(1);
                }
                n if n.to_string().len() % 2 == 0 => {
                    let s = n.to_string();
                    let mid = s.len() / 2;

                    let left = match s[..mid].parse::<i64>() {
                        Ok(n) => n,
                        Err(e) => return Err(Box::new(e))
                    };
                    let right = match s[mid..].parse::<i64>() {
                        Ok(n) => n,
                        Err(e) => return Err(Box::new(e))
                    };
                    new_stones.push(left);
                    new_stones.push(right);
                }
                _ => {
                    new_stones.push(stone * 2024);
                }
                
            }
        }
        initial_stones = new_stones;
    }
    return Ok(initial_stones.len() as i64);

}

fn _calculate_stones(stone: i64, remaining_blinks: i64, visited: &mut std::collections::HashMap<(i64, i64), i64>) -> i64 {

    let key = (stone, remaining_blinks);
    if let Some(&value) = visited.get(&key) {
        return value;
    }
    if remaining_blinks == 0 {
        return 1;
    }
    let result = match stone {
        0 => {
            _calculate_stones(1, remaining_blinks-1, visited)
        }
        n if n.to_string().len() % 2 == 0 => {
            let s = n.to_string();
            let mid = s.len() / 2;
            let left = s[..mid].parse::<i64>().unwrap_or_else(|e| panic!("Failed to parse left part: {}", e));
            let right = s[mid..].parse::<i64>().unwrap_or_else(|e| panic!("Failed to parse right part: {}", e));
            _calculate_stones(left, remaining_blinks - 1, visited) + _calculate_stones(right, remaining_blinks - 1, visited)
        }
        _ => {
            _calculate_stones(stone * 2024, remaining_blinks - 1, visited)
        }
    };

    visited.insert(key, result);
    result
}

fn p2(input_text: &str) -> Result<i64, Box<dyn Error>> {
    let blinks = 75;

    let initial_stones: Vec<i64> = input_text
        .split_whitespace()
        .map(|s| s.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;
    
    let mut memo: std::collections::HashMap<(i64, i64), i64> = std::collections::HashMap::new();

    let mut result = 0;
    for stone in initial_stones {
         result += _calculate_stones(stone, blinks, &mut memo);
    }
    
    
    Ok(result as i64)
}

pub fn run(day: u8, level: Level, debug: bool) -> () {
    let example_input = "125 17";

    let sol_func = match level {
        Level::One => p1,
        Level::Two => p2,
    };

    match sol_func(example_input) {
        Ok(result) => println!("Example result: {}", result),
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
