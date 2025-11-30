use anyhow::Result;
use std::{collections::{HashMap, VecDeque}};

pub const EXAMPLE_INPUT: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";


pub fn p1(input_text: &str) -> Result<String> {
    let parts: Vec<&str> = input_text.split("\n\n").collect();

    let mut seen = HashMap::new();

    for line in parts[0].lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let key = parts[0];
        let value = parts[1].trim() == "1";
        seen.insert(key, value);
    }

    let mut relations: HashMap<(&str, &str, &str), &str> = HashMap::new();
    let mut dq = VecDeque::new();

    for line in parts[1].lines() {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let keys: Vec<&str> = parts[0].split(" ").collect();
        relations.insert((keys[0], keys[1], keys[2]), parts[1]);
        dq.push_back((keys[0], keys[1], keys[2], parts[1]));
    }

    while let Some((k1, op, k2, out)) = dq.pop_front() {
        if seen.contains_key(k1) && seen.contains_key(k2) {
            let &v1 = seen.get(k1).ok_or_else(|| anyhow::anyhow!("key not found"))?;
            let &v2 = seen.get(k2).ok_or_else(|| anyhow::anyhow!("key not found"))?;
            let result = match op {
                "AND" => v1 && v2,
                "OR" => v1 || v2,
                "XOR" => v1 ^ v2,
                _ => {
                    anyhow::bail!("unknown op");
                },
            };
            seen.insert(out, result);
            continue;
        }
        if op == "AND" {
            if seen.contains_key(k1) && *seen.get(k1).ok_or_else(|| anyhow::anyhow!("key not found"))? == false {
                    seen.insert(out, false);
            } else if seen.contains_key(k2) && *seen.get(k2).ok_or_else(|| anyhow::anyhow!("key not found"))? == false {
                    seen.insert(out, false);
            } else {
                dq.push_back((k1, op, k2, out));
            }
            continue;
        }
        if op == "OR" {
            if seen.contains_key(k1) && *seen.get(k1).ok_or_else(|| anyhow::anyhow!("key not found"))? == true {
                    seen.insert(out, true);
            } else if seen.contains_key(k2) && *seen.get(k2).ok_or_else(|| anyhow::anyhow!("key not found"))? == true {
                    seen.insert(out, true);
            } else {
                dq.push_back((k1, op, k2, out));
            }
            continue;
        }
        
        dq.push_back((k1, op, k2, out));
    }

   
    let mut z_keys: Vec<&str> = seen.keys()
        .filter(|&&k| k.starts_with("z"))
        .copied()
        .collect();

    z_keys.sort();
    z_keys.reverse();


    let mut z_values: Vec<bool> = Vec::new();
    for k in z_keys {
        match seen.get(k) {
            Some(&value) => z_values.push(value),
            None => anyhow::bail!("Missing z-key value"),
        }
    }

    let binary_str: String = z_values.iter()
        .map(|&b| if b { '1' } else { '0' })
        .collect();

    let decimal = i64::from_str_radix(&binary_str, 2)?;

    Ok(decimal.to_string())

}

fn _swap_wires<'a>(rel: &mut HashMap<(&'a str, &'a str, &'a str), &'a str>, w1: &'a str, w2: &'a str) -> () {
    let mut keys_to_swap = Vec::new();
    for (&k, &v) in rel.iter() {
        if v == w1 || v == w2 {
            keys_to_swap.push(k);
        }
    }
    for k in keys_to_swap {
        let v = rel[&k];
        rel.insert(k, if v == w1 { w2 } else { w1 });
    }
}

fn _check(relations: &HashMap<(&str, &str, &str), &str>, n_bits: i64 ) -> i64 {
    let x : Vec<String> = (0..n_bits).map(|i| format!("x{:02}", i)).collect();
    let y : Vec<String> = (0..n_bits).map(|i| format!("y{:02}", i)).collect();
    let x00 = x[0].as_str();
    let y00 = y[0].as_str();
    let z00_test = match relations.get(&(x00, "XOR", y00)).or(relations.get(&(y00, "XOR", x00))) {
        Some(&val) => val == "z00",
        None => false,
    };
    if z00_test == false {
        return 0;
    }

    let mut c = *relations.get(&(x00, "AND", y00)).or(relations.get(&(y00, "AND", x00))).unwrap();

    
    for i in 1..n_bits {
        
        let x = x[i as usize].as_str();
        let y = y[i as usize].as_str();
        let x_and_y = *relations.get(&(x, "AND", y)).or(relations.get(&(y, "AND", x))).unwrap();
        let x_xor_y = *relations.get(&(x, "XOR", y)).or(relations.get(&(y, "XOR", x))).unwrap();



        let _c_xor_x_xor_y: &str = match relations.get(&(c, "XOR", x_xor_y)).or(relations.get(&(x_xor_y, "XOR", c))) {
            Some(&val) => {
                if val != format!("z{:02}", i) {
                    return i;
                }
                val
            },
            None => {
                return i;
            }
        };

        let c_and_x_xor_y = match relations.get(&(c, "AND", x_xor_y)).or(relations.get(&(x_xor_y, "AND", c))) {
            Some(&val) => val,
            None => {
                return i;
            }
        };

        c = match relations.get(&(c_and_x_xor_y, "OR", x_and_y)).or(relations.get(&(x_and_y, "OR", c_and_x_xor_y))) {
            Some(&val) => val,
            None => {
                return i;
            }
            
        };
        
    }

    return n_bits;
}


pub fn p2(input_text: &str) -> Result<String> {
    let parts: Vec<&str> = input_text.split("\n\n").collect();
    let n_bits = (parts[0].lines().count() / 2) as i64;
    println!("n_bits: {}", n_bits);
    let mut relations: HashMap<(&str, &str, &str), &str> = HashMap::new();

    for line in parts[1].lines() {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let keys: Vec<&str> = parts[0].split(" ").collect();
        relations.insert((keys[0], keys[1], keys[2]), parts[1]);
    }
    let mut wrong_wires: Vec<String> = Vec::new();
    let mut baseline = _check(&relations, n_bits);
    println!("Baseline start from: {}", baseline);
    let keys: Vec<(&str, &str, &str)> = relations.keys().copied().collect();
    for it in 0..4 {
        println!("Iteration: {}", it);
        'outer: for &k1 in keys.iter() {
            for &k2 in keys.iter() {
                if k1 == k2 {
                    continue;
                }
                let v1 = *relations.get(&k1).unwrap();
                let v2 = *relations.get(&k2).unwrap();
                
                _swap_wires(&mut relations, v1, v2);
                if it == 3 {
                    if v1 == "tqr" && v2 == "hth" || v1 == "hth" && v2 == "tqr" {
                        println!("Swapped: {} {}", v1, v2);
                    }
                }
                let tmp = _check(&relations, n_bits);
                // if it == 3 {
                //     println!("Tmp: {}", tmp);
                //     println!("Swapped: {} {}", v1, v2);
                // }
                if tmp > baseline {
                    println!("Swapped: {} {}", v1, v2);
                    println!("New baseline: {}", tmp);
                    baseline = tmp;
                    wrong_wires.push(v1.to_string());
                    wrong_wires.push(v2.to_string());
                    relations = relations;
                    break 'outer;
                }
                _swap_wires(&mut relations, v1, v2);

            }
        }
        
    }
    wrong_wires.sort();

    Ok(wrong_wires.join(",").to_string())

}

