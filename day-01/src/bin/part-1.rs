use anyhow::Result;
use std::fs;
use tracing::info;
use tracing_subscriber;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let lines = load_input()?;
    let sum: u32 = lines.iter()
                        .map(|line| find_line_calibration_value(line))
                        .sum::<Result<u32>>()?;

    println!("Sum: {}", sum);

    Ok(())
}

fn load_input() -> Result<Vec<String>> {
    info!("Loading input from file");
    let input = fs::read_to_string("input-2023-01.txt")?;
    let lines: Vec<String> = input.lines().map(String::from).collect();
    info!("Loaded {} lines from file", lines.len());
    
    Ok(lines)
}

fn find_all_digits_in_string(s: &str) -> Result<Vec<u32>> {
    let re = Regex::new(r"\d")?;
    
    let digits: Vec<u32> = re.find_iter(s)
                             .filter_map(|m| m.as_str().parse::<u32>().ok())
                             .collect();

    Ok(digits)
}

fn find_line_calibration_value(line: &str) -> Result<u32> {
    // If there is only one digit in the string, multiply it by 11
    // If there are two digits in the string, multiply by 10 the first digit and add the second digit

    let digits = find_all_digits_in_string(line)?;
    
    match digits.len() {
        1 => Ok(digits[0] * 11),
        _ => {
            let sum = digits.first().unwrap() * 10 + digits.last().unwrap();
            Ok(sum)
        }
    }
}