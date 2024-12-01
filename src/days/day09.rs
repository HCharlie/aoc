use super::utils::{get_input_content, submit_check_answer};

use crate::Level;


use std::error::Error;
use std::collections::HashMap;

fn p1(input_text: &str) -> Result<i64, Box<dyn Error>> {
    let mut checksum: i64 = 0;
    let nums: Vec<i64> = input_text
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as i64)
        .collect();

    let mut disk = Vec::new();
    let mut fid = 0;

    for (i, &x) in nums.iter().enumerate() {
        if i % 2 == 0 {
            disk.extend(vec![fid; x as usize]);
            fid += 1;
        } else {
            disk.extend(vec![-1; x as usize]);
        }
    }

    let blanks: Vec<usize> = disk.iter()
        .enumerate()
        .filter(|(_, &x)| x == -1)
        .map(|(i, _)| i)
        .collect();

    for i in blanks {
        // Remove trailing -1 values
        while disk.last().map_or(false, |&last| last == -1) {
            disk.pop();
        }
        // If we've run out of space or valid elements, break
        if disk.len() <= i {
            break;
        }
        // Get the last element and place it at index i
        if let Some(last) = disk.pop() {
            disk[i] = last;
        }
    }

    for (i, &x) in disk.iter().enumerate() {
        checksum += i as i64 * x;
    }

    Ok(checksum)
}


fn p2(input_text: &str) -> Result<i64, Box<dyn Error>> {
    let mut checksum: i64 = 0;
    let nums: Vec<i64> = input_text
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as i64)
        .collect();

    let mut files = HashMap::new();
    let mut blanks = Vec::new();
    let mut fid = 0;
    let mut pos = 0;

    for (i, &x) in nums.iter().enumerate() {
        if i % 2 == 0 {
            if x == 0 {
                return Err("unexpected x=0 for file".into());
            }
            files.insert(fid, (pos, x));
            fid += 1;
        } else if x != 0 {
            blanks.push((pos, x));
        }
        pos += x;
    }

    while fid > 0 {
        fid -= 1;
        let (pos, size) = files[&fid];
        for (i, (start, length)) in blanks.iter().enumerate() {
            if *start >= pos {
                blanks.truncate(i);
                break;
            }
            if size <= *length {
                files.insert(fid, (*start, size));
                if size == *length {
                    blanks.remove(i);
                } else {
                    blanks[i] = (*start + size, *length - size);
                }
                break;
            }
        }
    }

    for (&fid, &(pos, size)) in files.iter() {
        for x in pos..(pos + size) {
            checksum += fid * x;
        }
    }

    Ok(checksum)
}

pub fn run(day: u8, level: Level, debug: bool) -> () {
    let example_input = "2333133121414131402";

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
