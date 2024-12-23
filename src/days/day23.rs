use rustworkx_core::graph_ext::contraction;

use super::utils::{get_input_content, submit_check_answer};
use crate::Level;
use std::{collections::{HashMap, HashSet}, error::Error, vec};


fn p1(input_text: &str) -> Result<String, Box<dyn Error>> {
    let mut m: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in input_text.lines() {
        let parts: Vec<&str> = line.split('-').collect();
        let a = parts[0];
        let b = parts[1];
        m.entry(a).or_insert(HashSet::new()).insert(b);
        m.entry(b).or_insert(HashSet::new()).insert(a);
    }
    // println!("{:?}", m);
    let mut seen = HashSet::new();
    for (&k1, v1) in m.iter() {
        for (&k2, v2) in m.iter() {
            if k1 == k2 {
                continue;
            }
            if !v1.contains(k2) || !v2.contains(k1) {
                continue;
            }
            let intersection: HashSet<&str> = v1.intersection(v2).copied().collect();
            // println!("{:?} {:?} {:?}", k1, k2, intersection);
            for &k3 in intersection.iter() {
                if k1.starts_with('t') || k2.starts_with('t') || k3.starts_with('t') {
                    let mut triple = vec![k1, k2, k3];
                    triple.sort();
                    seen.insert(triple);
                }

            }
        }
    }
    // println!("{:?}", seen);
    Ok((seen.len()).to_string())

}


fn _search<'a>(node: &str, req: &mut Vec<&'a str>, seen: &mut HashSet<Vec<&'a str>>, conns: &HashMap<&str, HashSet<&'a str>>) -> () {
    let mut key = req.clone();
    key.sort();
    if seen.contains(&key) {
        return;
    }
    seen.insert(key);
    if let Some(neighbors) = conns.get(node) {
        for &neighbor in neighbors {
            if req.contains(&neighbor) {
                continue;
            }
            
            let req_set: HashSet<_> = req.iter().copied().collect();
            match conns.get(neighbor) {
                Some(n) if req_set.is_subset(n) => (),
                _ => continue,
            };

            let mut new_req = req.clone();
            new_req.push(neighbor);
            _search(neighbor, &mut new_req, seen, conns);
        }
    }
}

fn p2(input_text: &str) -> Result<String, Box<dyn Error>> {

    let mut m: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in input_text.lines() {
        let parts: Vec<&str> = line.split('-').collect();
        let a = parts[0];
        let b = parts[1];
        m.entry(a).or_insert(HashSet::new()).insert(b);
        m.entry(b).or_insert(HashSet::new()).insert(a);
    }
    let mut seen = HashSet::new();

    for (&k, _) in m.iter() {
        let mut req = vec![k];
        _search(k, &mut req, &mut seen, &m);
    }
    let mut longest: Vec<&str> = seen.iter()
        .max_by_key(|&x| x.len())
        .map_or_else(|| Vec::new(), |x| x.clone());
    longest.sort();
    Ok(longest.join(","))
}

pub fn run(day: u8, level: Level, debug: bool) -> () {
    let example_input = 
"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";



    let sol_func = match level {
        Level::One => p1,
        Level::Two => p2,
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
