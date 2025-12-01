use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn p1(input_text: &str) -> Result<String> {
    let grid = input_text.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    
    let n_rows = grid.len();
    let n_cols = grid[0].len();
    let mut cnt_dots = 0;
    let mut start = (0, 0);
    let mut end = (0, 0);
    for row in 0..n_rows {
        for col in 0..n_cols {
            if grid[row][col] == '.' {
                cnt_dots += 1;
            } else if grid[row][col] == 'S' {
                start = (row, col);
            } else if grid[row][col] == 'E' {
                end = (row, col);
            }
        }
    }
    println!("cnt_dots: {}", cnt_dots);
    // let mut original_steps = cnt_dots + 1;
    // println!("original_steps: {}", original_steps);
    let mut dq = VecDeque::new();
    dq.push_back((start.0 as i32, start.1 as i32, 0));
    let mut seen = HashSet::new();
    let mut pos_map = HashMap::new();
    pos_map.insert((start.0 as i32, start.1 as i32), 0 as i32);
    seen.insert(start);
    while let Some((r, c, d)) = dq.pop_front() {
        pos_map.insert((r, c), d);
        if (r as usize, c as usize) == end {
            // original_steps = d;
            println!("Found end at distance: {}", d);
        }
        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let new_r = r + dr;
            let new_c = c + dc;
            if new_r >= 0 && new_r < n_rows as i32 && new_c >= 0 && new_c < n_cols as i32 && grid[new_r as usize][new_c as usize] != '#' && !seen.contains(&(new_r as usize, new_c as usize)) {
                seen.insert((new_r as usize, new_c as usize));
                dq.push_back((new_r, new_c, d + 1));
            }
        }
    }
    // println!("original_steps: {}", original_steps);

    // println!("pos_map {:?}", pos_map);
    println!("len pos_map {}", pos_map.len());

    let mut dq_new = VecDeque::new();
    let mut seen_new = HashSet::new();
    let mut saved_steps = HashMap::new();

    dq_new.push_back((start.0 as i32, start.1 as i32, 0));
    seen_new.insert(start);

    let mut cnt = 0;

    while let Some((r, c, d)) = dq_new.pop_front() {
        if (r as usize, c as usize) == end {
            println!("Found end at distance: {}", d);
            break;
        }
        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let new_r = r + dr;
            let new_c = c + dc;
            if new_r >= 0 && new_r < n_rows as i32 && new_c >= 0 && new_c < n_cols as i32 && grid[new_r as usize][new_c as usize] != '#' && !seen_new.contains(&(new_r as usize, new_c as usize)) {
                seen_new.insert((new_r as usize, new_c as usize));
                dq_new.push_back((new_r, new_c, d + 1));
            }
            if new_r >= 0 && new_r < n_rows as i32 && new_c >= 0 && new_c < n_cols as i32 && grid[new_r as usize][new_c as usize] == '#' {
                let nnew_r = new_r + dr;
                let nnew_c = new_c + dc;
                if nnew_r >= 0 && nnew_r < n_rows as i32 && nnew_c >= 0 && nnew_c < n_cols as i32 && grid[nnew_r as usize][nnew_c as usize] != '#' {
                    let k = pos_map.get(&(nnew_r, nnew_c)).unwrap() - pos_map.get(&(r, c)).unwrap() - 2;
                    if k > 0 {
                        *saved_steps.entry(k).or_insert(0) += 1;
                    }
                    // println!("k: {}", k);
                    if k >= 100 {
                        cnt += 1
                    }
                    
                }

            }
        }
    }
    println!("saved_steps {:?}", saved_steps);
    Ok(cnt.to_string())
}




pub fn p2(input_text: &str) -> Result<String> {
    let grid = input_text.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    
    let n_rows = grid.len();
    let n_cols = grid[0].len();
    let mut cnt_dots = 0;
    let mut start = (0, 0);
    let mut end = (0, 0);
    for row in 0..n_rows {
        for col in 0..n_cols {
            if grid[row][col] == '.' {
                cnt_dots += 1;
            } else if grid[row][col] == 'S' {
                start = (row, col);
            } else if grid[row][col] == 'E' {
                end = (row, col);
            }
        }
    }
    println!("cnt_dots: {}", cnt_dots);
    // let mut original_steps = cnt_dots + 1;
    // println!("original_steps: {}", original_steps);
    let mut dq = VecDeque::new();
    dq.push_back((start.0 as i32, start.1 as i32, 0));
    let mut seen = HashSet::new();
    let mut pos_map = HashMap::new();
    pos_map.insert((start.0 as i32, start.1 as i32), 0 as i32);
    seen.insert(start);
    while let Some((r, c, d)) = dq.pop_front() {
        pos_map.insert((r, c), d);
        if (r as usize, c as usize) == end {
            // original_steps = d;
            println!("Found end at distance: {}", d);
        }
        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let new_r = r + dr;
            let new_c = c + dc;
            if new_r >= 0 && new_r < n_rows as i32 && new_c >= 0 && new_c < n_cols as i32 && grid[new_r as usize][new_c as usize] != '#' && !seen.contains(&(new_r as usize, new_c as usize)) {
                seen.insert((new_r as usize, new_c as usize));
                dq.push_back((new_r, new_c, d + 1));
            }
        }
    }
    // println!("original_steps: {}", original_steps);

    // println!("pos_map {:?}", pos_map);
    println!("len pos_map {}", pos_map.len());

    let mut save_steps = HashMap::new();
    let mut seen_cheat = HashSet::new();
    let mut cnt = 0;
    for r in 0..n_rows {
        for c in 0..n_cols {
            if grid[r][c].to_string() == "#" {
                continue;
            }
            let r = r as i32;
            let c = c as i32;
            for radius in 2..21 {
                for dr in 0..radius+1 {
                    let dc = radius - dr;
                    for &(nr, nc) in [(r+dr, c+dc), (r+dr, c-dc),(r-dr, c+dc), (r-dr, c-dc)].iter() {
                        if nr < 0 || nc < 0 || nr >= n_rows as i32 || nc >= n_cols as i32 {
                            continue;
                        }

                        
                        if grid[nr as usize][nc as usize].to_string() == "#" {
                            continue;
                        }
                        // println!("new pos {:?}", pos_map.get(&(nr, nc)).unwrap());
                        // println!("origin pos {:?}", pos_map.get(&(r, c)).unwrap());
                        if seen_cheat.contains(&(nr, nc, r, c)) {
                            continue
                        }
                        seen_cheat.insert((nr, nc, r, c));
                        let k = pos_map.get(&(nr, nc)).unwrap() - pos_map.get(&(r, c)).unwrap() - radius;
                        // println!("k: {:?}", k);
                        if k > 0 {
                            *save_steps.entry(k).or_insert(0) += 1;
                        }
                        if k >= 100 {
                            cnt += 1
                        }
                    }
                    
                }
            }
        }
    }
    for (k, v )in save_steps {
        if k >= 50 {
            println!("k: {}, v: {}", k, v)
        }
    }

    
    // println!("saved_steps {:?}", saved_steps);
    Ok(cnt.to_string())

}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_p1_example() {
        // p1 with example returns 0 because the threshold is >= 100
        let result = p1(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_p2_example() {
        // p2 with example returns 0 because the threshold is >= 100
        let result = p2(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, "0");
    }
}

