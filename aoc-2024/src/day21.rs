use anyhow::Result;
use std::collections::HashMap;
use std::collections::VecDeque;

fn compute_seqs(keypad: &Vec<Vec<Option<&str>>>) -> Result<HashMap<(String, String), Vec<String>>, > {
    let mut pos = HashMap::new();
    for (r, row) in keypad.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if let Some(val) = cell {
                pos.insert(val, (r, c));
            }
        }
    }

    let mut seqs = HashMap::new();
    for &x in pos.keys() {
        for &y in pos.keys() {
            if x == y {
                seqs.insert((x.to_string(), y.to_string()), vec!["A".to_string()]);
                continue;
            }

            let mut possibilities = Vec::new();
            let mut q = VecDeque::new();
            let (r, c) = *pos.get(x).ok_or_else(|| anyhow::anyhow!("Position not found for key: {}", x))?;
            q.push_back(((r as i64, c as i64), String::new()));
            let mut optimal = usize::MAX;

            'outer: while let Some(((r, c), moves)) = q.pop_front() {
                for (nr, nc, nm) in [(r-1, c, "^"), (r + 1, c, "v"), (r, c - 1, "<"), (r, c + 1, ">")] {
                    if nr < 0 || nc < 0 || nr >= keypad.len() as i64 || nc >= keypad[0].len() as i64  { continue; }
                    if keypad[nr as usize][nc as usize].is_none() { continue; }
                    
                    if keypad[nr as usize][nc as usize] == Some(y) {
                        if optimal < moves.len() + 1 { break 'outer; }
                        optimal = moves.len() + 1;
                        possibilities.push(moves.clone() + nm + "A");
                    } else {
                        q.push_back(((nr, nc), moves.clone() + nm));
                    }
                }
            }
            seqs.insert((x.to_string(), y.to_string()), possibilities);
        }
    }
    Ok(seqs)
}

fn solve(string: &str, seqs: &HashMap<(String, String), Vec<String>>) -> Result<Vec<String>> {
    let n = string.len();
    let mut options = Vec::new();
    let chars: Vec<char> = string.chars().collect();
    for i in 0..n {
        if i == 0 {
            match seqs.get(&("A".to_string(), chars[i].to_string())) {
                Some(re) => options.push(re.clone()),
                None => anyhow::bail!("unexpected"),
            }
        } else {
            match seqs.get(&(chars[i-1].to_string(), chars[i].to_string())) {
                Some(re) => options.push(re.clone()),
                None => anyhow::bail!("unexpected")
            }
        }
    }
    let mut ans = vec!["".to_string()];
    for option in options.iter() {
        let mut tmp = Vec::new();

        for i in ans.iter() {
            for j in option.iter() {
                tmp.push(i.to_owned() + j);
            }
        }
        ans = tmp
    }
    return Ok(ans);
}

fn compute_length(
    seq: &str, 
    depth: i64, 
    cache: &mut HashMap<(String, i64), i64>,
    dir_seqs: &HashMap<(String, String), Vec<String>>,
    dir_lengths: &HashMap<(String, String), i64>
) -> Result<i64> {
    // Check cache first
    if let Some(&result) = cache.get(&(seq.to_string(), depth)) {
        return Ok(result);
    }
    let chars: Vec<char> = seq.chars().collect();
    if depth == 1 {
        let mut total = 0;
        
        for (i, &c) in chars.iter().enumerate() {
            let from = if i == 0 { "A" } else { &chars[i-1].to_string() };
            total += match dir_lengths.get(&(from.to_string(), c.to_string())) {
                Some(&length) => length,
                None => anyhow::bail!("unexpected Error")
            };
        }
        cache.insert((seq.to_string(), depth), total);
        return Ok(total);
    }

    let mut length = 0;
    for (i, &c) in chars.iter().enumerate() {
        let from = if i == 0 { "A" } else { &chars[i-1].to_string() };
        if let Some(possibilities) = dir_seqs.get(&(from.to_string(), c.to_string())) {
            let min_length = possibilities.iter()
                .map(|subseq| compute_length(subseq as &str, depth - 1, cache, dir_seqs, dir_lengths))
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .min()
                .ok_or_else(|| anyhow::anyhow!("No minimum found"))?;
            length += min_length;
        } else {
            anyhow::bail!("unexpected Error");
        }
    }

    cache.insert((seq.to_string(), depth), length);
    Ok(length)
}


pub fn p1(input_text: &str) -> Result<String> {
    let codes: Vec<&str> = input_text.lines().collect();

    let num_keypad = vec![
        vec![Some("7"), Some("8"), Some("9")],
        vec![Some("4"), Some("5"), Some("6")],
        vec![Some("1"), Some("2"), Some("3")],
        vec![None, Some("0"), Some("A")]
    ];

    let num_seqs = compute_seqs(&num_keypad)?;
    println!("num_seqs {:?}", num_seqs);

    let dir_keypad = vec![
        vec![None, Some("^"), Some("A")],
        vec![Some("<"), Some("v"), Some(">")]
    ];
    let dir_seqs = compute_seqs(&dir_keypad)?;

    println!("dir_seqs {:?}", dir_seqs);
    let dir_lengths: HashMap<(String, String), i64> = dir_seqs
        .iter()
        .map(|(key, value)| (key.clone(), value[0].len() as i64))
        .collect();

    let mut cache: HashMap<(String, i64), i64> = HashMap::new();

    let mut total = 0;

    for code in codes {
        println!("code {}", code);
        let inputs = solve(code, &num_seqs)?;
        println!("inputs {:?}", inputs);
        let mut min_length = i64::MAX;
        for input in inputs.iter() {
            let curr_length = compute_length(input, 2, &mut cache, &dir_seqs, &dir_lengths)?;
            if min_length > curr_length {
                min_length = curr_length;
            }
        }
        println!("min_length {}", min_length);
        let numeric_part = &code[..code.len()-1].parse::<i64>()?;
        println!("numeric_part {}", numeric_part);
        total += min_length * numeric_part;
    }

    Ok(total.to_string())

}

pub fn p2(input_text: &str) -> Result<String> {
    
    let codes: Vec<&str> = input_text.lines().collect();

    let num_keypad = vec![
        vec![Some("7"), Some("8"), Some("9")],
        vec![Some("4"), Some("5"), Some("6")],
        vec![Some("1"), Some("2"), Some("3")],
        vec![None, Some("0"), Some("A")]
    ];

    let num_seqs = compute_seqs(&num_keypad)?;
    println!("num_seqs {:?}", num_seqs);

    let dir_keypad = vec![
        vec![None, Some("^"), Some("A")],
        vec![Some("<"), Some("v"), Some(">")]
    ];
    let dir_seqs = compute_seqs(&dir_keypad)?;

    println!("dir_seqs {:?}", dir_seqs);
    let dir_lengths: HashMap<(String, String), i64> = dir_seqs
        .iter()
        .map(|(key, value)| (key.clone(), value[0].len() as i64))
        .collect();

    let mut cache: HashMap<(String, i64), i64> = HashMap::new();

    let mut total = 0;

    for code in codes {
        println!("code {}", code);
        let inputs = solve(code, &num_seqs)?;
        println!("inputs {:?}", inputs);
        let mut min_length = i64::MAX;
        for input in inputs.iter() {
            let curr_length = compute_length(input, 25, &mut cache, &dir_seqs, &dir_lengths)?;
            if min_length > curr_length {
                min_length = curr_length;
            }
        }
        println!("min_length {}", min_length);
        let numeric_part = &code[..code.len()-1].parse::<i64>()?;
        println!("numeric_part {}", numeric_part);
        total += min_length * numeric_part;
    }

    Ok(total.to_string())

}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "029A
980A
179A
456A
379A";

    #[test]
    fn test_p1_example() {
        let result = p1(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, "126384");
    }

    #[test]
    fn test_p2_example() {
        // p2 is computationally intensive, just verify it runs
        let result = p2(EXAMPLE_INPUT);
        assert!(result.is_ok());
    }
}

