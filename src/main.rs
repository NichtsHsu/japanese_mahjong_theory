mod analyzer;
mod mahjong;

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    println!("{}", analyzer::parse_input_tiles(input.trim().to_string()).to_string());
}
