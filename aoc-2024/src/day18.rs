use anyhow::Result;
use std::collections::{HashSet, VecDeque};

fn parse_grid_params(input_text: &str) -> Result<((i64, i64), &str)> {
    let mut lines = input_text.lines();
    let first_line = lines.next().ok_or_else(|| anyhow::anyhow!("Empty input"))?;

    if let Some(params) = first_line.strip_prefix("GRID: ") {
        let parts: Vec<&str> = params.split_whitespace().collect();
        if parts.len() != 2 {
            anyhow::bail!("Invalid GRID format");
        }
        let grid_size = parts[0].parse::<i64>()?;
        let bytes_falling = parts[1].parse::<i64>()?;
        let remaining = input_text.lines().skip(1).collect::<Vec<_>>().join("\n");
        Ok((
            (grid_size, bytes_falling),
            Box::leak(remaining.into_boxed_str()),
        ))
    } else {
        Err(anyhow::anyhow!("Missing GRID header"))
    }
}

fn p1_impl(input_text: &str, grid_size: i64, bytes_falling: i64) -> Result<String> {
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; grid_size as usize]; grid_size as usize];
    let mut cnt = 0;
    for line in input_text.lines() {
        if cnt >= bytes_falling {
            break;
        }
        let (row, col) = match line.split_once(",") {
            Some((row, col)) => (row.parse::<i64>()?, col.parse::<i64>()?),
            None => {
                anyhow::bail!("Invalid input");
            }
        };

        grid[col as usize][row as usize] = '#';
        cnt += 1;
    }
    let s_row = 0;
    let s_col = 0;
    let e_row = grid_size - 1;
    let e_col = grid_size - 1;

    let mut dq = VecDeque::new();
    let mut seen = HashSet::new();
    dq.push_back((s_row, s_col, 0));
    seen.insert((s_row, s_col));

    // for line in grid.iter() {
    //     println!("{}", line.iter().collect::<String>());
    // }

    while !dq.is_empty() {
        let (row, col, dist) = match dq.pop_front() {
            Some((row, col, dist)) => (row, col, dist),
            None => {
                anyhow::bail!("invalid item in dq");
            }
        };

        if row == e_row && col == e_col {
            return Ok(dist.to_string());
        }

        for (d_row, d_col) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_row = row + d_row;
            let new_col = col + d_col;
            if new_row >= 0
                && new_row < grid_size
                && new_col >= 0
                && new_col < grid_size
                && grid[new_row as usize][new_col as usize] != '#'
                && !seen.contains(&(new_row, new_col))
            {
                dq.push_back((new_row, new_col, dist + 1));
                seen.insert((new_row, new_col));
            }
        }
    }
    Err(anyhow::anyhow!("No path found"))
}

fn p2_impl(input_text: &str, grid_size: i64, bytes_falling: i64) -> Result<String> {
    let total_lines = input_text.lines().count() as i64;
    println!("total_lines: {}", total_lines);
    let mut left = bytes_falling;
    let mut right = total_lines;

    while left < right {
        let mid = (left + right) / 2;
        match p1_impl(input_text, grid_size, mid) {
            Ok(_) => {
                left = mid + 1;
            }
            Err(_) => {
                right = mid - 1;
            }
        }
    }
    println!("left: {}", left);
    println!("right: {}", right);

    let line = input_text
        .lines()
        .nth((left - 1) as usize)
        .ok_or_else(|| anyhow::anyhow!("Line not found"))?;
    println!("line: {}", line);
    return Ok(line.to_string());
}

// Public wrappers for use with centralized orchestration
pub fn p1(input_text: &str) -> Result<String> {
    let ((grid_size, bytes_falling), remaining) = parse_grid_params(input_text)?;
    p1_impl(remaining, grid_size, bytes_falling)
}

pub fn p2(input_text: &str) -> Result<String> {
    let ((grid_size, bytes_falling), remaining) = parse_grid_params(input_text)?;
    p2_impl(remaining, grid_size, bytes_falling)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "GRID: 7 12
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_p1_example() {
        let result = p1(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, "22");
    }

    #[test]
    fn test_p2_example() {
        let result = p2(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, "6,1");
    }
}
