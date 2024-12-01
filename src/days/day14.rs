use super::utils::{get_input_content, submit_check_answer};
use crate::Level;
use std::error::Error;


fn p1(input_text: &str, wide: i64, tall: i64) -> Result<i64, Box<dyn Error>> {
    let re = regex::Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)")?;
    let bots: Vec<(i64, i64, i64, i64)> = input_text.lines()
        .map(|line| {
            let cap = re.captures(line)
                .ok_or("Failed to match regex pattern")?;
            let p1 = cap[1].parse::<i64>()
                .map_err(|_| "Failed to parse p1")?;
            let p2 = cap[2].parse::<i64>()
                .map_err(|_| "Failed to parse p2")?;
            let v1 = cap[3].parse::<i64>()
                .map_err(|_| "Failed to parse v1")?;
            let v2 = cap[4].parse::<i64>()
                .map_err(|_| "Failed to parse v2")?;
            Ok((p1, p2, v1, v2))
        })
        .collect::<Result<Vec<(i64, i64, i64, i64)>, Box<dyn Error>>>()?;
    let time = 100;
    let mut quadrants = [0i64; 4];
    for i in 0..bots.len() {
        let (x, y, vx, vy) = bots[i];
        println!("initial position {} {}", x, y);
        let x = (x + vx * time).rem_euclid(wide);
        let y = (y + vy * time).rem_euclid(tall);
        println!("after 100s movement {} {}", x, y);
        if x >= 0 && x < wide/2 && y >= 0 && y < tall/2 {
            quadrants[0] += 1;
        } else if x > wide/2 && x < wide && y >= 0 && y < tall/2 {
            quadrants[1] += 1;
        } else if x >= 0 && x < wide/2 && y > tall/2 && y < tall {
            quadrants[2] += 1;
        } else if x > wide/2 && x < wide && y > tall/2 && y < tall {
            quadrants[3] += 1;
        }
    }
    println!("{:?}", quadrants);
    Ok(quadrants.iter()
        .filter(|&&x| x > 0)
        .product())
}


fn p2(input_text: &str, wide: i64, tall: i64) -> Result<i64, Box<dyn Error>> {
    let re = regex::Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)")?;
    let bots: Vec<(i64, i64, i64, i64)> = input_text.lines()
        .map(|line| {
            let cap = re.captures(line)
                .ok_or("Failed to match regex pattern")?;
            let p1 = cap[1].parse::<i64>()
                .map_err(|_| "Failed to parse p1")?;
            let p2 = cap[2].parse::<i64>()
                .map_err(|_| "Failed to parse p2")?;
            let v1 = cap[3].parse::<i64>()
                .map_err(|_| "Failed to parse v1")?;
            let v2 = cap[4].parse::<i64>()
                .map_err(|_| "Failed to parse v2")?;
            Ok((p1, p2, v1, v2))
        })
        .collect::<Result<Vec<(i64, i64, i64, i64)>, Box<dyn Error>>>()?;
    let time = wide * tall * 2;
    let mut min_sf = i64::MAX;
    let mut best_iteration = 0;
    for seconds in 0..time {
        let mut quadrants = [0i64; 4];
        for i in 0..bots.len() {
            let (x, y, vx, vy) = bots[i];
            let x = (x + vx * seconds).rem_euclid(wide);
            let y = (y + vy * seconds).rem_euclid(tall);
            if x >= 0 && x < wide/2 && y >= 0 && y < tall/2 {
                quadrants[0] += 1;
            } else if x > wide/2 && x < wide && y >= 0 && y < tall/2 {
                quadrants[1] += 1;
            } else if x >= 0 && x < wide/2 && y > tall/2 && y < tall {
                quadrants[2] += 1;
            } else if x > wide/2 && x < wide && y > tall/2 && y < tall {
                quadrants[3] += 1;
            }
        }
        let current_sf = quadrants.iter()
            .filter(|&&x| x > 0)
            .product();
        if min_sf > current_sf {
            min_sf = current_sf;
            best_iteration = seconds;
        }
    }
    Ok(best_iteration)
}

pub fn run(day: u8, level: Level, debug: bool) -> () {

    let example_input = 
"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
    let example_bathroom_tiles_wide = 11;
    let example_bathroom_tiles_tall = 7;

    let bathroom_tiles_wide = 101;
    let bathroom_tiles_tall = 103;

    let sol_func = match level {
        Level::One => p1,
        Level::Two => p2,
    };

    match sol_func(example_input, example_bathroom_tiles_wide, example_bathroom_tiles_tall) {
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

    let answer = match sol_func(&content, bathroom_tiles_wide, bathroom_tiles_tall) {
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
