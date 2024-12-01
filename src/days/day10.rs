use super::utils::{get_input_content, submit_check_answer};
use crate::Level;
use std::error::Error;
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

fn p1(input_text: &str) -> Result<i64, Box<dyn Error>> {
    let mut scores: i64 = 0;
    let grid: Vec<Vec<i64>> = input_text
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10)
                    .ok_or_else(|| format!("Invalid digit: {}", c))
                    .map(|d| d as i64))
                .collect::<Result<Vec<i64>, String>>()
        })
        .collect::<Result<Vec<Vec<i64>>, String>>()?;
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

fn p2(input_text: &str) -> Result<i64, Box<dyn Error>> {
    let mut scores: i64 = 0;
    let grid: Vec<Vec<i64>> = input_text
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10)
                    .ok_or_else(|| format!("Invalid digit: {}", c))
                    .map(|d| d as i64))
                .collect::<Result<Vec<i64>, String>>()
        })
        .collect::<Result<Vec<Vec<i64>>, String>>()?;
    let rows = grid.len();
    let cols = grid[0].len();

    for i in 0..rows {
        for j in 0..cols {
            scores += _helper_p2(&grid, i, j, rows, cols);
        }
    }
    

    Ok(scores)
}

pub fn run(day: u8, level: Level, debug: bool) -> () {
    let example_input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    let sol_func = match level {
        Level::One => p1,
        Level::Two => p2,
    };

    match sol_func(example_input) {
        Ok(result) => println!("Example result: {}", result),
        Err(e) => eprintln!("Error processing example: {}", e),
    }

    let content = match get_input_content(day) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            return;
        }
    };

    let answer = match sol_func(&content) {
        Ok(answer) => answer,
        Err(e) => {
            eprintln!("Error processing input: {}", e);
            return;
        }
    };

    if debug {
        println!("Answer: {}", answer);
        return ();
    }
    match submit_check_answer(day, level as u8, &answer.to_string()) {
        Ok(is_correct) => println!(
            "Answer {} is {}",
            answer,
            if is_correct { "correct" } else { "wrong" }
        ),
        Err(e) => {
            eprintln!("Error submitting answer: {}", e);
            return;
        }
    }
}
