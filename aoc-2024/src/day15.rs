use anyhow::Result;
use std::collections::{HashSet, VecDeque};

fn _bot_pos(grid: &Vec<Vec<char>>) -> (i64, i64) {
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '@' {
                return (i as i64, j as i64);
            }
        }
    }
    (0, 0)
}

fn _cal(grid: &Vec<Vec<char>>) -> i64 {
    let mut sum: i64 = 0;
    let rows = grid.len();
    let cols = grid[0].len();
    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == 'O' {
                sum += (100*row + col) as i64;
            }
        }
    }
    sum    
}

pub fn p1(input_text: &str) -> Result<i64> {
    let parts: Vec<&str> = input_text.split("\n\n").collect();
    let (grid_str, directions) = (parts[0], parts[1]);
    let mut grid: Vec<Vec<char>> = grid_str.lines().map(|line| line.chars().collect()).collect();
    let directions: Vec<char> = directions.chars().filter(|c| *c != '\n').collect();
    let (mut bx, mut by) = _bot_pos(&grid);
    for d in directions {
        let (dx, dy) = match d {
            '^' => (-1, 0),
            'v' => (1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            _ => panic!("Invalid direction"),
        };
        let (nx, ny) = (bx + dx, by + dy);
        // next is wall
        if grid[nx as usize][ny as usize] == '#' {
            continue;
        }
        // next is box
        if grid[nx as usize][ny as usize] == 'O' {
            let mut check_x = nx + dx;
            let mut check_y = ny + dy;
            while grid[check_x as usize][check_y as usize] == 'O' {
                check_x += dx;
                check_y += dy;
            }
            if grid[check_x as usize][check_y as usize] == '.' {
                grid[check_x as usize][check_y as usize] = 'O';
                grid[nx as usize][ny as usize] = '@';
                grid[bx as usize][by as usize] = '.';
                // update bot position
                bx = nx;
                by = ny;
            }
        }
        // next is space
        if grid[nx as usize][ny as usize] == '.' {
            // move the bot
            grid[nx as usize][ny as usize] = '@';
            grid[bx as usize][by as usize] = '.';
            // update bot position
            bx = nx;
            by = ny;
        }
    }
    Ok(_cal(&grid))
}

fn _cal_p2(grid: &Vec<Vec<char>>) -> i64 {
    let mut sum: i64 = 0;
    let rows = grid.len();
    let cols = grid[0].len();
    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == '[' {
                sum += (100*row + col) as i64;
            }
        }
    }
    sum    
}

pub fn p2(input_text: &str) -> Result<i64> {
    let parts: Vec<&str> = input_text.split("\n\n").collect();
    let (grid_str, directions) = (parts[0], parts[1]);
    let grid_temp: Vec<Vec<char>> = grid_str.lines().map(|line| line.chars().collect()).collect();
    let mut grid: Vec<Vec<char>> = Vec::new();
    for row in grid_temp {
        let mut new_row = Vec::new();
        for c in row {
            match c {
                '#' => { new_row.push('#'); new_row.push('#'); }
                'O' => { new_row.push('['); new_row.push(']'); }
                '.' => { new_row.push('.'); new_row.push('.'); }
                '@' => { new_row.push('@'); new_row.push('.'); }
                _ => panic!("Invalid character in grid"),
            }
        }
        grid.push(new_row);
    }
    let moves: Vec<char> = directions.chars().filter(|c| *c != '\n').collect();
    let (mut br, mut bc) = _bot_pos(&grid);
    println!("bot pos: ({}, {})", br, bc);
    for row in &grid {
        println!("{}", row.iter().collect::<String>());
    }
    for mv in moves {
        println!("mv: {}", mv);
        let (dr, dc) = match mv {
            '^' => (-1, 0),
            'v' => (1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            _ => panic!("Invalid direction"),
        };
        let mut visited: HashSet<(i64, i64)> = HashSet::new();
        visited.insert((br, bc));

        let mut next_layber_box: VecDeque<(i64, i64)> = VecDeque::new();
        
        next_layber_box.push_back((br, bc));
        let mut go = true;

        while !next_layber_box.is_empty() {
            let mut candidates: VecDeque<(i64, i64)> = VecDeque::new();

            while !next_layber_box.is_empty() {
                let (cr, cc) = match next_layber_box.pop_front() {
                    Some(pos) => pos,
                    None => anyhow::bail!("Error popping from next_layber_box"),
                };
                let (nr, nc) = (cr + dr, cc + dc);
                match grid[nr as usize][nc as usize] {
                    '#' => {
                        go = false;
                        candidates.clear();
                        break;
                    },
                    '[' => {
                        if !visited.contains(&(nr, nc)) {
                            candidates.push_back((nr, nc));
                            visited.insert((nr, nc));
                        }
                        if !visited.contains(&(nr, nc+1)) {
                            candidates.push_back((nr, nc+1));
                            visited.insert((nr, nc+1));
                        }
                    },
                    ']' => {
                        if !visited.contains(&(nr, nc)) {
                            candidates.push_back((nr, nc));
                            visited.insert((nr, nc));
                        }
                        if !visited.contains(&(nr, nc-1)) {
                            candidates.push_back((nr, nc-1));
                            visited.insert((nr, nc-1));
                        }
                    },
                    '.' => {
                        continue;
                    },
                    _ => {
                        panic!("Invalid character in grid");
                    },
                }

            }

            next_layber_box = candidates;

        }

        if !go {
            continue;
        }

        let copy: Vec<Vec<char>> = grid.clone();
        
        for &(tr, tc) in visited.iter() {
            grid[tr as usize][tc as usize] = '.';
        }
        for &(tr, tc) in visited.iter() {
            grid[(tr + dr) as usize][(tc + dc) as usize] = copy[tr as usize][tc as usize];
        }
        
        br += dr;
        bc += dc;
    }

    Ok(_cal_p2(&grid))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn test_p1_example() {
        let result = p1(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 10092);
    }

    #[test]
    fn test_p2_example() {
        let result = p2(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 9021);
    }
}

