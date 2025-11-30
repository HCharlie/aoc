use anyhow::Result;
pub const EXAMPLE_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
fn count_xmas(grid: &Vec<Vec<char>>, start_row: usize, start_col: usize) -> Result<i32> {
    let directions = [
        (0, 1),   // Horizontal right
        (1, 0),   // Vertical down
        (1, 1),   // Diagonal down-right
        (1, -1),  // Diagonal down-left
        (0, -1),  // Horizontal left
        (-1, 0),  // Vertical up
        (-1, -1), // Diagonal up-left
        (-1, 1),  // Diagonal up-right
    ];

    let word = "XMAS";
    let word_len = word.len();
    let mut count = 0;

    for &(dr, dc) in &directions {
        let mut found = true;
        for i in 0..word_len {
            let new_row = start_row as isize + dr * i as isize;
            let new_col = start_col as isize + dc * i as isize;

            if new_row < 0
                || new_row >= grid.len() as isize
                || new_col < 0
                || new_col >= grid[0].len() as isize
                || grid[new_row as usize][new_col as usize] != word.as_bytes()[i] as char
            {
                found = false;
                break;
            }
        }
        if found {
            count += 1;
        }
    }

    Ok(count)
}

fn count_x_mas(grid: &Vec<Vec<char>>, start_row: usize, start_col: usize) -> i32 {
    let directions = [
        (1, 1),
        (1, -1),
    ];
    let mut found = true;

    for &(dr, dc) in &directions {
        let new_row = start_row as isize + dr as isize;
        let new_col = start_col as isize + dc as isize;
        let new_row_opposite = start_row as isize - dr as isize;
        let new_col_opposite = start_col as isize - dc as isize;

        if new_row < 0
            || new_row >= grid.len() as isize
            || new_col < 0
            || new_col >= grid[0].len() as isize
            || new_row_opposite < 0
            || new_row_opposite >= grid.len() as isize
            || new_col_opposite < 0
            || new_col_opposite >= grid[0].len() as isize
        {
            found = false;
            break
        }

        if !matches!(grid[new_row as usize][new_col as usize], 'M' | 'S') || !matches!(grid[new_row_opposite as usize][new_col_opposite as usize], 'M' | 'S') {
            found = false;
            break;
        }
        if grid[new_row as usize][new_col as usize] == 'M' {
            if grid[new_row_opposite as usize][new_col_opposite as usize] != 'S' {
                found = false;
                break
            }
        }  
        if grid[new_row as usize][new_col as usize] == 'S' && grid[new_row_opposite as usize][new_col_opposite as usize] != 'M' {
            found = false;
            break
        }
    }

    if found { 1 } else { 0 }
}


pub fn p1(input_text: &str) -> Result<i32> {
    
    let mut sum = 0;

    let lines: Vec<&str> = input_text.lines().collect();
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }

    // Now you can iterate over rows and columns
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &col) in row.iter().enumerate() {
            if col == 'X' {
                sum += count_xmas(&grid, row_idx, col_idx)?;
            }
        }
    }
    Ok(sum)
}

pub fn p2(input_text: &str) -> Result<i32> {
    
    let mut sum = 0;

    let lines: Vec<&str> = input_text.lines().collect();
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }

    // Now you can iterate over rows and columns
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &col) in row.iter().enumerate() {
            if col == 'A' {
                sum += count_x_mas(&grid, row_idx, col_idx);
            }
        }
    }
    Ok(sum)
}

