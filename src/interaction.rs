use crate::{analyzer, game, global, mahjong::*};
use serde_json::json;

pub fn parse(arg: String) -> bool {
    let interactive = *global::INTERACTIVE.read().unwrap();
    match interactive {
        global::InteractiveState::Noninteractive => noninteractive_command_parse(arg),
        _ => interactive_command_parse(arg),
    }
}

fn noninteractive_command_parse(arg: String) -> bool {
    if arg == "interactive" || arg == "i" {
        // Make scope let interactive unlock its write lock.
        {
            let mut interactive = global::INTERACTIVE.write().unwrap();
            *interactive = global::InteractiveState::WaitForFirstInput;
        }
        // Make scope let game unlock its write lock.
        {
            let mut game = global::GAME.write().unwrap();
            game.initialize();
        }
    } else if arg == "noninteractive" || arg == "ni" {
    } else if arg == "exit" || arg == "quit" || arg == "q" {
        return true;
    } else if arg == "3pl" {
        let mut players_number = global::PLAYERS_NUMBER.write().unwrap();
        *players_number = global::Players::Three;
    } else if arg == "4pl" {
        let mut players_number = global::PLAYERS_NUMBER.write().unwrap();
        *players_number = global::Players::Four;
    } else {
        base_analyze(arg);
    }

    false
}

fn interactive_command_parse(arg: String) -> bool {
    if arg == "noninteractive" || arg == "ni" {
        // Make scope let players_number unlock its write lock.
        {
            let mut players_number = global::PLAYERS_NUMBER.write().unwrap();
            *players_number = global::Players::Three;
        }
        // Make scope let interactive unlock its write lock.
        {
            let mut interactive = global::INTERACTIVE.write().unwrap();
            *interactive = global::InteractiveState::Noninteractive;
        }
    } else if arg == "interactive" || arg == "i" {
        // Make scope let interactive unlock its write lock.
        {
            let mut interactive = global::INTERACTIVE.write().unwrap();
            *interactive = global::InteractiveState::WaitForFirstInput;
        }
        // Make scope let game unlock its write lock.
        {
            let mut game = global::GAME.write().unwrap();
            game.initialize();
        }
    } else if arg == "exit" || arg == "quit" || arg == "q" {
        return true;
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
    } else if arg == "status" || arg == "s" {
        let output_format = *global::OUTPUT_FORMAT.read().unwrap();
        match output_format {
            global::OutputFormat::Standard => {
                start_print();
                let game = global::GAME.read().unwrap();
                println!("{}", game.to_string());
            }
            global::OutputFormat::Json => {
                let game = global::GAME.read().unwrap();
                println!("{}", game.to_json());
            }
        }
    } else if arg == "b" || arg == "back" {
        let output_format = *global::OUTPUT_FORMAT.read().unwrap();
        match output_format {
            global::OutputFormat::Standard => {
                start_print();
                println!("Not implement yet.");
            }
            global::OutputFormat::Json => {
                println!("{}", json!({"error" : "Not implement yet."}));
            }
        }
    } else if arg == "b!" || arg == "back!" {
        let output_format = *global::OUTPUT_FORMAT.read().unwrap();
        match output_format {
            global::OutputFormat::Standard => {
                start_print();
                println!("Not implement yet.");
            }
            global::OutputFormat::Json => {
                println!("{}", json!({"error" : "Not implement yet."}));
            }
        }
    } else {
        let output_format = *global::OUTPUT_FORMAT.read().unwrap();
        if let Err(error) = interactive_input_analyze(arg) {
            match output_format {
                global::OutputFormat::Standard => {
                    start_print();
                    println!("{}", error);
                }
                global::OutputFormat::Json => {
                    println!("{}", json!({ "error": error }));
                }
            }
        }
        let state = *global::INTERACTIVE.read().unwrap();
        if let global::InteractiveState::FullTiles = state {
            let game = global::GAME.read().unwrap();
            let tehai = game.tehai();
            if let Some(tehai) = tehai {
                match analyzer::machi::analyze(tehai, Some(&game)) {
                    Ok((shanten, conditions)) => match output_format {
                        global::OutputFormat::Standard => {
                            start_print();
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
                                for i in conditions {
                                    println!("{}", i);
                                }
                            }
                        }
                        global::OutputFormat::Json => {
                            let mut condition_json_vec = vec![];
                            for condition in conditions {
                                condition_json_vec.push(condition.to_json());
                            }
                            println!(
                                "{}",
                                json!({ "tehai": tehai.to_json(),
                            "shanten": shanten,
                            "conditions": condition_json_vec })
                            );
                        }
                    },
                    Err(error) => match output_format {
                        global::OutputFormat::Standard => {
                            start_print();
                            println!("{}", error);
                        }
                        global::OutputFormat::Json => {
                            println!("{}", json!({ "error": error }));
                        }
                    },
                }
            } else {
                match output_format {
                    global::OutputFormat::Standard => {
                        start_print();
                        println!("Logic error: empty tiles while must be full tiles.");
                    }
                    global::OutputFormat::Json => {
                        println!(
                            "{}",
                            json!({"error" : "Logic error: empty tiles while must be full tiles."})
                        );
                    }
                }
            }
        }
    }
    false
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
                global::InteractiveState::Noninteractive => "NI",
                _ => "I",
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
                        for i in conditions {
                            println!("{}", i);
                        }
                    }
                }
                Err(error) => println!("{}", error),
            }
        }
        global::OutputFormat::Json => println!(
            "{}",
            match result {
                Ok((shanten, conditions)) => {
                    let mut condition_json_vec = vec![];
                    for i in conditions {
                        condition_json_vec.push(i.to_json());
                    }
                    json!({
                        "tehai": tehai.to_json(),
                        "shanten_number": shanten,
                        "conditions": condition_json_vec
                    })
                }
                Err(error) => json!({ "error": error }),
            }
        ),
    }
}

fn interactive_input_analyze(mut arg: String) -> Result<(), String> {
    let mut game = global::GAME.write().unwrap();
    if arg.starts_with("+") {
        arg.remove(0);
        let hai_vec = simple_input_parse(arg)?;
        if hai_vec.len() == 1 {
            let op = game::InteractiveOperation::Add(hai_vec[0]);
            game.operate(op)
        } else {
            Err(format!("Need 1 tile, get {}.", hai_vec.len()))
        }
    } else if arg.starts_with("-") {
        arg.remove(0);
        let hai_vec = simple_input_parse(arg)?;
        if hai_vec.len() == 1 {
            let op = game::InteractiveOperation::Discard(hai_vec[0]);
            game.operate(op)
        } else {
            Err(format!("Need 1 tile, get {}.", hai_vec.len()))
        }
    } else if arg.starts_with("*-") {
        arg.remove(0);
        let hai_vec = simple_input_parse(arg)?;
        let op = game::InteractiveOperation::HaiyamaDiscard(hai_vec);
        game.operate(op)
    } else if arg.starts_with("*+") {
        arg.remove(0);
        let hai_vec = simple_input_parse(arg)?;
        let op = game::InteractiveOperation::HaiyamaAdd(hai_vec);
        game.operate(op)
    } else if arg.starts_with("*!-") {
        Err("Not implement yet.".to_string())
    } else if arg.starts_with("*!+") {
        Err("Not implement yet.".to_string())
    } else if arg.starts_with(">!") {
        Err("Not implement yet.".to_string())
    } else if arg.starts_with(">") {
        arg.remove(0);
        let hai_vec = simple_input_parse(arg)?;
        let op;
        if hai_vec.len() == 3 {
            if let Some(mentsu) = analyzer::input::check_mentsu(&hai_vec) {
                match &mentsu {
                    Mentsu::Juntsu(_, _, _) => {
                        op = game::InteractiveOperation::Chii(mentsu, hai_vec[2]);
                    }
                    Mentsu::Koutsu(_) => {
                        op = game::InteractiveOperation::Pon(mentsu);
                    }
                    Mentsu::Kantsu(_) => {
                        return Err(
                            "Logic error: Kantsu detected while can not be a Kantsu.".to_string()
                        );
                    }
                }
            } else {
                return Err(format!("{:?} cannot be parsed as a valid mentsu.", hai_vec));
            }
        } else if hai_vec.len() == 4 {
            if let Some(mentsu) = analyzer::input::check_mentsu(&hai_vec) {
                match &mentsu {
                    Mentsu::Kantsu(_) => {
                        op = game::InteractiveOperation::Kan(mentsu, None);
                    }
                    _ => {
                        return Err("Logic error: Not a kantsu detected while must be a Kantsu."
                            .to_string());
                    }
                }
            } else {
                return Err(format!("{:?} cannot be parsed as a valid mentsu.", hai_vec));
            }
        } else if hai_vec.len() == 5 {
            let mut type_kantsu = None;
            let mut type_rinshan = None;
            let mut kantsu_ensure = false;
            for hai in hai_vec.iter() {
                match type_kantsu {
                    Some(kantsu_hai) => {
                        if kantsu_hai == *hai {
                            kantsu_ensure = true;
                        } else {
                            match type_rinshan {
                                Some(rinshanhai) => {
                                    if rinshanhai == *hai {
                                        if !kantsu_ensure {
                                            kantsu_ensure = true;
                                            std::mem::swap(&mut type_kantsu, &mut type_rinshan);
                                        } else {
                                            return Err(format!(
                                                "{:?} cannot be parsed as a valid mentsu.",
                                                hai_vec
                                            ));
                                        }
                                    } else {
                                        return Err(format!(
                                            "{:?} cannot be parsed as a valid mentsu.",
                                            hai_vec
                                        ));
                                    }
                                }
                                None => type_rinshan = Some(*hai),
                            }
                        }
                    }
                    None => type_kantsu = Some(*hai),
                }
            }
            if let (Some(kantsu_hai), Some(rinshanhai)) = (type_kantsu, type_rinshan) {
                op = game::InteractiveOperation::Kan(Mentsu::Kantsu(kantsu_hai), Some(rinshanhai));
            } else {
                return Err(format!("{:?} cannot be parsed as a valid mentsu.", hai_vec));
            }
        } else {
            return Err(format!("{:?} cannot be parsed as a valid mentsu.", hai_vec));
        }
        game.operate(op)
    } else if arg.starts_with("set!") {
        Err("Not implement yet.".to_string())
    } else {
        let mut hai_vec = simple_input_parse(arg)?;
        hai_vec.sort();
        if hai_vec.len() == 14 {
            let tehai = Tehai::new(Ok(hai_vec), vec![]);
            let op = game::InteractiveOperation::Initialize(tehai);
            game.operate(op)
        } else {
            Err("Can only accept 14 tiles on interactive mode.".to_string())
        }
    }
}

fn simple_input_parse(arg: String) -> Result<Vec<Hai>, String> {
    use Hai::*;

    let mut hai_vec = vec![];
    let mut hai_stash = vec![];

    fn push_into_hai_vec(
        tile_type: char,
        index: usize,
        stash: &mut Vec<char>,
        output: &mut Vec<Hai>,
    ) -> Result<(), String> {
        if stash.len() == 0 {
            Err(format!(
                "Unused type character '{}' at index {}.",
                tile_type, index
            ))
        } else {
            for tile in stash.iter() {
                let hai = match tile_type {
                    'm' => Manzu(*tile as u8 - 48),
                    'p' => Pinzu(*tile as u8 - 48),
                    's' => Souzu(*tile as u8 - 48),
                    'z' => Jihai(*tile as u8 - 48),
                    _ => Manzu(0), // Never reach here.
                };
                if analyzer::input::check_hai_in_range(&hai) {
                    output.push(hai);
                } else {
                    stash.clear();
                    return Err(format!("Out-of-range tile '{}' found.", hai.to_string()));
                }
            }
            stash.clear();
            Ok(())
        }
    };

    for (id, ch) in arg.chars().enumerate() {
        match ch {
            'm' | 'p' | 's' | 'z' => {
                push_into_hai_vec(ch, id, &mut hai_stash, &mut hai_vec)?;
            }
            '1'..='9' => hai_stash.push(ch),
            // Ignore all spaces.
            ' ' => (),
            _ => {
                return Err(format!("Unknown character '{}' at index {}.", ch, id));
            }
        }
    }

    if hai_stash.len() > 0 {
        return Err(format!(
            "No type specified for '{:?}' at the end of input string.",
            hai_stash
        ));
    }

    Ok(hai_vec)
}
