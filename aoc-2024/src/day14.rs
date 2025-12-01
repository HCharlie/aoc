use anyhow::{Context, Result};

fn parse_dimensions(input_text: &str) -> Result<((i64, i64), &str)> {
    let mut lines = input_text.lines();
    let first_line = lines.next().ok_or_else(|| anyhow::anyhow!("Empty input"))?;
    
    if let Some(dims) = first_line.strip_prefix("DIMENSIONS: ") {
        let parts: Vec<&str> = dims.split_whitespace().collect();
        if parts.len() != 2 {
            anyhow::bail!("Invalid DIMENSIONS format");
        }
        let wide = parts[0].parse::<i64>()?;
        let tall = parts[1].parse::<i64>()?;
        let remaining = input_text.lines().skip(1).collect::<Vec<_>>().join("\n");
        Ok(((wide, tall), Box::leak(remaining.into_boxed_str())))
    } else {
        Err(anyhow::anyhow!("Missing DIMENSIONS header"))
    }
}

fn p1_impl(input_text: &str, wide: i64, tall: i64) -> Result<i64> {
    let re = regex::Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)")?;
    let bots: Vec<(i64, i64, i64, i64)> = input_text.lines()
        .map(|line| {
            let cap = re.captures(line)
                .ok_or_else(|| anyhow::anyhow!("Failed to match regex pattern"))?;
            let p1 = cap[1].parse::<i64>()
                .context("Failed to parse p1")?;
            let p2 = cap[2].parse::<i64>()
                .context("Failed to parse p2")?;
            let v1 = cap[3].parse::<i64>()
                .context("Failed to parse v1")?;
            let v2 = cap[4].parse::<i64>()
                .context("Failed to parse v2")?;
            Ok((p1, p2, v1, v2))
        })
        .collect::<Result<Vec<(i64, i64, i64, i64)>>>()?;
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


fn p2_impl(input_text: &str, wide: i64, tall: i64) -> Result<i64> {
    let re = regex::Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)")?;
    let bots: Vec<(i64, i64, i64, i64)> = input_text.lines()
        .map(|line| {
            let cap = re.captures(line)
                .ok_or_else(|| anyhow::anyhow!("Failed to match regex pattern"))?;
            let p1 = cap[1].parse::<i64>()
                .context("Failed to parse p1")?;
            let p2 = cap[2].parse::<i64>()
                .context("Failed to parse p2")?;
            let v1 = cap[3].parse::<i64>()
                .context("Failed to parse v1")?;
            let v2 = cap[4].parse::<i64>()
                .context("Failed to parse v2")?;
            Ok((p1, p2, v1, v2))
        })
        .collect::<Result<Vec<(i64, i64, i64, i64)>>>()?;
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

// Public wrappers for use with centralized orchestration
pub fn p1(input_text: &str) -> Result<i64> {
    let ((wide, tall), remaining) = parse_dimensions(input_text)?;
    p1_impl(remaining, wide, tall)
}

pub fn p2(input_text: &str) -> Result<i64> {
    let ((wide, tall), remaining) = parse_dimensions(input_text)?;
    p2_impl(remaining, wide, tall)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "DIMENSIONS: 11 7
p=0,4 v=3,-3
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

    #[test]
    fn test_p1_example() {
        let result = p1(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 12);
    }

    #[test]
    fn test_p2_example() {
        // p2 is about finding the easter egg pattern, just verify it runs
        let result = p2(EXAMPLE_INPUT);
        assert!(result.is_ok());
    }
}

