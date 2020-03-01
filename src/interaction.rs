use crate::{analyzer, global, mahjong::*};

pub fn parse(arg: String) -> bool {
    let interactive = *global::INTERACTIVE.read().unwrap();
    match interactive {
        false => noninteractive_parse(arg),
        true => interactive_parse(arg),
    }
}

fn noninteractive_parse(arg: String) -> bool {
    if arg == "interactive" || arg == "i" {
        let mut interactive = global::INTERACTIVE.write().unwrap();
        *interactive = true;
        false
    } else if arg == "exit" || arg == "quit" || arg == "q" {
        true
    } else if arg == "3pl" {
        let mut players_number = global::PLAYERS_NUMBER.write().unwrap();
        *players_number = global::Players::Three;
        false
    } else if arg == "4pl" {
        let mut players_number = global::PLAYERS_NUMBER.write().unwrap();
        *players_number = global::Players::Four;
        false
    } else {
        base_analyze(arg);
        false
    }
}

fn interactive_parse(arg: String) -> bool {
    if arg == "noninteractive" || arg == "ni" {
        // Make scope let players_number unlock its write lock.
        {
            let mut players_number = global::PLAYERS_NUMBER.write().unwrap();
            *players_number = global::Players::Three;
        }
        // Make scope let interactive unlock its write lock.
        {
            let mut interactive = global::INTERACTIVE.write().unwrap();
            *interactive = false;
        }
        false
    } else if arg == "exit" || arg == "quit" || arg == "q" {
        true
    } else if arg == "3pl" {
        // Make scope let players_number unlock its write lock.
        {
            let mut players_number = global::PLAYERS_NUMBER.write().unwrap();
            *players_number = global::Players::Three;
        }
        // Make scope let game unlock its write lock.
        {
            let mut game = global::GAME.write().unwrap();
            game.initialize();
        }
        false
    } else if arg == "4pl" {
        // Make scope let players_number unlock its write lock.
        {
            let mut players_number = global::PLAYERS_NUMBER.write().unwrap();
            *players_number = global::Players::Four;
        }
        // Make scope let game unlock its write lock.
        {
            let mut game = global::GAME.write().unwrap();
            game.initialize();
        }
        false
    } else {
        false
    }
}

fn start_print() {
    let output_format = *global::OUTPUT_FORMAT.read().unwrap();
    let interactive = *global::INTERACTIVE.read().unwrap();
    let players_number = *global::PLAYERS_NUMBER.read().unwrap();
    if let global::OutputFormat::Standard = output_format {
        println!(
            ">>> [{},{}]",
            players_number.to_string(),
            match interactive {
                true => "I",
                false => "NI",
            }
        )
    }
}

fn base_analyze(arg: String) {
    let tehai = Tehai::from(arg);
    let output_format = *global::OUTPUT_FORMAT.read().unwrap();
    let result = analyzer::machi::analyze(&tehai, None);
    match output_format {
        global::OutputFormat::Standard => {
            start_print();
            match result {
                Ok((shanten, conditions)) => {
                    println!("手牌：{}", tehai);
                    if shanten == -1 {
                        println!("和了");
                    } else {
                        if shanten == 0 {
                            println!("聴牌");
                        } else {
                            println!("向聴：{}", shanten);
                        }
                        println!("--------");
                        for i in conditions.iter() {
                            println!("{}", i);
                        }
                    }
                }
                Err(error) => println!("{}", error),
            }
        }
        global::OutputFormat::Json => {}
    }
}
