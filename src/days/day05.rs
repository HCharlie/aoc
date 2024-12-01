use super::utils::{get_input_content, submit_check_answer};

use crate::Level;

use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;

fn _parse_input(input_text: &str) -> Result<(&str, &str), Box<dyn Error>> {
    let parts: Vec<&str> = input_text.split("\n\n").collect();
    if parts.len() != 2 {
        return Err("Input does not contain exactly two parts separated by an empty line".into());
    }
    Ok((parts[0], parts[1]))
}

fn _build_rules(s: &str) -> Result<HashMap<i32, HashSet<i32>> , Box<dyn Error>> {
    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    for line in s.lines() {
        let items: Vec<i32> = line
            .split("|")
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        if items.len() != 2 {
            return Err("Each line must contain exactly two numbers".into());
        }
        let (should_come_first, should_come_after) = (items[0], items[1]);
        rules.entry(should_come_first as i32).or_insert(HashSet::new()).insert(should_come_after);

    }
    Ok(rules)
}

fn _check(nums: &Vec<i32>, rules: &HashMap<i32, HashSet<i32>>) -> Result<bool, Box<dyn Error>> {
    let mut visited: HashSet<&i32> = HashSet::new();
    for num in nums.iter() {
        if let Some(should_come_after_set) = rules.get(num) {
            for should_come_after in should_come_after_set {
                if visited.contains(should_come_after) {
                    return Ok(false);
                }
            }
        }
        visited.insert(num);
    }

    Ok(true)
}

fn _get_middle(nums: &Vec<i32>) -> Result<i32, Box<dyn Error>> {
    if nums.len() %2 == 0 {
        return Err("The number of elements must be odd".into());
    }
    let middle = nums.len() / 2;

    Ok(nums[middle])
}

fn p1(input_text: &str) -> Result<i32, Box<dyn Error>> {
    
    let mut ans = 0;
    let (part1, part2) = _parse_input(input_text)?;
    let rules = _build_rules(part1)?;

    for line in part2.lines() {
        let nums: Vec<i32> = line
            .split(",")
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        if _check(&nums, &rules)? {
            ans += _get_middle(&nums)?;
        }
    }
    Ok(ans)
}

fn _update_order(nums:&Vec<i32>, rules: &HashMap<i32, HashSet<i32>>) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut indegree: HashMap<i32, i32> = nums.iter().map(|&x| (x, 0)).collect();
    let mut result = Vec::new();
    let mut remaining: HashSet<i32> = nums.iter().copied().collect();

    // Initialize indegree counts
    for &num in nums {
        if let Some(dependents) = rules.get(&num) {
            for &after in dependents {
                if nums.contains(&after) {
                    *indegree.entry(after).or_insert(0) += 1;
                }
            }
        }
    }

    // Process nodes with zero indegree
    while !remaining.is_empty() {
        if let Some(&num) = remaining.iter().find(|&&x| indegree.get(&x).unwrap_or(&0) == &0) {
            result.push(num);
            remaining.remove(&num);
            
            if let Some(dependents) = rules.get(&num) {
                for &dep in dependents {
                    if let Some(count) = indegree.get_mut(&dep) {
                        *count -= 1;
                    }
                }
            }
        } else {
            return Err("Cycle detected in dependencies".into());
        }
    }

    Ok(result)
}

fn p2(input_text: &str) -> Result<i32, Box<dyn Error>> {
    let mut ans = 0;
    let (part1, part2) = _parse_input(input_text)?;
    let rules = _build_rules(part1)?;

    for line in part2.lines() {
        let nums: Vec<i32> = line
            .split(",")
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        if !_check(&nums, &rules)? {
            let update_nums = _update_order(&nums, &rules)?;
            ans += _get_middle(&update_nums)?;
        }
    }
    Ok(ans)
}

pub fn run(day: u8, level: Level, debug: bool) -> () {
    let example_input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

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
