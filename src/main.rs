use std::fs::read_to_string;

use tokei::{Config, Languages};

const BANANA_LENGTH: f32 = 177.8;
const LINE_HEIGHT: f32 = 4.212;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let paths = &["."];

    let excluded = read_to_string(".gitignore")?
        .lines()
        .map(|line| line.trim().to_string())
        .collect::<Vec<String>>();

    let config = Config::default();

    let mut languages = Languages::new();
    languages.get_statistics(
        paths,
        &excluded
            .iter()
            .map(|line| line.as_str())
            .collect::<Vec<&str>>(),
        &config,
    );

    let count: usize = languages
        .iter()
        .map(|(_type, language)| language.code)
        .sum();

    println!(
        "Your code is {} 🍌 for scale",
        ((count as f32) * LINE_HEIGHT / BANANA_LENGTH)
    );
    Ok(())
}
