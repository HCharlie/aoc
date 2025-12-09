use anyhow::Result;

fn rectangle_area(p1: (i64, i64), p2: (i64, i64)) -> i64 {
    let width = (p1.0 - p2.0).abs() + 1;
    let height = (p1.1 - p2.1).abs() + 1;
    width * height
}


pub fn p1(input_text: &str) -> Result<i64> {
    let mut largest_rectangle_area = 0;
    let points: Vec<(i64, i64)> = input_text
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x: i64 = parts.next().unwrap().parse().unwrap();
            let y: i64 = parts.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect();
    // println!("Points: {:?}", points);
    
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let area = rectangle_area(points[i], points[j]);
            if area > largest_rectangle_area {
                largest_rectangle_area = area;
            }
        }
    }
    Ok(largest_rectangle_area)
}

fn rect_line_intersects(
    rect_a: (i64, i64),
    rect_b: (i64, i64),
    line_start: (i64, i64),
    line_end: (i64, i64),
) -> bool {
    // Check if rectangle and line segment intersect
    // They intersect if they're NOT completely separated
    let left_of_rect = rect_a.0.max(rect_b.0) <= line_start.0.min(line_end.0);
    let right_of_rect = rect_a.0.min(rect_b.0) >= line_start.0.max(line_end.0);
    let above = rect_a.1.max(rect_b.1) <= line_start.1.min(line_end.1);
    let below = rect_a.1.min(rect_b.1) >= line_start.1.max(line_end.1);
    
    // If none of the separation conditions are true, they intersect
    !(left_of_rect || right_of_rect || above || below)
}

pub fn p2(input_text: &str) -> Result<i64> {
    let points: Vec<(i64, i64)> = input_text
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x: i64 = parts.next().unwrap().parse().unwrap();
            let y: i64 = parts.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect();
    
    // Create polygon edges (each point connects to the next, wrapping around)
    let mut edges = Vec::new();
    for i in 0..points.len() {
        let next_i = (i + 1) % points.len();
        edges.push((points[i], points[next_i]));
    }
    
    // Find all possible rectangles and their areas
    let mut candidates = Vec::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let area = rectangle_area(points[i], points[j]);
            candidates.push((points[i], points[j], area));
        }
    }
    
    // Sort by area (largest first)
    candidates.sort_by_key(|&(_, _, area)| std::cmp::Reverse(area));
    
    // Find the largest rectangle that doesn't intersect with any polygon edge
    for (a, b, area) in candidates {
        let mut valid = true;
        for &(line_start, line_end) in &edges {
            if rect_line_intersects(a, b, line_start, line_end) {
                valid = false;
                break;
            }
        }
        if valid {
            return Ok(area);
        }
    }
    
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_p1_example() {
        let result = p1(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 50);
    }

    #[test]
    fn test_p2_example() {
        let result = p2(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 24);
    }
}
