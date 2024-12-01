use reqwest;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn get_input_content(day: u8) -> Result<String, Box<dyn Error>> {
    let file_path = format!("src/days/data/day{:02}.txt", day);

    if !Path::new(&file_path).exists() {
        download_input_data(day, &file_path)?;
    }

    let content = std::fs::read_to_string(&file_path)?;
    Ok(content.trim().to_string())
}


fn get_session_token() -> Result<String, Box<dyn Error>> {
    // Try to get from environment variable first
    if let Ok(token) = env::var("AOC_TOKEN") {
        return Ok(token);
    }

    // Fallback to reading from a local file
    if let Ok(token) = std::fs::read_to_string(".aoc_token") {
        return Ok(token.trim().to_string());
    }
    return Err(".aoc_token file required".into());
}

fn download_input_data(day: u8, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://adventofcode.com/2024/day/{}/input", day);
    let cookie = env::var("AOC_TOKEN").unwrap_or(get_session_token()?);

    let client = reqwest::blocking::Client::new();
    let response = client.get(&url).header("Cookie", &cookie).send()?.text()?;
    let mut file = File::create(file_path)?;
    file.write_all(response.as_bytes())?;

    Ok(())
}

pub fn submit_check_answer(
    day: u8,
    level: u8,
    answer: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let url = format!("https://adventofcode.com/2024/day/{}/answer", day);
    let cookie = env::var("AOC_COOKIE").unwrap_or(get_session_token()?);
    let client = reqwest::blocking::Client::new();
    let params = [("level", level.to_string()), ("answer", answer.to_string())];

    let response = client
        .post(&url)
        .header("Cookie", &cookie)
        .form(&params)
        .send()?
        .text()?;
    
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
