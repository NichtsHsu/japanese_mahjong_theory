mod analyzer;
mod mahjong;

use std::io;

fn main() -> Result<(), String> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    let tehai = mahjong::Tehai::from(input.trim().to_string());
    analyzer::machi::analyze_and_print(&tehai, None)?;
    Ok(())
}
