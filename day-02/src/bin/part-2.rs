use anyhow::Result;
use tracing::{info, Level};
use tracing_subscriber;
use std::fs;

fn load_input() -> Result<Vec<String>> {
    info!("Loading input from file");
    let input = fs::read_to_string("input-2023-02.txt")?;
    let lines: Vec<String> = input.lines().map(String::from).collect();
    info!("Loaded {} lines from file", lines.len());
    
    Ok(lines)
}

fn min_cubes_per_game(rounds: &[&str]) -> (i32, i32, i32) {
    let (mut max_red, mut max_green, mut max_blue) = (0, 0, 0);

    for round in rounds {
        let (mut round_red, mut round_green, mut round_blue) = (0, 0, 0);
        round.split("; ").for_each(|color_set| {
            color_set.split(", ").for_each(|color_count| {
                let parts: Vec<&str> = color_count.split_whitespace().collect();
                let count: i32 = parts[0].parse().unwrap();
                match parts[1] {
                    "red" => round_red = round_red.max(count),
                    "green" => round_green = round_green.max(count),
                    "blue" => round_blue = round_blue.max(count),
                    _ => {}
                }
            });
        });
        max_red = max_red.max(round_red);
        max_green = max_green.max(round_green);
        max_blue = max_blue.max(round_blue);
    }

    (max_red, max_green, max_blue)
}

fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let games = load_input()?;
    let mut sum_of_powers = 0;

    for game in games {
        let parts: Vec<&str> = game.split(": ").collect();
        let rounds: Vec<&str> = parts[1].split("; ").collect();

        let (min_red, min_green, min_blue) = min_cubes_per_game(&rounds);
        let power = min_red * min_green * min_blue;
        sum_of_powers += power;

        info!("Game: {}, Minimum Cubes: Red: {}, Green: {}, Blue: {}, Power: {}", parts[0], min_red, min_green, min_blue, power);
    }

    info!("Sum of the power of these sets: {}", sum_of_powers);
    Ok(())
}
