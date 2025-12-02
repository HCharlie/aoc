use anyhow::Result;
use std::collections::HashSet;
use std::collections::VecDeque;

use ordered_float::OrderedFloat;

fn _calculate_region(
    grid: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    visited: &mut HashSet<(usize, usize)>,
) -> i64 {
    let n_rows = grid.len();
    let n_cols = grid[0].len();
    let mut perimeter = 0;
    let mut area = 0;
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut queue = VecDeque::new();
    queue.push_back((row, col));

    visited.insert((row, col));

    while let Some((r, c)) = queue.pop_front() {
        area += 1;
        for (dr, dc) in directions.iter() {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr >= 0 && nr < n_rows as isize && nc >= 0 && nc < n_cols as isize {
                let nr = nr as usize;
                let nc = nc as usize;
                if grid[nr][nc] == grid[row][col] && !visited.contains(&(nr, nc)) {
                    visited.insert((nr, nc));
                    queue.push_back((nr, nc));
                } else if grid[nr][nc] != grid[row][col] {
                    perimeter += 1;
                }
            } else {
                perimeter += 1;
            }
        }
    }

    perimeter * area
}

pub fn p1(input_text: &str) -> Result<i64> {
    let grid: Vec<Vec<char>> = input_text.lines().map(|l| l.chars().collect()).collect();
    let n_rows = grid.len();
    let n_cols = grid[0].len();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut price = 0;

    for row in 0..n_rows {
        for col in 0..n_cols {
            if visited.contains(&(row, col)) {
                continue;
            }
            price += _calculate_region(&grid, row, col, &mut visited);
        }
    }

    Ok(price)
}

fn _calculate_score_for_region(region: &HashSet<(usize, usize)>) -> i64 {
    let corner_candidates = collect_corner_candidates(region);
    calculate_total_corners(&corner_candidates, region)
}

fn collect_corner_candidates(
    region: &HashSet<(usize, usize)>,
) -> HashSet<(OrderedFloat<f64>, OrderedFloat<f64>)> {
    let mut candidates = HashSet::new();

    for &(r, c) in region {
        let corner_points = [
            (r as f64 - 0.5, c as f64 - 0.5),
            (r as f64 + 0.5, c as f64 - 0.5),
            (r as f64 + 0.5, c as f64 + 0.5),
            (r as f64 - 0.5, c as f64 + 0.5),
        ];
        candidates.extend(
            corner_points
                .iter()
                .map(|&(cr, cc)| (OrderedFloat(cr), OrderedFloat(cc))),
        );
    }

    candidates
}

fn calculate_total_corners(
    corner_candidates: &HashSet<(OrderedFloat<f64>, OrderedFloat<f64>)>,
    region: &HashSet<(usize, usize)>,
) -> i64 {
    let mut total_corners = 0;

    for &(cr, cc) in corner_candidates {
        let adjacent_cells = get_adjacent_cells_state(cr.0, cc.0, region);
        total_corners += count_corners_for_configuration(&adjacent_cells);
    }

    total_corners
}

fn get_adjacent_cells_state(cr: f64, cc: f64, region: &HashSet<(usize, usize)>) -> [bool; 4] {
    [
        is_cell_in_region(cr - 0.5, cc - 0.5, region),
        is_cell_in_region(cr + 0.5, cc - 0.5, region),
        is_cell_in_region(cr + 0.5, cc + 0.5, region),
        is_cell_in_region(cr - 0.5, cc + 0.5, region),
    ]
}

fn is_cell_in_region(r: f64, c: f64, region: &HashSet<(usize, usize)>) -> bool {
    if r < 0.0 || c < 0.0 {
        return false;
    }
    region.contains(&(r as usize, c as usize))
}

fn count_corners_for_configuration(config: &[bool; 4]) -> i64 {
    match config.iter().filter(|&&x| x).count() {
        1 => 1,
        2 if *config == [true, false, true, false] || *config == [false, true, false, true] => 2,
        3 => 1,
        _ => 0,
    }
}

pub fn p2(input_text: &str) -> Result<i64> {
    let grid: Vec<Vec<char>> = input_text.lines().map(|l| l.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut regions = Vec::new();
    let mut seen = HashSet::new();

    for r in 0..rows {
        for c in 0..cols {
            if seen.contains(&(r, c)) {
                continue;
            }
            seen.insert((r, c));
            let mut region = HashSet::new();
            region.insert((r, c));
            let mut q = VecDeque::new();
            q.push_back((r, c));
            let crop = grid[r][c];

            while let Some((cr, cc)) = q.pop_front() {
                for (nr, nc) in [
                    (cr.wrapping_sub(1), cc),
                    (cr + 1, cc),
                    (cr, cc.wrapping_sub(1)),
                    (cr, cc + 1),
                ] {
                    if nr >= rows || nc >= cols {
                        continue;
                    }
                    if grid[nr][nc] != crop {
                        continue;
                    }
                    if region.contains(&(nr, nc)) {
                        continue;
                    }
                    region.insert((nr, nc));
                    q.push_back((nr, nc));
                }
            }
            seen.extend(&region);
            regions.push(region);
        }
    }

    Ok(regions
        .iter()
        .map(|region| _calculate_score_for_region(region) * region.len() as i64)
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_p1_example() {
        let result = p1(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 1930);
    }

    #[test]
    fn test_p2_example() {
        let result = p2(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 1206);
    }
}
