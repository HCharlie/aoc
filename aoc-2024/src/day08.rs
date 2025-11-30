use anyhow::Result;
use std::collections::HashMap;
use std::collections::HashSet;

pub const EXAMPLE_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";


fn _get_hmap(grid: &Vec<Vec<char>>) -> HashMap<char, Vec<(usize, usize)>> {
    let mut hmap: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c != '.' {
                let entry = hmap.entry(*c).or_insert(Vec::new());
                entry.push((i, j));
            }
        }
    }

    hmap
}

fn _get_unique_antinode_per_frequency_p1(antennas: &Vec<(usize, usize)>, n_rows: usize, n_cols: usize) -> Result<HashSet<(usize, usize)>> {
    let mut unique_locations: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..antennas.len() - 1 {
        for j in i+1..antennas.len() {
            let (row1, col1) = (antennas[i].0 as i32, antennas[i].1 as i32);
            let (row2, col2) = (antennas[j].0 as i32, antennas[j].1 as i32);

            let antinode_1 = (row1*2 - row2, col1*2 - col2);
            if antinode_1.0 >= 0 && antinode_1.0 < (n_rows as i32) && antinode_1.1 >= 0 && antinode_1.1 < (n_cols as i32) {
                unique_locations.insert((antinode_1.0 as usize, antinode_1.1 as usize));
            }
            let antinode_2 = (row2*2 - row1, col2*2 - col1);
            if antinode_2.0 >= 0 && antinode_2.0 < (n_rows as i32) && antinode_2.1 >= 0 && antinode_2.1 < (n_cols as i32) {
                unique_locations.insert((antinode_2.0 as usize, antinode_2.1 as usize));
            }
        }
    }

    Ok(unique_locations)

}

fn _get_unique_antinode_per_frequency_p2(antennas: &Vec<(usize, usize)>, n_rows: usize, n_cols: usize) -> Result<HashSet<(usize, usize)>> {
    let mut unique_locations: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..antennas.len() - 1 {
        for j in i+1..antennas.len() {
            let (row1, col1) = (antennas[i].0 as i32, antennas[i].1 as i32);
            let (row2, col2) = (antennas[j].0 as i32, antennas[j].1 as i32);

            // Get direction vector between antennas
            let delta_row = row2 - row1;
            let delta_col = col2 - col1;

            // Check points in both directions from both antennas
            for multiplier in -(n_rows as i32)..=(n_rows as i32) {
                let antinode = (
                    row1 + delta_row * multiplier,
                    col1 + delta_col * multiplier
                );
                
                // Check if point is within grid bounds
                if antinode.0 >= 0 && antinode.0 < (n_rows as i32) && 
                   antinode.1 >= 0 && antinode.1 < (n_cols as i32) {

                    unique_locations.insert((antinode.0 as usize, antinode.1 as usize));

                }
            }
        }
    }

    Ok(unique_locations)

}

pub fn p1(input_text: &str) -> Result<i32> {
    let mut unique_locations: HashSet<(usize, usize)> = HashSet::new();
    let grid: Vec<Vec<char>> = input_text.lines().map(|l| l.chars().collect()).collect();
    let n_rows = grid.len();
    let n_cols = grid[0].len();
    let hmap = _get_hmap(&grid);
    for (_, antennas) in hmap.iter() {
        let unique_antennas = _get_unique_antinode_per_frequency_p1(antennas, n_rows, n_cols)?;
        unique_locations.extend(&unique_antennas);
    }

    Ok(unique_locations.len() as i32)
}


pub fn p2(input_text: &str) -> Result<i32> {
    let mut unique_locations: HashSet<(usize, usize)> = HashSet::new();
    let grid: Vec<Vec<char>> = input_text.lines().map(|l| l.chars().collect()).collect();
    let n_rows = grid.len();
    let n_cols = grid[0].len();
    let hmap = _get_hmap(&grid);
    for (_, antennas) in hmap.iter() {
        let unique_antennas = _get_unique_antinode_per_frequency_p2(antennas, n_rows, n_cols)?;
        unique_locations.extend(&unique_antennas);
    }

    Ok(unique_locations.len() as i32)
}

