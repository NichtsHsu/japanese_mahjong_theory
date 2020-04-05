#![feature(bindings_after_at)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod game;
mod interaction;

use std::{io::stdin, process};

fn main() {
    process::exit(match run_application() {
        Ok(_) => 0,
        Err(_) => 1,
    })
}

fn run_application() -> Result<(), ()> {
    let mut controller = interaction::Controller::new(interaction::OutputFormat::Standard);

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
