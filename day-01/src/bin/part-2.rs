use anyhow::{Result, anyhow};
use std::fs;
use tracing::{info, warn};
use tracing_subscriber;
use aho_corasick::AhoCorasick;

fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    let lines = load_input()?;
    let sum: u32 = lines.iter()
                        .filter_map(|line| find_line_calibration_value(line).ok())
                        .sum();

    info!("Sum: {}", sum);
    Ok(())
}

fn load_input() -> Result<Vec<String>> {
    info!("Loading input from file");
    let input = fs::read_to_string("input-2023-01.txt")?;
    let lines: Vec<String> = input.lines().map(String::from).collect();
    info!("Loaded {} lines from file", lines.len());
    
    Ok(lines)
}

fn find_first_and_last_digits_in_string(s: &str) -> Result<(u32, u32), anyhow::Error> {
    let digit_valid_pattern: Vec<&str> = vec![
        "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7", "eight", "8", "nine", "9",
    ];

    let ac = AhoCorasick::new(&digit_valid_pattern).unwrap();

    let matches = ac.find_overlapping_iter(s).collect::<Vec<_>>();
    // Divide by 2 because the pattern is duplicated in the vector and add 1 because the pattern is 0-indexed
    let first_digit = matches.iter().nth(0).unwrap().pattern().as_usize() / 2 + 1;
    let last_digit = matches.iter().last().unwrap().pattern().as_usize() / 2 + 1;
    info!("{} -> first: {} -> last: {}", s, first_digit, last_digit);

    return Ok((first_digit as u32, last_digit as u32));
}

fn find_line_calibration_value(line: &str) -> Result<u32> {
    let (first_digit, last_digit) = find_first_and_last_digits_in_string(line)?;

    match (first_digit, last_digit) {
        (0, 0) => {
            warn!("No digit found in line: '{}'", line);
            Err(anyhow!("No digit found"))
        },
        (0, _) => Ok(last_digit * 11),
        (_, 0) => Ok(first_digit * 11),
        _ => Ok(first_digit * 10 + last_digit),
    }
}
