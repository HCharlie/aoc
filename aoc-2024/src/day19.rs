use anyhow::Result;
use std::collections::HashMap;

pub const EXAMPLE_INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

fn _check_p1(design: &str, towels: &Vec<&str>) -> bool {
    let mut stack = vec![(design, 0)];

    while let Some((remaining, start_idx)) = stack.pop() {
        if remaining.is_empty() {
            return true;
        }

        for &towel in towels {
            if remaining.starts_with(towel) {
                stack.push((&remaining[towel.len()..], start_idx + towel.len()));
            }
        }
    }
    false
}

pub fn p1(input_text: &str) -> Result<String> {
    let parts = input_text.split("\n\n").collect::<Vec<&str>>();
    let towels = parts[0].split(", ").collect::<Vec<&str>>();
    let designs = parts[1].split("\n").collect::<Vec<&str>>();

    let mut cnt = 0;
    for design in designs {
        if _check_p1(design, &towels) {
            cnt += 1;
        }
    } 
    Ok(cnt.to_string())
}


fn _check_p2(design: &str, towels: &Vec<&str>) -> i64 {
    let mut dp: HashMap<usize, i64> = HashMap::new();
    dp.insert(design.len(), 1);  // Base case: empty string has 1 way

    // Iterate from end to start
    for i in (0..design.len()).rev() {
        let mut count = 0;
        for &towel in towels {
            if design[i..].starts_with(towel) {
                if let Some(&ways) = dp.get(&(i + towel.len())) {
                    count += ways;
                }
            }
        }
        dp.insert(i, count);
    }

    match dp.get(&0) {
        Some(&ways) => {
            return ways;
        },
        None => {
            return 0;
        }
        
    }
}

pub fn p2(input_text: &str) -> Result<String> {
    let parts = input_text.split("\n\n").collect::<Vec<&str>>();
    let towels = parts[0].split(", ").collect::<Vec<&str>>();
    let designs = parts[1].split("\n").collect::<Vec<&str>>();

    let mut cnt = 0;
    for design in designs {
        cnt += _check_p2(design, &towels);
    } 
    Ok(cnt.to_string())

}

