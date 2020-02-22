mod analyzer;
mod mahjong;

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    println!("{}", mahjong::Tehai::from(input.trim().to_string()));
}