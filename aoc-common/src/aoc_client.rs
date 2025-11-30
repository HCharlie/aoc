use anyhow::{Context, Result};
use reqwest;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn get_input_content(year: u16, day: u8) -> Result<String> {
    let file_path = format!("aoc-{}/data/day{:02}.txt", year, day);

    if !Path::new(&file_path).exists() {
        download_input_data(year, day, &file_path)
            .context(format!("Failed to download input for year {} day {}", year, day))?;
    }

    let content = std::fs::read_to_string(&file_path)
        .context(format!("Failed to read input file: {}", file_path))?;
    Ok(content.trim().to_string())
}

pub fn submit_check_answer(
    year: u16,
    day: u8,
    level: u8,
    answer: &str,
) -> Result<bool> {
    let url = format!("https://adventofcode.com/{}/day/{}/answer", year, day);
    let cookie = env::var("AOC_COOKIE").unwrap_or(get_session_token()?);
    let client = reqwest::blocking::Client::new();
    let params = [("level", level.to_string()), ("answer", answer.to_string())];

    let response = client
        .post(&url)
        .header("Cookie", &cookie)
        .form(&params)
        .send()
        .context("Failed to submit answer")?
        .text()
        .context("Failed to read response")?;
    
    // only print the content of the main tag
    if let Some(start) = response.find("<main>") {
        if let Some(end) = response.find("</main>") {
            let content = &response[start + 6..end];
            println!("content: {}", content);
        }
    }
    println!("Request sent to {} with params {:?}", url, params);
    Ok(response.contains("That's the right answer!")
        || response.contains("You don't seem to be solving the right level."))
}

fn get_session_token() -> Result<String> {
    // Try to get from environment variable first
    if let Ok(token) = env::var("AOC_TOKEN") {
        return Ok(token);
    }

    // Fallback to reading from a local file
    if let Ok(token) = std::fs::read_to_string(".aoc_token") {
        return Ok(token.trim().to_string());
    }
    anyhow::bail!("AOC_TOKEN environment variable or .aoc_token file required")
}

fn download_input_data(year: u16, day: u8, file_path: &str) -> Result<()> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let cookie = env::var("AOC_TOKEN").unwrap_or(get_session_token()?);

    let client = reqwest::blocking::Client::new();
    let response = client.get(&url)
        .header("Cookie", &cookie)
        .send()
        .context("Failed to download input data")?
        .text()
        .context("Failed to read response text")?;
    
    let mut file = File::create(file_path)
        .context(format!("Failed to create file: {}", file_path))?;
    file.write_all(response.as_bytes())
        .context("Failed to write input data to file")?;

    Ok(())
}
