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
            Command::Back { haiyama_sensitive } => match &mut self.game_manager {
                Some(game_manager) => {
                    let (op, state) = game_manager.back(haiyama_sensitive)?;
                    return Ok(Some(format!(
                        "Undo operation: {:?}\nBack to state: {:?}",
                        op, state
                    )));
                }
                None => {
                    return Err(
                        "Can not execute interactive command at non-interactive mode.".to_string(),
                    );
                }
            },
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
            Command::Display => match &self.game_manager {
                Some(game_manager) => {
                    if let game::State::FullHai = game_manager.state {
                        let tehai = game_manager.tehai().ok_or("Not initialized.".to_string())?;
                        let (shanten, conditions) = game_manager.tehai_analyze()?;
                        return Ok(Some(print_machi(
                            &tehai,
                            shanten,
                            conditions,
                            self.output_format,
                        )));
                    } else {
                        return Err("Can only analyze tehai when full with hai.".to_string());
                    }
                }
                None => {
                    return Err(
                        "Can not execute interactive command at non-interactive mode.".to_string(),
                    );
                }
            },
            Command::History => match &self.game_manager {
                Some(game_manager) => {
                    let iter = game_manager.history().iter();
                    match self.output_format {
                        OutputFormat::Standard => {
                            let mut string = String::from("Operation History");
                            for (id, (op, state)) in iter.enumerate() {
                                string += &format!(
                                    "\n[{}]:\t\tState: {:?}\n\t\tOperation: {:?}",
                                    id, state, op
                                );
                            }
                            return Ok(Some(string));
                        }
                        OutputFormat::Json => {
                            let mut json_vec = vec![];
                            for (op, state) in iter {
                                json_vec.push(json!({
                                    "operation": op.to_json(),
                                    "state": format!("{:?}", state),
                                }))
                            }
                            return Ok(Some(json!({ "history": json_vec }).to_string()));
                        }
                    }
                }
                None => {
                    return Err(
                        "Can not execute interactive command at non-interactive mode.".to_string(),
                    );
                }
            },
            Command::Help => {
                return Ok(Some(format!(
                    "Common command:\n\
                    * i,interactive -- Interactive mode. Reinitialize if already at interactive mod.\n\
                    * ni,noninteractive -- Exit interactive mode.\n\
                    * 3pl,3-player -- 3 players mahjong. Reinitialize if interactive mode.\n\
                    * 4pl,4-player -- 4 players mahjong. Reinitialize if interactive mode.\n\
                    * std, standard -- Standard output mode.\n\
                    * json -- JSON output mode.\n\
                    * q,quit,exit -- Exit program.\n\
                    * h,help -- Print command list.\n\
                    \n\
                    Command for interactive mode:\n\
                    * + -- Add a hai to tehai. For an example, \"+4m\".\n\
                    * - -- Discard a hai from tehai. For an example, \"-1s\".\n\
                    * *+ -- Add some hai to haiyama. Limit is 4 for each type of hai.\n\
                    * *- -- Discard some hai from haiyama. For an example, \"*-1s777z\". Note that \
                    you shouldn't use \"*-\" for nakihai.\n\
                    * > -- Naku. It means chii, pon or kan. Third hai will be regarded as nakihai if chii. \
                    You can use \">4444p5s\" to represent kan 4p and get rinshanhai 5s and also you can use \
                    \"4444p\" then \"+5s\". However, you can also write \"44p5s44p\", the order does not \
                    matter. Note: \">4444p\" is daiminkan, \"+4p\" then \">4444p\" is kakan or ankan.\n\
                    * b,back -- Undo last operation.\n\
                    * s,state -- Print current game state, including haiyama, types of sutehai, tehai.\n\
                    * d,display -- Normally program will print tehai analysis result after operation if \
                    tehai full with hai. You can use this command print again.\n\
                    * log,history -- Print operation history.\n\
                    \n\
                    Haiyama errors will cause operation failure and game state recovery. \
                    If you don't care errors from haiyama, you can use following command. \
                    If the number of a type of hai is 0, discard from haiyama will keep 0; \
                    if 4, add to haiyama will keep 4:\n\
                    * +! -- Add a hai to tehai ignoring haiyama error.\n\
                    * -! -- Equal with -, no difference.\n\
                    * *+! -- Add some hai to haiyama ignoring haiyama error.\n\
                    * *-! -- Discard some hai from haiyama ignoring haiyama error.\n\
                    * >! -- Naku ignoring haiyama error.\n\
                    * b!,back! -- Undo operation ignoring haiyama error. Note if you use \"back\" for operations \
                    who ignored haiyama error, \"back\" will keep reporting haiyama errors."
                )))
            }
        };
        Ok(None)
    }
}
