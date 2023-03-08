#![forbid(unsafe_code)]

mod game;
mod interaction;
use clap::Parser;
use rustyline::{self, DefaultEditor, error::ReadlineError};

#[derive(Parser, Debug)]
#[command(name = "Japanese Mahjong Theory Shell")]
#[command(author = "Nichts Hsu <NichtsVonChaos@gmail.com>")]
#[command(version = "1.19")]
#[command(about = "Japanese Mahjong Theory Shell", long_about = None)]
struct Args {
    #[arg(short, long, help = "Set output format: standard | json", default_value_t = String::from("standard"))]
    format_type: String,
    #[arg(short, long, help = "Set players number: 3 | 4", default_value_t = 4)]
    players_number: u8,
    #[arg(short, help = "Start with interactive mode", long)]
    interactive: bool,
}

fn main() -> Result<(), String> {
    // Handle program arguments.
    let args = Args::parse();

    let output_format = if args.format_type == "standard" {
        interaction::OutputFormat::Standard
    } else if args.format_type == "json" {
        interaction::OutputFormat::Json
    } else {
        return Err(format!("Unknown format type: {}.", args.format_type));
    };

    let player_number = match args.players_number {
        3 => game::PlayerNumber::Three,
        4 => game::PlayerNumber::Four,
        _ => return Err(format!("Not support {}-players mode.", args.players_number)),
    };

    let interactive = args.interactive;

    // Initialize controller.
    let mut controller = interaction::Controller::new(output_format, player_number, interactive);

    // Initialize RustyLine.
    let mut rl =
        DefaultEditor::new().map_err(|e| format!("Failed to initialize RustyLine: {e}."))?;

    // Main loop
    loop {
        let prompt = match controller.output_format() {
            interaction::OutputFormat::Standard => ">>> ",
            interaction::OutputFormat::Json => "",
        };
        match rl.readline(prompt) {
            Ok(input) => {
                _ = rl.add_history_entry(input.as_str());
                let mut exit = false;
                controller.execute(input.trim().to_string(), &mut exit);
                if exit {
                    break Ok(());
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Ctrl-C detected, program exited.");
                break Ok(());
            },
            Err(ReadlineError::Eof) => {
                println!("Ctrl-D detected, program exited.");
                break Ok(());
            },
            Err(_) => {
                break Err(String::from(match controller.output_format() {
                    interaction::OutputFormat::Standard => "Failed to read input.",
                    interaction::OutputFormat::Json => "{\"error\":\"Failed to read input.\"}",
                }));
            }
        }
    }
}
