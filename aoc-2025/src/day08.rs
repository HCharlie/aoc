use std::collections::{BinaryHeap, HashMap};

use anyhow::Result;

fn squared_euclidean_distance(p1: (i64, i64, i64), p2: (i64, i64, i64)) -> i64 {
    (p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2) + (p1.2 - p2.2).pow(2)
}

// Union-Find data structure
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // path compression
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            // Union by rank
            if self.rank[root_x] < self.rank[root_y] {
                self.parent[root_x] = root_y;
            } else if self.rank[root_x] > self.rank[root_y] {
                self.parent[root_y] = root_x;
            } else {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
        }
    }

    fn get_circuit_sizes(&mut self, size: usize) -> Vec<usize> {
        let mut circuit_sizes: HashMap<usize, usize> = HashMap::new();
        for i in 0..size {
            let root = self.find(i);
            *circuit_sizes.entry(root).or_insert(0) += 1;
        }
        let mut sizes: Vec<usize> = circuit_sizes.values().copied().collect();
        sizes.sort_by(|a, b| b.cmp(a)); // sort descending
        sizes
    }
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

    // Use Union-Find to connect points and track circuits
    let mut uf = UnionFind::new(points.len());
    
    while !shortest_distances.is_empty() {
        let (_, i, j) = shortest_distances.pop().unwrap();
        uf.union(i, j);
    }

    // Get circuit sizes and multiply the three largest
    let circuit_sizes = uf.get_circuit_sizes(points.len());
    let product: i64 = circuit_sizes.iter().take(3).map(|&size| size as i64).product();
    
    Ok(product)
}

pub fn p2(_input_text: &str) -> Result<i64> {
    let total: i64 = 0;

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
