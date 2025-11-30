use anyhow::Result;
use std::collections::HashSet;

pub const EXAMPLE_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

fn _get_start_position(grid: &Vec<Vec<char>>) -> Result<(i32, i32)> {
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '^' {
                return Ok((i as i32, j as i32));
            }
        }
    }
    Err(anyhow::anyhow!("No start position found"))

}

fn _get_distinct_positions(grid: &Vec<Vec<char>>) -> Result<HashSet<(i32, i32)>> {
    let mut distinct_positions: HashSet<(i32, i32)> = HashSet::new();

    let rows = grid.len() as i32;
    let cols: i32 = grid[0].len() as i32;
    let (start_row, start_col) = _get_start_position(&grid)?;
    distinct_positions.insert((start_row, start_col));
    let mut direction = (-1, 0);
    let mut current_position = (start_row, start_col);

    while current_position.0 >= 0 && current_position.0 < rows && current_position.1 >= 0 && current_position.1 < cols {
        let next_position = (current_position.0 + direction.0, current_position.1 + direction.1);

        if next_position.0 < 0 || next_position.0 >= rows || next_position.1 < 0 || next_position.1 >= cols {
            break;
        }

        if grid[next_position.0 as usize][next_position.1 as usize] == '#' {
            direction = match direction {
                (0, -1) => (-1, 0),
                (1, 0) => (0, -1),
                (0, 1) => (1, 0),
                (-1, 0) => (0, 1),
                _ => direction,
            };
        } else {
            current_position = next_position;
            distinct_positions.insert(current_position);
        }
    }
    Ok(distinct_positions)
}

pub fn p1(input_text: &str) -> Result<i32> {
    
    let grid: Vec<Vec<char>> = input_text.lines().map(|l| l.chars().collect()).collect();
    
    let distinct_positions = _get_distinct_positions(&grid)?;

    Ok(distinct_positions.len() as i32)
}

fn _check_loop_exist(grid: &Vec<Vec<char>>, start_row: i32, start_col: i32) -> Result<bool> {
    let mut distinct_positions: HashSet<(i32, i32, i32, i32)> = HashSet::new();
    let rows = grid.len() as i32;
    let cols: i32 = grid[0].len() as i32;
    let mut direction = (-1, 0);
    let mut current_position = (start_row, start_col);
    distinct_positions.insert((start_row, start_col, direction.0, direction.1));
    while current_position.0 >= 0 && current_position.0 < rows && current_position.1 >= 0 && current_position.1 < cols {
        distinct_positions.insert((current_position.0, current_position.1, direction.0, direction.1));
        
        let next_position = (current_position.0 + direction.0, current_position.1 + direction.1);

        if next_position.0 < 0 || next_position.0 >= rows || next_position.1 < 0 || next_position.1 >= cols {
            break;
        }
        

        if grid[next_position.0 as usize][next_position.1 as usize] == '#' {
            direction = match direction {
                (0, -1) => (-1, 0),
                (1, 0) => (0, -1),
                (0, 1) => (1, 0),
                (-1, 0) => (0, 1),
                _ => direction,
            };
        } else {
            current_position = next_position;
            if distinct_positions.contains(&(current_position.0, current_position.1, direction.0, direction.1)) {
                return Ok(true);
            }
            
        }
    }



    Ok(false)
}

pub fn p2(input_text: &str) -> Result<i32> {
    let mut grid: Vec<Vec<char>> = input_text.lines().map(|l| l.chars().collect()).collect();
    let (start_row, start_col) = _get_start_position(&grid)?;

    let mut potential_obstacles_positions: i32 = 0;
    let distinct_positions = _get_distinct_positions(&grid)?;
    for &(i, j) in distinct_positions.iter() {
        if grid[i as usize][j as usize] != '.' {
            continue;
        }
        grid[i as usize][j as usize] = '#';
        if _check_loop_exist(&grid, start_row, start_col)? {
            potential_obstacles_positions += 1;
        }
        grid[i as usize][j as usize] = '.';

    }

    Ok(potential_obstacles_positions)
}

