use anyhow::Result;

pub fn p1(input_text: &str) -> Result<i64> {
    let mut total: i64 = 0;
    let mut number_grid: Vec<Vec<i64>> = Vec::new();
    let mut symbols: Vec<char> = Vec::new();

    for line in input_text.lines() {
        if line.contains('+') || line.contains('*') {
            for ch in line.chars() {
                if ch == '+' || ch == '*' {
                    symbols.push(ch);
                }
            }
            continue;
        }
        let number_row: Vec<i64> = line
            .split_whitespace()
            .filter_map(|num_str| num_str.parse::<i64>().ok())
            .collect();
        number_grid.push(number_row);
    }
    let rotated_grid: Vec<Vec<i64>> = (0..number_grid[0].len())
        .map(|col| number_grid.iter().map(|row| row[col]).collect())
        .collect();

    for (col_idx, symbol) in symbols.iter().enumerate() {
        let column = &rotated_grid[col_idx];
        match symbol {
            '+' => {
                let col_sum: i64 = column.iter().sum();
                total += col_sum;
            }
            '*' => {
                let col_product: i64 = column.iter().product();
                total += col_product;
            }
            _ => {}
        }
    }

    Ok(total)
}

pub fn p2(input_text: &str) -> Result<i64> {
    let mut total: i64 = 0;
    let mut grid: Vec<Vec<char>> = Vec::new();
    let n = input_text.lines().next().unwrap().chars().count();
    // println!("Number of columns: {}", n);

    for line in input_text.lines() {
        let mut row: Vec<char> = line.chars().collect();
        // Ensure row has exactly n characters, padding with spaces if needed
        row.resize(n, ' ');
        grid.push(row);
    }
    println!("Grid has {} rows", grid.len());
    for row in &grid {
        // println!("Row length: {}", row.len());
        if row.len() != n {
            panic!("Inconsistent row length: expected {}, got {}", n, row.len());
        }
    }
    let mut numbers: Vec<i64> = Vec::new();
    let mut operation: char = ' ';
    for col_idx in (0..n).rev() {
        // println!("Processing column {}", col_idx);
        let mut num_str = String::new();
        for row in &grid {
            let ch = row[col_idx];
            if ch.is_digit(10) {
                num_str.push(ch);
            }
            if ch == '+' || ch == '*' {
                operation = ch;
            }
        }
        if !num_str.is_empty() {
            if let Ok(num) = num_str.parse::<i64>() {
                numbers.push(num);
            }
        }

        if operation == ' ' {
            continue;
        }

        if operation == '+' {
            let sum: i64 = numbers.iter().sum();
            total += sum;
        } else if operation == '*' {
            let product: i64 = numbers.iter().product();
            total += product;
        } else {
            panic!("Unexpected operation: {}", operation);
        }
        numbers.clear();
        operation = ' ';
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_p1_example() {
        let result = p1(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_p2_example() {
        let result = p2(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 3263827);
    }
}
