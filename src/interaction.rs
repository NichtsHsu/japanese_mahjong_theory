use crate::{analyzer, global, mahjong::*};

pub fn parse(arg: String) {
    let interactive = *global::INTERACTIVE.read().unwrap();
    match interactive {
        false => {
            noninteractive_parse(arg);
        }
        true => {
            interactive_parse(arg);
        }
    }
}

fn noninteractive_parse(arg: String) {
    if arg == "interactive" || arg == "i" {
        let mut interactive = global::INTERACTIVE.write().unwrap();
        *interactive = true;
    } else {
        base_analyze(arg);
    }
}

fn interactive_parse(arg: String) {
    if arg == "noninteractive" || arg == "ni" {
        let mut interactive = global::INTERACTIVE.write().unwrap();
        *interactive = false;
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
    start_print();
    if let Err(error) = analyzer::machi::analyze_and_print(&tehai, None) {
        println!("{}", error);
    }
}
