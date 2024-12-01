use super::utils::{get_input_content, submit_check_answer};
use crate::Level;
use std::error::Error;
use std::collections::{HashSet, VecDeque};

fn p1(input_text: &str, grid_size: i64, bytes_falling: i64) -> Result<String, Box<dyn Error>> {
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; grid_size as usize]; grid_size as usize];
    let mut cnt = 0;
    for line in input_text.lines() {
        if cnt >= bytes_falling {
            break;
        }
        let (row, col ) = match line.split_once(",") {
            Some((row, col)) => {
                (row.parse::<i64>()?, col.parse::<i64>()?)
            }
            None => {
                return Err("Invalid input".into());
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
                return Err("invalid item in dq".into());
            }
        };

        if row == e_row && col == e_col {
            return Ok(dist.to_string());
        }
        
        for (d_row, d_col) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_row = row + d_row;
            let new_col = col + d_col;
            if new_row >= 0 && new_row < grid_size && new_col >= 0 && new_col < grid_size && grid[new_row as usize][new_col as usize] != '#' && !seen.contains(&(new_row, new_col)) {
                dq.push_back((new_row, new_col, dist + 1));
                seen.insert((new_row, new_col));
            }
        }


    }
    Err("No path found".into())
}


fn p2(input_text: &str, grid_size: i64, bytes_falling: i64) -> Result<String, Box<dyn Error>> {
    let total_lines = input_text.lines().count() as i64;
    println!("total_lines: {}", total_lines);
    let mut left = bytes_falling;
    let mut right = total_lines;

    while left < right {
        let mid = (left + right) / 2;
        match p1(input_text, grid_size, mid) {
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
    
    let line = input_text.lines().nth((left - 1) as usize).ok_or("Line not found")?;
    println!("line: {}", line);
    return Ok(line.to_string());
}

pub fn run(day: u8, level: Level, debug: bool) -> () {
    let example_input = 
"5,4
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
    let example_grid_size = 7;
    let example_bytes_falling = 12;
    let grid_size = 71;
    let bytes_falling = 1024;


    let sol_func = match level {
        Level::One => p1,
        Level::Two => p2,
    };

    match sol_func(example_input, example_grid_size, example_bytes_falling) {
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

    let answer = match sol_func(&content, grid_size, bytes_falling) {
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
