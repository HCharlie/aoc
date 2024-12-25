use super::utils::{get_input_content, submit_check_answer};
use crate::Level;
use std::{error::Error, vec};

fn _convert_grid(grid: &Vec<Vec<char>>) -> Vec<i64> {
    let cols = grid[0].len();
    let mut lock = vec![-1; cols];
    for row in grid {
        for (i, &cell) in row.iter().enumerate() {
            if cell == '#' {
                lock[i] += 1;
            }
        }
    }
    lock
}

fn _get_size(grids: &Vec<&str>) -> (usize, usize) {
    let m: Vec<Vec<char>> = grids[0].lines().map(|l| l.chars().collect()).collect();
    (m.len(), m[0].len())
}


fn p1(input_text: &str) -> Result<String, Box<dyn Error>> {
    let grids: Vec<&str> = input_text.split("\n\n").collect();
    let mut locks: Vec<Vec<i64>> = Vec::new();
    let mut keys: Vec<Vec<i64>> = Vec::new();
    for &grid in grids.iter() {
        let m: Vec<Vec<char>> = grid.lines().map(|l| l.chars().collect()).collect();
        if m[0][0] == '#' {
            locks.push(_convert_grid(&m));
        } else {
            keys.push(_convert_grid(&m));
        }
    }
    let (rows, cols) = _get_size(&grids);
    println!("rows: {}, cols: {}", rows, cols);
    let mut fits = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            let mut flag = true;
            for col in 0..cols {
                if key[col] + lock[col] > rows as i64 - 2  {
                    flag = false;
                    break;
                }
            }
            if flag {
                fits += 1;
            }
        }
    }
    Ok(fits.to_string())
}



fn p2(input_text: &str) -> Result<String, Box<dyn Error>> {
    Err("Not implemented".into())
}

pub fn run(day: u8, level: Level, debug: bool) -> () {
    let example_input = 
"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";



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
