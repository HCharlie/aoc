use anyhow::Result;

fn check_safe(items: &Vec<i32>) -> Result<bool> {
    if items.len() == 1 {
        anyhow::bail!("items length is 1");
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
pub fn p1(input_text: &str) -> Result<i32> {
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

pub fn p2(input_text: &str) -> Result<i32> {
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

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn test_p1_example() {
        let result = p1(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_p2_example() {
        let result = p2(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 4);
    }
}
