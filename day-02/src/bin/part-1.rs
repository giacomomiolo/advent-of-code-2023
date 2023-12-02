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

fn is_game_possible(rounds: &[&str], red: i32, green: i32, blue: i32) -> bool {
    rounds.iter().all(|round| {
        let (mut round_red, mut round_green, mut round_blue) = (0, 0, 0);
        round.split("; ").for_each(|color_set| {
            color_set.split(", ").for_each(|color_count| {
                let parts: Vec<&str> = color_count.split_whitespace().collect();
                let count: i32 = parts[0].parse().unwrap();
                match parts[1] {
                    "red" => round_red += count,
                    "green" => round_green += count,
                    "blue" => round_blue += count,
                    _ => {}
                }
            });
        });
        round_red <= red && round_green <= green && round_blue <= blue
    })
}

fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let games = load_input()?;

    let mut sum_of_possible_game_ids = 0;
    for game in games {
        let parts: Vec<&str> = game.split(": ").collect();
        let game_id: i32 = parts[0].split_whitespace().nth(1).unwrap().parse()?;
        let rounds: Vec<&str> = parts[1].split("; ").collect();

        if is_game_possible(&rounds, 12, 13, 14) {
            sum_of_possible_game_ids += game_id;
        }
    }

    info!("Sum of possible game IDs: {}", sum_of_possible_game_ids);
    Ok(())
}
