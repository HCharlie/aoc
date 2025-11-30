use anyhow::Result;
use regex::Regex;

pub const EXAMPLE_INPUT: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

pub fn p1(input_text: &str) -> Result<i32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)")?;
    let mut sum = 0;

    for cap in re.captures_iter(input_text) {
        let x: i32 = cap[1].parse()?;
        let y: i32 = cap[2].parse()?;
        sum += x * y;
    }

    Ok(sum)
}

pub fn p2(input_text: &str) -> Result<i32> {
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

