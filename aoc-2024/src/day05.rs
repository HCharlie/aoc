use anyhow::Result;
use std::collections::HashMap;
use std::collections::HashSet;

fn _parse_input(input_text: &str) -> Result<(&str, &str)> {
    let parts: Vec<&str> = input_text.split("\n\n").collect();
    if parts.len() != 2 {
        anyhow::bail!("Input does not contain exactly two parts separated by an empty line");
    }
    Ok((parts[0], parts[1]))
}

fn _build_rules(s: &str) -> Result<HashMap<i32, HashSet<i32>> , > {
    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    for line in s.lines() {
        let items: Vec<i32> = line
            .split("|")
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        if items.len() != 2 {
            anyhow::bail!("Each line must contain exactly two numbers");
        }
        let (should_come_first, should_come_after) = (items[0], items[1]);
        rules.entry(should_come_first as i32).or_insert(HashSet::new()).insert(should_come_after);

    }
    Ok(rules)
}

fn _check(nums: &Vec<i32>, rules: &HashMap<i32, HashSet<i32>>) -> Result<bool> {
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

fn _get_middle(nums: &Vec<i32>) -> Result<i32> {
    if nums.len() %2 == 0 {
        anyhow::bail!("The number of elements must be odd");
    }
    let middle = nums.len() / 2;

    Ok(nums[middle])
}

pub fn p1(input_text: &str) -> Result<i32> {
    
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

fn _update_order(nums:&Vec<i32>, rules: &HashMap<i32, HashSet<i32>>) -> Result<Vec<i32>> {
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
            anyhow::bail!("Cycle detected in dependencies");
        }
    }

    Ok(result)
}

pub fn p2(input_text: &str) -> Result<i32> {
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

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "47|53
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

    #[test]
    fn test_p1_example() {
        let result = p1(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 143);
    }

    #[test]
    fn test_p2_example() {
        let result = p2(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 123);
    }
}

