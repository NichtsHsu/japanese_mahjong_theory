#![forbid(unsafe_code)]

mod game;
mod interaction;
use clap::{App, Arg};
use std::{io::stdin, process};

fn main() {
    process::exit(match run_application() {
        Ok(_) => 0,
        Err(_) => 1,
    })
}

fn run_application() -> Result<(), ()> {
    // Handle program arguments.
    let matches = App::new("Japanese Mahjong Theory")
        .version("1.18")
        .author("Nichts Hsu <NichtsVonChaos@gmail.com>")
        .arg(
            Arg::with_name("format")
                .short('f')
                .long("format")
                .takes_value(true)
                .value_name("format_type")
                .help("Set output format: standard, json"),
        )
        .arg(
            Arg::with_name("players")
                .short('p')
                .long("player")
                .takes_value(true)
                .value_name("players_number")
                .help("Set players number, 3 or 4"),
        )
        .arg(
            Arg::with_name("interactive")
                .short('i')
                .long("interactive")
                .help("Start with interactive mode"),
        )
        .get_matches();

    let output_format = if let Some(format_type) = matches.value_of("format") {
        if format_type == "standard" {
            interaction::OutputFormat::Standard
        } else if format_type == "json" {
            interaction::OutputFormat::Json
        } else {
            println!("Unknown format type: {}.", format_type);
            return Err(());
        }
    } else {
        interaction::OutputFormat::Standard
    };

    let player_number = if let Some(players_number) = matches.value_of("players") {
        if let Ok(players_number) = players_number.parse::<u32>() {
            match players_number {
                3 => game::PlayerNumber::Three,
                4 => game::PlayerNumber::Four,
                _ => {
                    println!("Not support {}-players mode.", players_number);
                    return Err(());
                }
            }
        } else {
            println!("Unparsed argument: {}.", players_number);
            return Err(());
        }
    } else {
        game::PlayerNumber::Four
    };

    let interactive = matches.is_present("interactive");

    // Initialize controller.
    let mut controller = interaction::Controller::new(output_format, player_number, interactive);

    // Main loop
    loop {
        let mut input = String::new();
        if let Err(_) = stdin().read_line(&mut input) {
            println!(
                "{}",
                match controller.output_format() {
                    interaction::OutputFormat::Standard => "Failed to read input.",
                    interaction::OutputFormat::Json => "{\"error\":\"Failed to read input.\"}",
                }
            );
            break Err(());
        }
        let mut exit = false;
        controller.execute(input.trim().to_string(), &mut exit);
        if exit {
            break Ok(());
        }
    }
}
