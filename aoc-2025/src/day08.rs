use std::collections::{BinaryHeap, HashSet};

use anyhow::Result;

fn squared_euclidean_distance(p1: (i64, i64, i64), p2: (i64, i64, i64)) -> i64 {
    (p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2) + (p1.2 - p2.2).pow(2)
}

pub fn p1(input_text: &str) -> Result<i64> {
    let mut points = Vec::new();
    let mut n_connections = 0;

    
    for line in input_text.lines() {
        if line.is_empty() {
            continue;
        }
        if line.contains("NUMBERS: ") {
            n_connections = line.split(' ').nth(1).unwrap().parse::<i64>().unwrap();
            continue;
        }
        let parts: Vec<i64> = line.split(',').map(|x| x.parse::<i64>().unwrap()).collect();
        points.push((parts[0], parts[1], parts[2]));
    }
    println!("connections: {}", n_connections);
    println!("number of points: {}", points.len());
    
    let mut shortest_distances = BinaryHeap::new();
    for i in 0..points.len() {
        for j in i+1..points.len() {
            let distance = squared_euclidean_distance(points[i], points[j]);
            if shortest_distances.len() < n_connections as usize {
                shortest_distances.push((distance, i, j));
            } else {
                if distance < shortest_distances.peek().unwrap().0 {
                    shortest_distances.pop();
                    shortest_distances.push((distance, i, j));
                }
            }
        }
    }
    // println!("shortest_distances: {:?}", shortest_distances);

    // connect the points with the shortest distances to form a circuit, and count the number of circuits
    let mut connections = Vec::new();
    while !shortest_distances.is_empty() {
        let (_, i, j) = shortest_distances.pop().unwrap();
        connections.push((i, j));
    }
    // println!("connections: {:?}", connections);

    let mut circuit_list: Vec<HashSet<i64>> = Vec::new();
    for i in 0..connections.len() {
        let mut found = false;
        for circuit in &mut circuit_list {
            if circuit.contains(&(connections[i].0 as i64)) || circuit.contains(&(connections[i].1 as i64)) {
                circuit.insert(connections[i].0 as i64);
                circuit.insert(connections[i].1 as i64);
                found = true;
                break;
            }
        }
        if !found {
            circuit_list.push(HashSet::from([connections[i].0 as i64, connections[i].1 as i64]));
        }
    }
    // println!("circuit_list: {:?}", circuit_list);

    circuit_list.sort_by_key(|circuit| circuit.len());
    let largest_3_circuits = circuit_list.iter().rev().take(3).collect::<Vec<&HashSet<i64>>>();
    let product = largest_3_circuits.iter().map(|circuit| circuit.len() as i64).product();
    Ok(product)
}

pub fn p2(input_text: &str) -> Result<i64> {
    let mut total: i64 = 0;

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
NUMBERS: 10";

    #[test]
    fn test_p1_example() {
        let result = p1(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 40);
    }

    #[test]
    fn test_p2_example() {
        let result = p2(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 0);
    }
}
