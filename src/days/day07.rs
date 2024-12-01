use super::utils::{get_input_content, submit_check_answer};

use crate::Level;


use std::error::Error;

fn _add(a: i64, b: i64) -> i64 {
    a + b
}

fn _mul(a: i64, b: i64) -> i64 {
    a * b
}

fn _concat(a: i64, b: i64) -> i64 {
    a * 10_i64.pow(b.to_string().len() as u32) + b
}

fn _dfs(target: i64, nums: &Vec<i64>, idx: usize, current: i64) -> Result<bool, Box<dyn Error>> {
    if idx == nums.len() {
        return Ok(target == current);
    }
    let next = nums[idx];
    return Ok(_dfs(target, nums, idx + 1, _add(current, next))? || _dfs(target, nums, idx + 1, _mul(current, next))?);
}

fn _dfs_p2(target: i64, nums: &Vec<i64>, idx: usize, current: i64) -> Result<bool, Box<dyn Error>> {
    if idx == nums.len() {
        return Ok(target == current);
    }
    let next = nums[idx];
    return Ok(_dfs_p2(target, nums, idx + 1, _add(current, next))? || _dfs_p2(target, nums, idx + 1, _mul(current, next))? || _dfs_p2(target, nums, idx + 1, _concat(current, next))?);
}

fn p1(input_text: &str) -> Result<i64, Box<dyn Error>> {
    let mut result = 0;
    
    for line in input_text.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let test_value: i64 = parts[0].parse()?;
        let nums: Vec<i64> = parts[1].split(" ").map(|x| x.parse()).collect::<Result<Vec<i64>, _>>()?;
        if _dfs(test_value, &nums, 1, nums[0])? {
            result += test_value;
        }
        
    }
    Ok(result)
}


fn p2(input_text: &str) -> Result<i64, Box<dyn Error>> {
    let mut result = 0;
    
    for line in input_text.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let test_value: i64 = parts[0].parse()?;
        let nums: Vec<i64> = parts[1].split(" ").map(|x| x.parse()).collect::<Result<Vec<i64>, _>>()?;
        if _dfs_p2(test_value, &nums, 1, nums[0])? {
            result += test_value;
        }
        
    }
    Ok(result)
}

pub fn run(day: u8, level: Level, debug: bool) -> () {
    let example_input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

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
