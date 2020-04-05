use super::Command;
use crate::game;
use serde_json::json;
use std::io::{stdout, Write};

#[derive(Clone, Debug)]
pub struct Controller {
    game_manager: Option<game::GameManager>,
    player_number: game::PlayerNumber,
    output_format: OutputFormat,
}

#[derive(Copy, Clone, Debug)]
pub enum OutputFormat {
    Standard,
    Json,
}

impl Controller {
    pub fn new(output_format: OutputFormat) -> Self {
        if let OutputFormat::Standard = output_format {
            print!(">>> ");
            stdout().flush().unwrap();
        }

        Self {
            game_manager: None,
            player_number: game::PlayerNumber::Four,
            output_format: OutputFormat::Standard,
        }
    }

    pub fn output_format(&self) -> OutputFormat {
        self.output_format
    }

    pub fn execute(&mut self, command: String, exit: &mut bool) {
        let result = self.execute_core(command, exit);

        match result {
            Ok(Some(output)) => {
                if !*exit {
                    if let OutputFormat::Standard = self.output_format {
                        println!(
                            "<<< [{},{}]",
                            self.player_number,
                            match &self.game_manager {
                                Some(_) => "I",
                                None => "NI",
                            }
                        )
                    }
                }
                println!("{}", output);
            }
            Err(error) => match self.output_format {
                OutputFormat::Standard => {
                    println!(
                        "<<< [{},{}]",
                        self.player_number,
                        match &self.game_manager {
                            Some(_) => "I",
                            None => "NI",
                        }
                    );
                    println!("{}", error);
                }
                OutputFormat::Json => println!("{}", json!({ "error": error })),
            },
            _ => (),
        }
        if !*exit {
            if let OutputFormat::Standard = self.output_format {
                print!(">>> ");
                stdout().flush().unwrap();
            }
        }
    }

    fn execute_core(&mut self, command: String, exit: &mut bool) -> Result<Option<String>, String> {
        fn print_machi(
            tehai: &game::Tehai,
            shanten: i32,
            conditions: Vec<game::MachiCondition>,
            format: OutputFormat,
        ) -> String {
            match format {
                OutputFormat::Standard => format!(
                    "手牌：{}\n{}",
                    tehai,
                    if shanten == -1 {
                        format!("和了")
                    } else {
                        let mut conditions_string = String::new();
                        for i in conditions {
                            conditions_string += &format!("\n{}", i);
                        }
                        format!(
                            "{}\n--------{}",
                            if shanten == 0 {
                                format!("聴牌")
                            } else {
                                format!("向聴：{}", shanten)
                            },
                            conditions_string
                        )
                    }
                ),
                OutputFormat::Json => {
                    let mut condition_json_vec = vec![];
                    for i in conditions {
                        condition_json_vec.push(i.to_json());
                    }
                    json!({
                        "tehai": tehai.to_json(),
                        "shanten_number": shanten,
                        "conditions": condition_json_vec
                    })
                    .to_string()
                }
            }
        }

        *exit = false;
        let command = Command::parse(command, self.player_number)?;
        match command {
            Command::Exit => *exit = true,
            Command::Noninteractive => self.game_manager = None,
            Command::Interactive => {
                self.game_manager = Some(game::GameManager::new(self.player_number))
            }
            Command::OutputFormat(output_format) => self.output_format = output_format,
            Command::PlayerNumber(player_number) => {
                self.player_number = player_number;
                if let Some(game_manager) = &mut self.game_manager {
                    game_manager.reinitialize(player_number);
                }
            }
            Command::GameOperation(op) => match &mut self.game_manager {
                Some(game_manager) => {
                    game_manager.operate(op)?;
                    if let game::State::FullHai = game_manager.state {
                        let tehai = game_manager.tehai().ok_or("Not initialized.".to_string())?;
                        let (shanten, conditions) = game_manager.tehai_analyze()?;
                        return Ok(Some(print_machi(
                            &tehai,
                            shanten,
                            conditions,
                            self.output_format,
                        )));
                    }
                }
                None => {
                    return Err(
                        "Can not execute interactive command at non-interactive mode.".to_string(),
                    );
                }
            },
            Command::TehaiInput(tehai) => match &mut self.game_manager {
                Some(game_manager) => {
                    game_manager.operate(game::Operation::Tehai(
                        game::TehaiOperation::Initialize(tehai),
                    ))?;
                    if let game::State::FullHai = game_manager.state {
                        let tehai = game_manager.tehai().ok_or("Not initialized.".to_string())?;
                        let (shanten, conditions) = game_manager.tehai_analyze()?;
                        return Ok(Some(print_machi(
                            &tehai,
                            shanten,
                            conditions,
                            self.output_format,
                        )));
                    }
                }
                None => {
                    let (shanten, conditions) = tehai.analyze(self.player_number, None)?;
                    return Ok(Some(print_machi(
                        &tehai,
                        shanten,
                        conditions,
                        self.output_format,
                    )));
                }
            },
            Command::Back { haiyama_sensitive } => {}
            Command::State => match &self.game_manager {
                Some(game_manager) => {
                    return Ok(Some(format!(
                        "{}",
                        match self.output_format {
                            OutputFormat::Standard => game_manager.to_string(),
                            OutputFormat::Json => game_manager.to_json().to_string(),
                        }
                    )))
                }
                None => {
                    return Err(
                        "Can not execute interactive command at non-interactive mode.".to_string(),
                    );
                }
            },
        };
        Ok(None)
    }
}
