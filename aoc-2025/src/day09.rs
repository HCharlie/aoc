use std::collections::{BinaryHeap, HashMap};

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

pub fn p2(input_text: &str) -> Result<i64> {
    let total = 0;
    Ok(total)
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
