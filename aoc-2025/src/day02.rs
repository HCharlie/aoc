use anyhow::Result;

fn is_valid_p1(num: i64) -> bool {
    let num_str = num.to_string();
    if num_str.len() % 2 != 0 {
        return false;
    }
    let half_len = num_str.len() / 2;
    &num_str[..half_len] == &num_str[half_len..]
}

fn is_valid_p2(num: i64) -> bool {
    let num_str = num.to_string();
    let l = num_str.len();
    for i in 1..=l / 2 {
        if l % i != 0 {
            continue;
        }
        let part = &num_str[..i];
        if part.repeat(l / i) == num_str {
            return true;
        }
    }
    false
}

pub fn p1(input_text: &str) -> Result<i64> {
    let mut total = 0;

    for line in input_text.lines() {
        if line.is_empty() {
            continue;
        }
        for range_str in line.split(',') {
            let mut parts = range_str.split('-');
            let start: i64 = parts.next().unwrap().parse()?;
            let end: i64 = parts.next().unwrap().parse()?;
            // println!("Range: {}-{}, Diff: {}", start, end, end - start);
            for num in start..=end {
                if is_valid_p1(num) {
                    total += num;
                }
            }
        }
    }
    Ok(total)
}

pub fn p2(input_text: &str) -> Result<i64> {
    let mut total = 0;

    for line in input_text.lines() {
        if line.is_empty() {
            continue;
        }
        for range_str in line.split(',') {
            let mut parts = range_str.split('-');
            let start: i64 = parts.next().unwrap().parse()?;
            let end: i64 = parts.next().unwrap().parse()?;
            // println!("Range: {}-{}, Diff: {}", start, end, end - start);
            for num in start..=end {
                if is_valid_p2(num) {
                    total += num;
                }
            }
        }
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_p1_example() {
        let result = p1(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn test_p2_example() {
        let result = p2(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 4174379265);
    }
}
