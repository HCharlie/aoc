use anyhow::Result;

pub const EXAMPLE_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

pub fn p1(input_text: &str) -> Result<i32> {
    let mut curr = 50;
    let circle_len = 100;
    let mut total_zeros: i32 = 0;
    for line in input_text.lines() {
        if line.is_empty() {
            continue;
        }
        let (turn, dist) = line.split_at(1);
        let dist: i32 = dist.parse()?;
        match turn {            
            "L" => {
                curr = (curr + circle_len - dist) % circle_len;
            }
            "R" => {
                curr = (curr + dist) % circle_len;
            }
            _ => anyhow::bail!("Invalid turn direction: {}", turn),
        }
        if curr == 0 {
            total_zeros += 1;
        }
    }

    Ok(total_zeros)
}

pub fn p2(input_text: &str) -> Result<i32> {
    println!("Day 1 Part 2 not implemented yet.");
    for line in input_text.lines() {
        println!("{}", line);
    }
    
    Ok(0)

}