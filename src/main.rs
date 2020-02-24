mod analyzer;
mod mahjong;

use std::io;

fn main() -> Result<(), String> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    let tehai = mahjong::Tehai::from(input.trim().to_string());
    let (shanten, conditions) = analyzer::machi::analyze(&tehai, None)?;
    println!("--------");
    println!("手牌：{}", tehai);
    println!("向听：{}", shanten);
    println!("--------");
    for i in conditions.iter() {
        println!("{}", i);
    }
    Ok(())
}
