use anyhow::Result;

fn _add(a: i64, b: i64) -> i64 {
    a + b
}

fn _mul(a: i64, b: i64) -> i64 {
    a * b
}

fn _concat(a: i64, b: i64) -> i64 {
    a * 10_i64.pow(b.to_string().len() as u32) + b
}

fn _dfs(target: i64, nums: &Vec<i64>, idx: usize, current: i64) -> Result<bool> {
    if idx == nums.len() {
        return Ok(target == current);
    }
    let next = nums[idx];
    return Ok(_dfs(target, nums, idx + 1, _add(current, next))? || _dfs(target, nums, idx + 1, _mul(current, next))?);
}

fn _dfs_p2(target: i64, nums: &Vec<i64>, idx: usize, current: i64) -> Result<bool> {
    if idx == nums.len() {
        return Ok(target == current);
    }
    let next = nums[idx];
    return Ok(_dfs_p2(target, nums, idx + 1, _add(current, next))? || _dfs_p2(target, nums, idx + 1, _mul(current, next))? || _dfs_p2(target, nums, idx + 1, _concat(current, next))?);
}

pub fn p1(input_text: &str) -> Result<i64> {
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


pub fn p2(input_text: &str) -> Result<i64> {
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

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_p1_example() {
        let result = p1(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 3749);
    }

    #[test]
    fn test_p2_example() {
        let result = p2(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 11387);
    }
}

