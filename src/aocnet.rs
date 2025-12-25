use std::env;
use std::error::Error;

const YEAR: u32 = 2025;

pub fn fetch_input(day: u32, session: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", YEAR, day);

    let client = reqwest::blocking::Client::new();
    let resp = client
        .get(url)
        .header(reqwest::header::COOKIE, format!("session={}", session))
        .header(
            reqwest::header::USER_AGENT,
            format!("github.com/{}/aoc{} (Rust client)", get_username(), YEAR),
        )
        .send()?;

    let status = resp.status();
    if !status.is_success() {
        return Err(format!("failed to fetch input: status {}", status.as_u16()).into());
    }

    let text = resp.text()?;

    // Behaves like your Go: splits on '\n', trims trailing '\r', keeps empty lines if they exist
    let lines = text
        .split_terminator('\n')
        .map(|l| l.trim_end_matches('\r').to_string())
        .collect::<Vec<_>>();

    Ok(lines)
}

fn get_username() -> String {
    env::var("USER")
        .or_else(|_| env::var("USERNAME"))
        .unwrap_or_else(|_| "anonymous".to_string())
}