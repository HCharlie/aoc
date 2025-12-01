use anyhow::Result;
use std::vec;

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


pub fn p1(input_text: &str) -> Result<String> {
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



pub fn p2(_input_text: &str) -> Result<String> {
    Err(anyhow::anyhow!("Not implemented"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "#####
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

    #[test]
    fn test_p1_example() {
        let result = p1(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, "3");
    }

    #[test]
    fn test_p2_example() {
        // p2 not implemented for day 25
        let result = p2(EXAMPLE_INPUT);
        assert!(result.is_err());
    }
}

