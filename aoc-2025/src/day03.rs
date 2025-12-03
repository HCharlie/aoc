use anyhow::Result;

fn find_largest_number_p1(nums: &Vec<u8>) -> i64 {
    let len = nums.len();
    let mut largest_digit = 0;
    let mut largest_digit_first_index = 0;
    for (i, &num) in nums[0..len-1].iter().enumerate() {
        if num > largest_digit {
            largest_digit = num;
            largest_digit_first_index = i;
        }
    }
    let mut second_digit = nums[largest_digit_first_index+1];
    for &num in &nums[largest_digit_first_index+1..len] {
        if num > second_digit {
            second_digit = num;
        }
    }
    return (largest_digit * 10 + second_digit) as i64;
}

fn find_largest_number_p2(nums: &Vec<u8>) -> i64 {
    let len = nums.len();
    let n = 12; // number of digits to select
    
    let mut result: i64 = 0;
    let mut start_pos = 0; // where to start searching for next digit
    
    for i in 0..n {
        // Calculate the search window: we need to leave enough digits for remaining positions
        let end_pos = len - n + i + 1;
        
        // Find the largest digit in the current search window
        let mut max_digit = nums[start_pos];
        let mut max_digit_pos = start_pos;
        
        for pos in start_pos..end_pos {
            if nums[pos] > max_digit {
                max_digit = nums[pos];
                max_digit_pos = pos;
            }
        }
        
        // Add this digit to result
        result = result * 10 + max_digit as i64;
        
        // Next search starts right after this digit
        start_pos = max_digit_pos + 1;
    }
    
    result
}


pub fn p1(input_text: &str) -> Result<i64> {
    let mut total: i64 = 0;

    for line in input_text.lines() {
        if line.is_empty() {
            continue;
        }
        let nums: Vec<u8> = line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
        let largest_number = find_largest_number_p1(&nums);
        total += largest_number as i64;
    }
    Ok(total)
}

pub fn p2(input_text: &str) -> Result<i64> {
    let mut total: i64 = 0;

    for line in input_text.lines() {
        if line.is_empty() {
            continue;
        }
        let nums: Vec<u8> = line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
        let largest_number = find_largest_number_p2(&nums);
        total += largest_number as i64;
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_p1_example() {
        let result = p1(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 357);
    }

    #[test]
    fn test_p2_example() {
        let result = p2(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 3121910778619);
    }
}
