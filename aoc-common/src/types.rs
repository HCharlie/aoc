use anyhow::{anyhow, Result};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Level {
    One = 1,
    Two = 2,
}

impl FromStr for Level {
    type Err = anyhow::Error;
    
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "1" => Ok(Level::One),
            "2" => Ok(Level::Two),
            _ => Err(anyhow!("Invalid level '{}'. Must be 1 or 2", s)),
        }
    }
}

