use anyhow::{Ok, Result};

fn solve_p1(target: Vec<char>, buttons: Vec<Vec<i64>>) -> i64 {
    // convert target to a vector of bools for easier comparison
    let target: Vec<bool> = target.iter().map(|&c| c == '#').collect();
    // iterate over all possible button combinations and return the minimum number of presses that matches the target
    let n_buttons = buttons.len();
    let mut min_presses = n_buttons as i64;
    for i in 0..(1 << n_buttons) {
        let mut current = vec![false; target.len()];
        let mut presses = 0;
        for j in 0..n_buttons {
            if (i & (1 << j)) != 0 {
                presses += 1;
                for &pos in &buttons[j] {
                    if (pos as usize) < current.len() {
                        current[pos as usize] = !current[pos as usize];
                    }
                }
            }
        }
        if current == target {
            min_presses = min_presses.min(presses);
        }
    }
    min_presses
}

pub fn p1(input_text: &str) -> Result<i64> {
    let mut total = 0;
    for line in input_text.lines() {
        let parts = line.split(' ').collect::<Vec<&str>>();
        let indicators = parts[0][1..parts[0].len()-1].chars().collect::<Vec<char>>();
        let buttons = parts[1..parts.len()-1].to_vec().iter().map(|s| {
            s[1..s.len()-1].split(',').map(|n| n.parse::<i64>().unwrap()).collect::<Vec<i64>>()
        }).collect::<Vec<Vec<i64>>>();
        let joltages = parts[parts.len()-1][1..parts[parts.len()-1].len()-1].split(',').map(|n| n.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        // println!("indicators: {:?}, buttons: {:?}, joltages: {:?}", indicators, buttons, joltages);
        // println!("number of buttons: {}", buttons.len());

        total += solve_p1(indicators, buttons);
    }
    Ok(total)
}

pub fn p2(input_text: &str) -> Result<i64> {
    let total = 0;
    Ok(total)
    
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_p1_example() {
        let result = p1(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 7);
    }

    #[test]
    fn test_p2_example() {
        let result = p2(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 0);
    }
}
