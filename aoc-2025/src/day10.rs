use anyhow::{Ok, Result};
use z3::ast::Int;
use z3::{Optimize, SatResult};

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
        let _joltages = parts[parts.len()-1][1..parts[parts.len()-1].len()-1].split(',').map(|n| n.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        // println!("indicators: {:?}, buttons: {:?}, joltages: {:?}", indicators, buttons, joltages);
        // println!("number of buttons: {}", buttons.len());

        total += solve_p1(indicators, buttons);
    }
    Ok(total)
}

fn solve_p2(
    buttons: Vec<Vec<i64>>,
    joltages: Vec<i64>,
) -> i64 {
    // use z3 to solve the system of equations, when press each button, that adds 1 to the joltages at the positions specified by the button
    // try to find the minimum number of button presses such that the resulting joltages match the target joltages
    
    let optimizer = Optimize::new();
    
    // Create integer variables for each button (number of times pressed)
    let button_vars: Vec<Int> = (0..buttons.len())
        .map(|i| Int::new_const(format!("button_{}", i)))
        .collect();
    
    let zero = Int::from_i64(0);
    
    // Add constraints that all button press counts >= 0
    for var in &button_vars {
        optimizer.assert(&var.ge(&zero));
    }
    
    // For each joltage counter, add constraint that the sum of button presses
    // (for buttons affecting that counter) equals the target
    for (counter_idx, &target) in joltages.iter().enumerate() {
        let mut sum_terms: Vec<Int> = Vec::new();
        
        for (button_idx, button) in buttons.iter().enumerate() {
            // Check if this button affects this counter
            if button.contains(&(counter_idx as i64)) {
                sum_terms.push(button_vars[button_idx].clone());
            }
        }
        
        // Create the sum constraint
        if !sum_terms.is_empty() {
            let refs: Vec<&Int> = sum_terms.iter().collect();
            let sum = Int::add(&refs[..]);
            let target_val = Int::from_i64(target);
            optimizer.assert(&sum.eq(&target_val));
        } else {
            // No button affects this counter, so target must be 0
            if target != 0 {
                return i64::MAX; // Impossible to solve
            }
        }
    }
    
    // Minimize the total number of button presses
    let refs: Vec<&Int> = button_vars.iter().collect();
    let total_presses = Int::add(&refs[..]);
    optimizer.minimize(&total_presses);
    
    // Solve and return the result
    match optimizer.check(&[]) {
        SatResult::Sat => {
            let model = optimizer.get_model().unwrap();
            let result = model.eval(&total_presses, true).unwrap();
            result.as_i64().unwrap()
        }
        _ => i64::MAX, // No solution found
    }
}


pub fn p2(input_text: &str) -> Result<i64> {
    let mut total = 0;
    for line in input_text.lines() {
        let parts = line.split(' ').collect::<Vec<&str>>();
        let _indicators = parts[0][1..parts[0].len()-1].chars().collect::<Vec<char>>();
        let buttons = parts[1..parts.len()-1].to_vec().iter().map(|s| {
            s[1..s.len()-1].split(',').map(|n| n.parse::<i64>().unwrap()).collect::<Vec<i64>>()
        }).collect::<Vec<Vec<i64>>>();
        let joltages = parts[parts.len()-1][1..parts[parts.len()-1].len()-1].split(',').map(|n| n.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        // println!("indicators: {:?}, buttons: {:?}, joltages: {:?}", indicators, buttons, joltages);
        // println!("number of buttons: {}", buttons.len());

        total += solve_p2(buttons, joltages);
    }
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
        assert_eq!(result, 33);
    }
}
