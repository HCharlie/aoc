use anyhow::{anyhow, Result};
use std::collections::{HashSet, HashMap};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    r: i32,
    c: i32,
}

#[derive(Debug, Clone)]
struct Shape {
    id: usize,
    points: Vec<Point>,
}

impl Shape {
    fn normalize(&mut self) {
        if self.points.is_empty() {
            return;
        }
        let min_r = self.points.iter().map(|p| p.r).min().unwrap();
        let min_c = self.points.iter().map(|p| p.c).min().unwrap();
        for p in &mut self.points {
            p.r -= min_r;
            p.c -= min_c;
        }
        self.points.sort_by(|a, b| a.r.cmp(&b.r).then(a.c.cmp(&b.c)));
    }

    fn rotate(&self) -> Shape {
        let mut new_points = Vec::with_capacity(self.points.len());
        for p in &self.points {
            new_points.push(Point { r: p.c, c: -p.r });
        }
        let mut s = Shape {
            id: self.id,
            points: new_points,
        };
        s.normalize();
        s
    }

    fn flip(&self) -> Shape {
        let mut new_points = Vec::with_capacity(self.points.len());
        for p in &self.points {
            new_points.push(Point { r: p.r, c: -p.c });
        }
        let mut s = Shape {
            id: self.id,
            points: new_points,
        };
        s.normalize();
        s
    }

    fn variations(&self) -> Vec<Shape> {
        let mut variations = Vec::new();
        let mut seen = HashSet::new();

        let mut current = self.clone();
        current.normalize();
        
        for _ in 0..4 {
            if !seen.contains(&current.points) {
                seen.insert(current.points.clone());
                variations.push(current.clone());
            }
            let flipped = current.flip();
            if !seen.contains(&flipped.points) {
                seen.insert(flipped.points.clone());
                variations.push(flipped);
            }
            current = current.rotate();
        }
        variations
    }
}

struct Problem {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}

struct Region {
    width: i32,
    height: i32,
    to_fit: Vec<usize>, 
    #[allow(dead_code)]
    original_line: String, 
}

fn parse_input(input: &str) -> Result<Problem> {
    let mut shapes = Vec::new();
    let mut regions = Vec::new();

    for chunk in input.split("\n\n") {
        let chunk = chunk.trim();
        if chunk.is_empty() { continue; }

        let first_line = chunk.lines().next().unwrap_or("");
        
        if first_line.contains('x') {
            // Regions block
            for line in chunk.lines() {
                let line = line.trim();
                if line.is_empty() { continue; }
                
                let (dims, counts_str) = line.split_once(": ").ok_or(anyhow!("Invalid region line"))?;
                let (w_str, h_str) = dims.split_once('x').ok_or(anyhow!("Invalid dims"))?;
                let width: i32 = w_str.parse()?;
                let height: i32 = h_str.parse()?;

                let counts: Vec<usize> = counts_str
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();

                let mut to_fit = Vec::new();
                for (shape_idx, &count) in counts.iter().enumerate() {
                    for _ in 0..count {
                        to_fit.push(shape_idx);
                    }
                }
                regions.push(Region { width, height, to_fit, original_line: line.to_string() });
            }
        } else {
            // Shape block
            let mut lines = chunk.lines();
            let header = lines.next().unwrap();
            let id = header.trim_end_matches(':').parse()?;
            
            let mut points = Vec::new();
            for (r, line) in lines.enumerate() {
                for (c, ch) in line.chars().enumerate() {
                    if ch == '#' {
                        points.push(Point { r: r as i32, c: c as i32 });
                    }
                }
            }
            let mut s = Shape { id, points };
            s.normalize();
            shapes.push(s);
        }
    }
    
    shapes.sort_by_key(|s| s.id);

    Ok(Problem { shapes, regions })
}

fn solve_region(region: &Region, shapes: &[Shape]) -> bool {
    // Basic optimization: Check total area first
    let presents_area: usize = region.to_fit.iter().map(|&id| shapes[id].points.len()).sum();
    let region_area = (region.width * region.height) as usize;
    if presents_area > region_area {
        return false;
    }

    let mut pieces_to_fit = region.to_fit.clone();
    // Sort logic: Larger pieces first. AND identical pieces adjacent.
    // Sorting by ID as secondary key keeps identical pieces together.
    pieces_to_fit.sort_by(|&a, &b| {
        let size_a = shapes[a].points.len();
        let size_b = shapes[b].points.len();
        // larger size first
        size_b.cmp(&size_a).then(a.cmp(&b))
    });

    let unique_ids: HashSet<usize> = pieces_to_fit.iter().cloned().collect();
    let mut variations_map = HashMap::new();
    for &id in &unique_ids {
        variations_map.insert(id, shapes[id].variations());
    }

    // Grid: flat vector
    let mut grid = vec![false; region_area];
    
    backtrack(&mut grid, &pieces_to_fit, 0, &variations_map, region.width, region.height, shapes, -1)
}

fn backtrack(
    grid: &mut Vec<bool>,
    pieces: &[usize],
    idx: usize,
    variations_map: &HashMap<usize, Vec<Shape>>,
    w: i32,
    h: i32,
    shapes: &[Shape], // needed for id check
    last_pos_for_same_shape: i32,
) -> bool {
    if idx >= pieces.len() {
        return true;
    }

    let shape_id = pieces[idx];
    let variations = &variations_map[&shape_id];
    
    // Symmetry breaking:
    let mut start_pos = 0;
    if idx > 0 && pieces[idx] == pieces[idx-1] {
        start_pos = last_pos_for_same_shape; 
    }

    let area = (w * h) as usize;

    for pos in (start_pos as usize)..area {
        let r = (pos as i32) / w;
        let c = (pos as i32) % w;

        for var in variations {
            if can_place_flat(grid, var, r, c, w, h) {
                place_flat(grid, var, r, c, w, true);
                
                if backtrack(grid, pieces, idx + 1, variations_map, w, h, shapes, pos as i32) {
                    return true;
                }
                place_flat(grid, var, r, c, w, false);
            }
        }
    }
    
    false
}

fn can_place_flat(grid: &[bool], shape: &Shape, r: i32, c: i32, w: i32, h: i32) -> bool {
    for p in &shape.points {
        let nr = r + p.r;
        let nc = c + p.c;
        if nr < 0 || nr >= h || nc < 0 || nc >= w {
            return false;
        }
        if grid[(nr * w + nc) as usize] {
            return false;
        }
    }
    true
}

fn place_flat(grid: &mut Vec<bool>, shape: &Shape, r: i32, c: i32, w: i32, val: bool) {
    for p in &shape.points {
        let nr = r + p.r;
        let nc = c + p.c;
        grid[(nr * w + nc) as usize] = val;
    }
}

pub fn p1(input_text: &str) -> Result<i64> {
    let problem = parse_input(input_text)?;
    let mut solved_count = 0;
    
    for region in &problem.regions {
        if solve_region(region, &problem.shapes) {
            solved_count += 1;
        }
    }
    
    Ok(solved_count)
}

pub fn p2(_input_text: &str) -> Result<i64> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_P1: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]
    fn test_p1_example() {
        let result = p1(EXAMPLE_INPUT_P1).unwrap();
        assert_eq!(result, 2);
    }
}
