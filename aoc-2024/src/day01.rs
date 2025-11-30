use anyhow::Result;
use std::collections::HashMap;

pub const EXAMPLE_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3
";

pub fn p1(input_text: &str) -> Result<i32> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input_text.lines() {
        if line.is_empty() {
            continue;
        }
        let items: Vec<&str> = line.split_whitespace().collect();
        if items.len() < 2 {
            anyhow::bail!("Invalid line format: not enough numbers");
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

pub fn p2(input_text: &str) -> Result<i32> {
    let mut left: HashMap<i32, i32> = HashMap::new();
    let mut right: HashMap<i32, i32> = HashMap::new();

    for line in input_text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let items: Vec<&str> = line.split_whitespace().collect();
        if items.len() != 2 {
            anyhow::bail!("Invalid line format: expected two numbers");
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

