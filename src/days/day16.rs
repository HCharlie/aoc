use super::utils::{get_input_content, submit_check_answer};
use crate::Level;
use std::{collections::{BinaryHeap,HashMap, VecDeque, HashSet}, error::Error};
use rustworkx_core::petgraph::{graph::NodeIndex, Graph};

use rustworkx_core::shortest_path::{dijkstra, all_shortest_paths};
use rustworkx_core::dictmap::DictMap;

use rustworkx_core::Result as GraphResult;



fn p1(input_text: &str) -> Result<i64, Box<dyn Error>> {
    let mut score = 0;
    let grid: Vec<Vec<char>> = input_text.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    // Find starting position
    let (mut sr, mut sc) = (0, 0);
    'outer: for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 'S' {
                sr = r as i64;
                sc = c as i64;
                break 'outer;
            }
        }
    }

    let mut pq = BinaryHeap::new();
    let mut seen = HashSet::new();
    
    // BinaryHeap is max-heap, so we negate cost for min-heap behavior
    pq.push((0, sr, sc, 0, 1));
    seen.insert((sr, sc, 0, 1));

    while let Some((cost, r, c, dr, dc)) = pq.pop() {
        seen.insert((r, c, dr, dc));
        if grid[r as usize][c as usize] == 'E' {
            score = -cost;
            break;
        }

        let moves = [
            (cost - 1, r + dr, c + dc, dr, dc),      // forward
            (cost - 1000, r, c, dc, -dr),            // turn right
            (cost - 1000, r, c, -dc, dr)             // turn left
        ];

        for (new_cost, nr, nc, ndr, ndc) in moves {

            if grid[nr as usize][nc as usize] == '#' {
                continue;
            }
            if seen.contains(&(nr, nc, ndr, ndc)) {
                continue;
            }
            pq.push((new_cost, nr, nc, ndr, ndc));

        }
    }
    Ok(score)
}


fn p2(input_text: &str) -> Result<i64, Box<dyn Error>> {
    let grid: Vec<Vec<char>> = input_text.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    // Find starting position
    let (mut sr, mut sc) = (0, 0);
    'outer: for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 'S' {
                sr = r as i64;
                sc = c as i64;
                break 'outer;
            }
        }
    }

    let mut pq = BinaryHeap::new();
    let mut lowest_cost: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();
    let mut backtrack: HashMap<(i64, i64, i64, i64), HashSet<(i64, i64, i64, i64)>> = HashMap::new();
    let mut best_cost = i64::MAX;
    let mut end_states = HashSet::new();

    // BinaryHeap is max-heap, so we negate costs
    pq.push((0, sr, sc, 0, 1));
    lowest_cost.insert((sr, sc, 0, 1), 0);

    while let Some((neg_cost, r, c, dr, dc)) = pq.pop() {
        let cost = -neg_cost;  // Convert back to positive cost
        if cost > *lowest_cost.get(&(r, c, dr, dc)).unwrap_or(&i64::MAX) {
            continue;
        }
        
        if grid[r as usize][c as usize] == 'E' {
            if cost > best_cost {
                break;
            }
            best_cost = cost;
            end_states.insert((r, c, dr, dc));
        }

        let moves = [
            (cost + 1, r + dr, c + dc, dr, dc),      // forward
            (cost + 1000, r, c, dc, -dr),            // turn right
            (cost + 1000, r, c, -dc, dr)             // turn left
        ];

        for (new_cost, nr, nc, ndr, ndc) in moves {
            if grid[nr as usize][nc as usize] == '#' {
                continue;
            }
            
            let lowest = *lowest_cost.get(&(nr, nc, ndr, ndc)).unwrap_or(&i64::MAX);
            if new_cost > lowest {
                continue;
            }
            
            if new_cost < lowest {
                backtrack.insert((nr, nc, ndr, ndc), HashSet::new());
                lowest_cost.insert((nr, nc, ndr, ndc), new_cost);
            }
            
            if let Some(prev_states) = backtrack.get_mut(&(nr, nc, ndr, ndc)) {
                prev_states.insert((r, c, dr, dc));
            }
            
            pq.push((-new_cost, nr, nc, ndr, ndc));
        }
    }

    let mut states: VecDeque<_> = end_states.iter().cloned().collect();
    let mut seen: HashSet<_> = end_states.clone();

    while let Some(key) = states.pop_front() {
        if let Some(last_states) = backtrack.get(&key) {
            for last in last_states {
                if seen.contains(last) {
                    continue;
                }
                seen.insert(*last);
                states.push_back(*last);
            }
        }
    }

    let num = seen.iter().map(|(r, c, _, _)| (r, c)).collect::<HashSet<_>>().len() as i64;
    Ok(num)
}


fn p1_rustworkx(input_text: &str) -> Result<i64, Box<dyn Error>> {
    let grid: Vec<Vec<char>> = input_text.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut graph = Graph::new();
    let mut node_indices = HashMap::new();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    // Create nodes for each valid position
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] != '#' {
                for d in 0..directions.len() {
                    let idx = graph.add_node((r, c, d));
                    node_indices.insert((r as i64, c as i64, d as i64), idx);
                }
                
            }
        }
    }

    // Find start and end
    let (mut start_pos, mut end_pos) = ((0, 0), (0, 0));
    for (pos, _) in &node_indices {
        if grid[pos.0 as usize][pos.1 as usize] == 'S' {
            start_pos = (pos.0, pos.1);
        } else if grid[pos.0 as usize][pos.1 as usize] == 'E' {
            end_pos = (pos.0, pos.1);
        }
    }

    // Add edges
    for (r, c, d) in node_indices.keys() {        
        let curr_idx = node_indices[&(*r, *c, *d)];
        for dd in [1, -1] {
            let next_d = ((*d + dd + 4) % 4) as i64;
            if let Some(&next_idx) = node_indices.get(&(*r, *c, next_d)) {
                graph.add_edge(curr_idx, next_idx, 1000);
            }
        }

        let (dr, dc) = directions[*d as usize];
        if let Some(&next_idx) = node_indices.get(&(*r+dr, *c+dc, *d)) {
            graph.add_edge(curr_idx, next_idx, 1);
        }
    }

    let start_idx = node_indices[&(start_pos.0, start_pos.1, 0)];
    let mut ans = i64::MAX;
    for d in 0..4 {
        let end_idx = node_indices[&(end_pos.0, end_pos.1, d)];

        let res: GraphResult<DictMap<NodeIndex, usize>> = dijkstra(
           &graph, start_idx, Some(end_idx), |e| Ok(*e.weight()), None
        );
        let dist_map = res?;
        let dist = dist_map.get(&end_idx).ok_or("No path found")?;
        ans = ans.min(*dist as i64);   
    }
    Ok(ans)
}


fn p2_rustworkx(input_text: &str) -> Result<i64, Box<dyn Error>> {
    let grid: Vec<Vec<char>> = input_text.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut graph = Graph::new();
    let mut node_indices = HashMap::new();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    // Create nodes for each valid position
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] != '#' {
                for d in 0..directions.len() {
                    let idx = graph.add_node((r, c, d));
                    node_indices.insert((r as i64, c as i64, d as i64), idx);
                }
                
            }
        }
    }

    // Find start and end
    let (mut start_pos, mut end_pos) = ((0, 0), (0, 0));
    for (pos, _) in &node_indices {
        if grid[pos.0 as usize][pos.1 as usize] == 'S' {
            start_pos = (pos.0, pos.1);
        } else if grid[pos.0 as usize][pos.1 as usize] == 'E' {
            end_pos = (pos.0, pos.1);
        }
    }

    // Add edges
    for (r, c, d) in node_indices.keys() {        
        let curr_idx = node_indices[&(*r, *c, *d)];
        for dd in [1, -1] {
            let next_d = ((*d + dd + 4) % 4) as i64;
            if let Some(&next_idx) = node_indices.get(&(*r, *c, next_d)) {
                graph.add_edge(curr_idx, next_idx, 1000);
            }
        }

        let (dr, dc) = directions[*d as usize];
        if let Some(&next_idx) = node_indices.get(&(*r+dr, *c+dc, *d)) {
            graph.add_edge(curr_idx, next_idx, 1);
        }
        
    }

    let start_idx = node_indices[&(start_pos.0, start_pos.1, 0)];
    let mut shortest_distance = i64::MAX;
    let mut best_end = (0, 0, 0);
    for d in 0..4 {
        let end_idx = node_indices[&(end_pos.0, end_pos.1, d)];

        let res: GraphResult<DictMap<NodeIndex, usize>> = dijkstra(
           &graph, start_idx, Some(end_idx), |e| Ok(*e.weight()), None
        );
        let dist_map = res?;
        let dist = dist_map.get(&end_idx).ok_or("No path found")?;
        if shortest_distance > *dist as i64 {
            shortest_distance = *dist as i64;
            best_end = (end_pos.0, end_pos.1, d);
        }        
    }
    println!("shortest distance: {:?}", shortest_distance);

    let end_idx = node_indices[&best_end];

    let res: GraphResult<Vec<Vec<NodeIndex>>> = all_shortest_paths(
        &graph, start_idx, end_idx, |e| Ok(*e.weight())
    );
    let dist_map = res?;

    let mut unique_nodes = HashSet::new();
    for path in dist_map {
        for node in path {
            let (r, c, _) = graph[node];
            unique_nodes.insert((r, c));
        }
    }
    Ok(unique_nodes.len() as i64)

}


pub fn run(day: u8, level: Level, debug: bool) -> () {
    let example_input = 
"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";


    let sol_func = match level {
        Level::One => p1_rustworkx,
        Level::Two => p2_rustworkx,
    };

    match sol_func(example_input) {
        Ok(result) => println!("Example result: {}", result),
        Err(e) => eprintln!("Error processing example: {}", e),
    }

    let content = match get_input_content(day) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            return;
        }
    };

    let answer = match sol_func(&content) {
        Ok(answer) => answer,
        Err(e) => {
            eprintln!("Error processing input: {}", e);
            return;
        }
    };

    if debug {
        println!("Answer: {}", answer);
        return ();
    }
    match submit_check_answer(day, level as u8, &answer.to_string()) {
        Ok(is_correct) => println!(
            "Answer {} is {}",
            answer,
            if is_correct { "correct" } else { "wrong" }
        ),
        Err(e) => {
            eprintln!("Error submitting answer: {}", e);
            return;
        }
    }
}
