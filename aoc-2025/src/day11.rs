use anyhow::Result;
use std::collections::HashMap;

pub fn p1(input_text: &str) -> Result<i64> {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input_text.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() < 2 { continue; }
        let key = parts[0];
        let values: Vec<&str> = parts[1].split_whitespace().collect();
        graph.insert(key, values);
    }

    let mut memo = HashMap::new();
    Ok(count_paths("you", "out", &graph, &mut memo))
}

fn count_paths<'a>(
    current: &'a str,
    target: &'a str,
    graph: &HashMap<&str, Vec<&'a str>>,
    memo: &mut HashMap<(&'a str, &'a str), i64>,
) -> i64 {
    if current == target {
        return 1;
    }
    if let Some(&count) = memo.get(&(current, target)) {
        return count;
    }

    let mut total_paths = 0;
    if let Some(neighbors) = graph.get(current) {
        for neighbor in neighbors {
            total_paths += count_paths(neighbor, target, graph, memo);
        }
    }

    memo.insert((current, target), total_paths);
    total_paths
}

pub fn p2(input_text: &str) -> Result<i64> {
     let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input_text.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() < 2 { continue; }
        let key = parts[0];
        let values: Vec<&str> = parts[1].split_whitespace().collect();
        graph.insert(key, values);
    }

    let mut memo = HashMap::new();
    
    // Path 1: svr -> dac -> fft -> out
    let svr_dac = count_paths("svr", "dac", &graph, &mut memo);
    let dac_fft = count_paths("dac", "fft", &graph, &mut memo);
    let fft_out = count_paths("fft", "out", &graph, &mut memo);
    
    // Path 2: svr -> fft -> dac -> out
    let svr_fft = count_paths("svr", "fft", &graph, &mut memo);
    let fft_dac = count_paths("fft", "dac", &graph, &mut memo);
    let dac_out = count_paths("dac", "out", &graph, &mut memo);

    let total = (svr_dac * dac_fft * fft_out) + (svr_fft * fft_dac * dac_out);
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_P1: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const EXAMPLE_INPUT_P2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_p1_example() {
        let result = p1(EXAMPLE_INPUT_P1).unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    fn test_p2_example() {
        let result = p2(EXAMPLE_INPUT_P2).unwrap();
        assert_eq!(result, 2);
    }
}
