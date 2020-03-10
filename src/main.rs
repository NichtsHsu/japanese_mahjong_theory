mod analyzer;
mod game;
mod global;
mod interaction;
mod mahjong;

use clap::{App, Arg};
use std::{
    io::{stdin, stdout, Write},
    process,
};

fn main() {
    process::exit(match run() {
        Ok(_) => 0,
        Err(error) => {
            println!("{}", error);
            1
        }
    })
}

fn run() -> Result<(), String> {
    initialize()?;

    // Main loop
    loop {
        let mut input = String::new();
        let output_format = *global::OUTPUT_FORMAT.read().unwrap();
        match output_format {
            global::OutputFormat::Standard => {
                print!("<<< ");
                stdout().flush().unwrap();
            }
            global::OutputFormat::Json => (),
        }

        if let Err(_) = stdin().read_line(&mut input) {
            println!("Unable to read user input");
            continue;
        }
        if interaction::command_parse(input.trim().to_string()) {
            break Ok(());
        }
    }
}

fn initialize() -> Result<(), String> {
    // Handle program arguments.
    let matches = App::new("Japanese Mahjong Theory")
        .version("1.05")
        .author("Nichts Hsu <NichtsVonChaos@gmail.com>")
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .takes_value(true)
                .value_name("format_type")
                .help("Set output format: standard, json"),
        )
        .arg(
            Arg::with_name("players")
                .short("p")
                .long("player")
                .takes_value(true)
                .value_name("players_number")
                .help("Set players number, 3 or 4"),
        )
        .arg(
            Arg::with_name("interactive")
                .short("i")
                .long("interactive")
                .help("Start with interactive mode"),
        )
        .get_matches();

    let format_type = matches.value_of("format");
    if let Some(format_type) = format_type {
        if format_type == "standard" {
            global::OUTPUT_FORMAT
                .set(global::OutputFormat::Standard)
                .unwrap();
        } else if format_type == "json" {
            global::OUTPUT_FORMAT
                .set(global::OutputFormat::Json)
                .unwrap();
        } else {
            return Err(format!("Unknown format type called '{}'.", format_type));
        }
    } else {
        global::OUTPUT_FORMAT
            .set(global::OutputFormat::Standard)
            .unwrap();
    }

    let players_number = matches.value_of("players");
    if let Some(players_number) = players_number {
        if let Ok(players_number) = players_number.parse::<u32>() {
            match players_number {
                3 => global::PLAYERS_NUMBER.set(global::Players::Three).unwrap(),
                4 => global::PLAYERS_NUMBER.set(global::Players::Four).unwrap(),
                _ => {
                    return Err(format!(
                        "Not support {}-players mahjong yet.",
                        players_number
                    ));
                }
            };
        } else {
            return Err(format!(
                "Required players number, but get '{}'.",
                players_number
            ));
        }
    } else {
        global::PLAYERS_NUMBER.set(global::Players::Four).unwrap();
    }

    let interactive = match matches.is_present("interactive") {
        true => global::InteractiveState::WaitForFirstInput,
        false => global::InteractiveState::Noninteractive,
    };
    global::INTERACTIVE.set(interactive).unwrap();

    Ok(())
}
