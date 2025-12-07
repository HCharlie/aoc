use std::collections::HashMap;

use anyhow::Result;

pub fn p1(input_text: &str) -> Result<i64> {
    let mut total: i64 = 0;
    let lines: Vec<Vec<char>> = input_text.lines().map(|line| line.chars().collect()).collect();
    let cols = lines[0].len();
    let mut prev_line = lines[0].clone();

    for line in lines.iter().skip(1) {
        let mut curr_line = line.clone();
        for i in 0..cols {
            if curr_line[i] == '^' && prev_line[i] == '|' || prev_line[i] == 'S' {
                total += 1;
                if i > 0 && curr_line[i - 1] == '.' {
                    curr_line[i - 1] = '|';
                }
                if i + 1 < cols && curr_line[i + 1] == '.' {
                    curr_line[i + 1] = '|';
                }
            }
            if curr_line[i] == '.' && prev_line[i] == '|' {
                curr_line[i] = '|';
            }
        }
        prev_line = curr_line;
    }

    Ok(total)
}

pub fn p2(input_text: &str) -> Result<i64> {
    let lines: Vec<Vec<char>> = input_text.lines().map(|line| line.chars().collect()).collect();
    let cols = lines[0].len();
    let rows = lines.len();
    
    // find the start position
    let mut start = (0, 0);
    for row in 0..lines.len() {
        for col in 0..lines[row].len() {
            if lines[row][col] == 'S' {
                start = (row, col);
                break;
            }
        }
    }
    let mut cache: HashMap<(usize, usize), i64> = HashMap::new();
    
    for row in (0..rows).rev() {
        for col in 0..cols {
            if matches!(lines[row][col], '^' | 'S') {
                let mut total = 0;
                if col > 0 {
                    total += (row..rows)
                        .find_map(|r| cache.get(&(r, col - 1)))
                        .copied()
                        .unwrap_or(1);
                }
                if col + 1 < cols {
                    total += (row..rows)
                        .find_map(|r| cache.get(&(r, col + 1)))
                        .copied()
                        .unwrap_or(1);
                }
                cache.insert((row, col), total);
            }
        }
    }
    
    Ok(*cache.get(&start).unwrap_or(&0))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_p1_example() {
        let result = p1(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 21);
    }

    #[test]
    fn test_p2_example() {
        let result = p2(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 40);
    }
}
