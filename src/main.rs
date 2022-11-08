#![forbid(unsafe_code)]

mod game;
mod interaction;
use clap::Parser;
use std::{io::stdin, process};

#[derive(Parser, Debug)]
#[command(name = "Japanese Mahjong Theory Shell")]
#[command(author = "Nichts Hsu <NichtsVonChaos@gmail.com>")]
#[command(version = "1.18")]
#[command(about = "Japanese Mahjong Theory Shell", long_about = None)]
struct Args {
    #[arg(short, long, help = "Set output format: standard | json", default_value_t = String::from("standard"))]
    format_type: String,
    #[arg(short, long, help = "Set players number: 3 | 4", default_value_t = 4)]
    players_number: u8,
    #[arg(short, help = "Start with interactive mode", long)]
    interactive: bool,
}

fn main() {
    process::exit(match run_application() {
        Ok(_) => 0,
        Err(_) => 1,
    })
}

fn run_application() -> Result<(), ()> {
    // Handle program arguments.
    let args = Args::parse();

    let output_format = if args.format_type == "standard" {
        interaction::OutputFormat::Standard
    } else if args.format_type == "json" {
        interaction::OutputFormat::Json
    } else {
        println!("Unknown format type: {}.", args.format_type);
        return Err(());
    };

    let player_number = match args.players_number {
        3 => game::PlayerNumber::Three,
        4 => game::PlayerNumber::Four,
        _ => {
            println!("Not support {}-players mode.", args.players_number);
            return Err(());
        }
    };

    let interactive = args.interactive;

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
