use anyhow::Result;

pub fn p1(input_text: &str) -> Result<i64> {
    let mut total: i64 = 0;
    let mut ranges: Vec<Vec<i64>> = Vec::new();
    let mut start_counting: bool = false;
    for line in input_text.lines() {
        if line.is_empty() {
            start_counting = true;
            continue;
        }
        if !start_counting {
            ranges.push(line.split('-').map(|x| x.parse::<i64>().unwrap()).collect());
            continue;
        }
        let num: i64 = line.parse::<i64>().unwrap();
        for range in ranges.iter() {
            if num >= range[0] && num <= range[1] {
                total += 1;
                break;
            }
        }
    }
    Ok(total)
}

pub fn p2(input_text: &str) -> Result<i64> {
    let mut total: i64 = 0;
    let mut ranges: Vec<Vec<i64>> = Vec::new();
    for line in input_text.lines() {
        if line.is_empty() {
            break;
        }
        ranges.push(line.split('-').map(|x| x.parse::<i64>().unwrap()).collect());
    }
    ranges.sort_by_key(|x| x[0]);
    let mut curr_start = ranges[0][0];
    let mut curr_end = ranges[0][1];
    for range in ranges.iter().skip(1) {
        if range[0] > curr_end {
            total += curr_end - curr_start + 1;
            curr_start = range[0];
            curr_end = range[1];
        } else {
            curr_end = range[1].max(curr_end);
        }
    }
    total += curr_end - curr_start + 1;
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_p1_example() {
        let result = p1(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_p2_example() {
        let result = p2(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 14);
    }
}
