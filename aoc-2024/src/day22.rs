use anyhow::Result;
use std::{collections::HashMap};
use std::collections::VecDeque;

pub const EXAMPLE_INPUT: &str = "1
2
3
2024";

fn _mix(num: &i64, secret: &i64) -> i64 {
    *num ^ *secret
}

fn _prune(secret: &i64) -> i64 {
    *secret & 0xFFFFFF
}

fn _cal(num: &i64, it: &i64) -> i64 {
    let mut secret = *num;
    for _ in 0..*it {
        
        let new = secret << 6;
        secret = _mix(&new, &secret);
        secret = _prune(&secret);

        let new = secret >> 5;
        secret = _mix(&new, &secret);
        secret = _prune(&secret);

        let new = secret << 11;
        secret = _mix(&new, &secret);
        secret = _prune(&secret);
    }
    secret
}

pub fn p1(input_text: &str) -> Result<String> {
    let nums = input_text.lines()
        .map(|line| line.parse::<i64>())
        .collect::<Result<Vec<i64>, _>>()?;

    let mut total = 0;
    let it = 2000;
    for num in nums.iter() {
        total += _cal(&num, &it);
    }
    Ok(total.to_string())

}


fn _collect_seqs_map(num: &i64, it: &i64) -> HashMap<(i64, i64, i64, i64), i64> {
    let mut seqs_map = HashMap::new();
    let mut secret = *num;
    let mut diff: VecDeque<i64> = VecDeque::new();
    let mut prev = secret%10;

    for i in 0..*it {
        
        let new = secret << 6;
        secret = _mix(&new, &secret);
        secret = _prune(&secret);

        let new = secret >> 5;
        secret = _mix(&new, &secret);
        secret = _prune(&secret);

        let new = secret << 11;
        secret = _mix(&new, &secret);
        secret = _prune(&secret);

        let curr = secret%10;
        diff.push_back(curr - prev);

        if i >= 3 {
            let key = (diff[0], diff[1], diff[2], diff[3]);
            if !seqs_map.contains_key(&key) {
                seqs_map.insert(key, curr);
            }
            diff.pop_front();
        }
        prev = curr;
    }
    seqs_map

}


pub fn p2(input_text: &str) -> Result<String> {
    let nums = input_text.lines()
        .map(|line| line.parse::<i64>())
        .collect::<Result<Vec<i64>, _>>()?;
    // println!("{:?}", nums);
    let mut seqs_map = HashMap::new();

    let it = 2000;
    for num in nums.iter() {
        let new_seqs_map = _collect_seqs_map(&num, &it);
        for (key, value) in new_seqs_map {
            seqs_map.entry(key)
                .and_modify(|e| *e += value)
                .or_insert(value);
        }
    }
    let total = *seqs_map.values().max().unwrap_or(&0);
    Ok(total.to_string())
}

