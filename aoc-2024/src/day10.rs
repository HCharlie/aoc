use anyhow::Result;
use std::collections::VecDeque;

fn _helper_p1(grid: &Vec<Vec<i64>>, i: usize, j: usize, row: usize, col: usize) -> i64 {
    let mut visited = vec![vec![false; col]; row];
    let mut queue = VecDeque::new();
    let mut count = 0;

    if grid[i][j] == 0 {
        queue.push_back((i, j));
        visited[i][j] = true;
    }

    while let Some((x, y)) = queue.pop_front() {
        for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && nx < row as isize && ny >= 0 && ny < col as isize {
                let nx = nx as usize;
                let ny = ny as usize;

                if !visited[nx][ny] && grid[nx][ny] == grid[x][y] + 1 {
                    visited[nx][ny] = true;
                    queue.push_back((nx, ny));

                    if grid[nx][ny] == 9 {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

pub fn p1(input_text: &str) -> Result<i64> {
    let mut scores: i64 = 0;
    let grid: Vec<Vec<i64>> = input_text
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .ok_or_else(|| anyhow::anyhow!("Invalid digit: {}", c))
                        .map(|d| d as i64)
                })
                .collect::<Result<Vec<i64>>>()
        })
        .collect::<Result<Vec<Vec<i64>>>>()?;
    let rows = grid.len();
    let cols = grid[0].len();

    for i in 0..rows {
        for j in 0..cols {
            scores += _helper_p1(&grid, i, j, rows, cols);
        }
    }

    Ok(scores)
}

fn _helper_p2(grid: &Vec<Vec<i64>>, i: usize, j: usize, row: usize, col: usize) -> i64 {
    let mut queue = VecDeque::new();
    let mut count = 0;

    if grid[i][j] == 0 {
        queue.push_back((i, j));
    }

    while let Some((x, y)) = queue.pop_front() {
        for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && nx < row as isize && ny >= 0 && ny < col as isize {
                let nx = nx as usize;
                let ny = ny as usize;

                if grid[nx][ny] == grid[x][y] + 1 {
                    queue.push_back((nx, ny));

                    if grid[nx][ny] == 9 {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

pub fn p2(input_text: &str) -> Result<i64> {
    let mut scores: i64 = 0;
    let grid: Vec<Vec<i64>> = input_text
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .ok_or_else(|| anyhow::anyhow!("Invalid digit: {}", c))
                        .map(|d| d as i64)
                })
                .collect::<Result<Vec<i64>>>()
        })
        .collect::<Result<Vec<Vec<i64>>>>()?;
    let rows = grid.len();
    let cols = grid[0].len();

    for i in 0..rows {
        for j in 0..cols {
            scores += _helper_p2(&grid, i, j, rows, cols);
        }
    }

    Ok(scores)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_p1_example() {
        let result = p1(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 36);
    }

    #[test]
    fn test_p2_example() {
        let result = p2(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 81);
    }
}
