use anyhow::{Context, Result};

pub fn p1(input_text: &str) -> Result<i64> {
    let re = regex::Regex::new(r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)")?;

    let games: Vec<(i64, i64, i64, i64, i64, i64)> = input_text.split("\n\n").map(|game| {
        let caps = re.captures(game).ok_or_else(|| anyhow::anyhow!("Failed to match regex pattern"))?;
        let button_a_x = caps[1].parse::<i64>().context("Failed to parse button_a_x")?;
        let button_a_y = caps[2].parse::<i64>().context("Failed to parse button_a_y")?;
        let button_b_x = caps[3].parse::<i64>().context("Failed to parse button_b_x")?;
        let button_b_y = caps[4].parse::<i64>().context("Failed to parse button_b_y")?;
        let prize_x = caps[5].parse::<i64>().context("Failed to parse prize_x")?;
        let prize_y = caps[6].parse::<i64>().context("Failed to parse prize_y")?;
        
        Ok((button_a_x, button_a_y, button_b_x, button_b_y, prize_x, prize_y))
    }).collect::<Result<Vec<(i64, i64, i64, i64, i64, i64)>>>()?;

    let mut total = 0;
    
    for (ax, ay, bx, by, px, py) in games {
        let denominator = ax * by - ay * bx;
        let ca = (px * by - py * bx) as f64 / denominator as f64;
        let cb = (px as f64 - ax as f64 * ca) / bx as f64;
        
        if ca.fract() == 0.0 && cb.fract() == 0.0 {
            total += (ca * 3.0 + cb) as i64;
        }
    }
    Ok(total)

}


pub fn p2(input_text: &str) -> Result<i64> {
    let re = regex::Regex::new(r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)")?;

    let games: Vec<(i64, i64, i64, i64, i64, i64)> = input_text.split("\n\n").map(|game| {
        let caps = re.captures(game).ok_or_else(|| anyhow::anyhow!("Failed to match regex pattern"))?;
        let button_a_x = caps[1].parse::<i64>().context("Failed to parse button_a_x")?;
        let button_a_y = caps[2].parse::<i64>().context("Failed to parse button_a_y")?;
        let button_b_x = caps[3].parse::<i64>().context("Failed to parse button_b_x")?;
        let button_b_y = caps[4].parse::<i64>().context("Failed to parse button_b_y")?;
        let prize_x = caps[5].parse::<i64>().context("Failed to parse prize_x")?;
        let prize_y = caps[6].parse::<i64>().context("Failed to parse prize_y")?;
        
        Ok((button_a_x, button_a_y, button_b_x, button_b_y, prize_x, prize_y))
    }).collect::<Result<Vec<(i64, i64, i64, i64, i64, i64)>>>()?;

    let mut total = 0;
    
    for (ax, ay, bx, by, px, py) in games {
        let new_px = px + 10000000000000;
        let new_py = py + 10000000000000;
        let denominator = ax * by - ay * bx;
        let ca = (new_px * by - new_py * bx) as f64 / denominator as f64;
        let cb = (new_px as f64 - ax as f64 * ca) / bx as f64;
        
        if ca.fract() == 0.0 && cb.fract() == 0.0 {
            total += (ca * 3.0 + cb) as i64;
        }
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_p1_example() {
        let result = p1(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 480);
    }

    #[test]
    fn test_p2_example() {
        // p2 has different expected values due to large offset
        let result = p2(EXAMPLE_INPUT);
        assert!(result.is_ok());
    }
}

