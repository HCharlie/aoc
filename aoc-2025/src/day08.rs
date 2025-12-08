use std::collections::{BinaryHeap, HashMap};

use anyhow::Result;

type Point = (i64, i64, i64);

fn squared_euclidean_distance(p1: Point, p2: Point) -> i64 {
    (p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2) + (p1.2 - p2.2).pow(2)
}

/// Union-Find data structure with path compression and union by rank
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    n_components: usize,
}

impl UnionFind {
    fn new(n_nodes: usize) -> Self {
        UnionFind {
            parent: (0..n_nodes).collect(),
            rank: vec![0; n_nodes],
            n_components: n_nodes,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // path compression
        }
        self.parent[x]
    }

    /// Unites two components. Returns true if they were in different components.
    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        // Union by rank
        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
        } else {
            self.parent[root_y] = root_x;
            self.rank[root_x] += 1;
        }
        
        self.n_components -= 1;
        true
    }

    fn component_count(&self) -> usize {
        self.n_components
    }

    /// Returns component sizes sorted in descending order
    fn get_component_sizes(&mut self) -> Vec<usize> {
        let mut sizes: HashMap<usize, usize> = HashMap::new();
        for i in 0..self.parent.len() {
            let root = self.find(i);
            *sizes.entry(root).or_insert(0) += 1;
        }
        let mut result: Vec<usize> = sizes.values().copied().collect();
        result.sort_unstable_by(|a, b| b.cmp(a));
        result
    }
}

/// Parse input into points and optionally extract n_connections
fn parse_input(input_text: &str, extract_n_connections: bool) -> (Vec<Point>, Option<usize>) {
    let mut points = Vec::new();
    let mut n_connections = None;

    for line in input_text.lines() {
        if line.is_empty() {
            continue;
        }
        if line.contains("NUMBERS: ") {
            if extract_n_connections {
                n_connections = line
                    .split_whitespace()
                    .nth(1)
                    .and_then(|s| s.parse::<usize>().ok());
            }
            continue;
        }
        let parts: Vec<i64> = line
            .split(',')
            .filter_map(|x| x.parse::<i64>().ok())
            .collect();
        if parts.len() == 3 {
            points.push((parts[0], parts[1], parts[2]));
        }
    }

    (points, n_connections)
}

pub fn p1(input_text: &str) -> Result<i64> {
    let (points, n_connections) = parse_input(input_text, true);
    let n_connections = n_connections.unwrap_or(0);

    // Use a max heap to efficiently track the k smallest distances
    // (max heap because we want to remove the largest when we exceed k)
    let mut max_heap = BinaryHeap::new();
    
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let distance = squared_euclidean_distance(points[i], points[j]);
            
            if max_heap.len() < n_connections {
                max_heap.push((distance, i, j));
            } else if let Some(&(max_dist, _, _)) = max_heap.peek() {
                if distance < max_dist {
                    max_heap.pop();
                    max_heap.push((distance, i, j));
                }
            }
        }
    }

    // Connect points using Union-Find
    let mut uf = UnionFind::new(points.len());
    while let Some((_, i, j)) = max_heap.pop() {
        uf.union(i, j);
    }

    // Multiply the three largest component sizes
    let component_sizes = uf.get_component_sizes();
    let product: i64 = component_sizes
        .iter()
        .take(3)
        .map(|&size| size as i64)
        .product();
    
    Ok(product)
}

pub fn p2(input_text: &str) -> Result<i64> {
    let (points, _) = parse_input(input_text, false);

    // Build min heap of all pairwise distances (using negative distances for max heap)
    let mut min_heap = BinaryHeap::new();
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let distance = squared_euclidean_distance(points[i], points[j]);
            min_heap.push((-distance, i, j)); // Negative for min heap behavior
        }
    }

    // Connect points in order of increasing distance until all points are in one component
    let mut uf = UnionFind::new(points.len());
    let mut last_pair = (Point::default(), Point::default());
    
    while let Some((_, i, j)) = min_heap.pop() {
        if uf.union(i, j) && uf.component_count() == 1 {
            last_pair = (points[i], points[j]);
            break;
        }
    }

    Ok(last_pair.0.0 * last_pair.1.0)
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
        assert_eq!(result, 25272);
    }
}
