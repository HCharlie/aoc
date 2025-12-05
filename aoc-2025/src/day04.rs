use anyhow::Result;

pub fn check_surroudings(grid: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let mut rolls_of_paper = 0;
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for (row_offset, col_offset) in directions.iter() {
        let new_row = row as isize + row_offset;
        let new_col = col as isize + col_offset;
        if new_row >= 0
            && new_row < grid.len() as isize
            && new_col >= 0
            && new_col < grid[0].len() as isize
        {
            if grid[new_row as usize][new_col as usize] == '@' {
                rolls_of_paper += 1;
            }
        }
    }

    rolls_of_paper < 4
}

pub fn update_surroudings(grid: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let mut rolls_of_paper = 0;
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for (row_offset, col_offset) in directions.iter() {
        let new_row = row as isize + row_offset;
        let new_col = col as isize + col_offset;
        if new_row >= 0
            && new_row < grid.len() as isize
            && new_col >= 0
            && new_col < grid[0].len() as isize
        {
            if grid[new_row as usize][new_col as usize] != '.' {
                rolls_of_paper += 1;
            }
        }
    }

    rolls_of_paper < 4
}

pub fn p1(input_text: &str) -> Result<i64> {
    let mut total: i64 = 0;

    let grid: Vec<Vec<char>> = input_text
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == '@' && check_surroudings(&grid, row, col) {
                total += 1;
            }
        }
    }
    println!("Total: {}", total);
    Ok(total)
}

pub fn p2(input_text: &str) -> Result<i64> {
    let mut total: i64 = 0;
    let mut grid: Vec<Vec<char>> = input_text
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();
    let mut scan = true;
    while scan {
        scan = false;
        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                if grid[row][col] == '@' && update_surroudings(&grid, row, col) {
                    total += 1;
                    grid[row][col] = 'x'; // Mark cell as processed
                    scan = true;
                }
            }
        }
        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                if grid[row][col] == 'x' {
                    grid[row][col] = '.'; // Reset marked cells
                }
            }
        }
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_p1_example() {
        let result = p1(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 13);
    }

    #[test]
    fn test_p2_example() {
        let result = p2(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 43);
    }
}
