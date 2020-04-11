use super::OutputFormat;
use crate::game;

pub enum Command {
    Interactive,
    Noninteractive,
    OutputFormat(OutputFormat),
    PlayerNumber(game::PlayerNumber),
    TehaiInput(game::Tehai),
    GameOperation(game::Operation),
    Back { haiyama_sensitive: bool },
    State,
    Display,
    History,
    Help,
    Exit,
}

impl Command {
    pub fn parse(command: String, player_number: game::PlayerNumber) -> Result<Command, String> {
        match &*command {
            "ni" | "noninteractive" => Ok(Command::Noninteractive),
            "i" | "interactive" => Ok(Command::Interactive),
            "q" | "quit" | "exit" => Ok(Command::Exit),
            "s" | "state" => Ok(Command::State),
            "b" | "back" => Ok(Command::Back {
                haiyama_sensitive: true,
            }),
            "b!" | "back!" => Ok(Command::Back {
                haiyama_sensitive: false,
            }),
            "d" | "display" => Ok(Command::Display),
            "log" | "history" => Ok(Command::History),
            "h" | "help" => Ok(Command::Help),
            "3pl" | "3-player" => Ok(Command::PlayerNumber(game::PlayerNumber::Three)),
            "4pl" | "4-player" => Ok(Command::PlayerNumber(game::PlayerNumber::Four)),
            "std" | "standard" => Ok(Command::OutputFormat(OutputFormat::Standard)),
            "json" => Ok(Command::OutputFormat(OutputFormat::Json)),
            _ => Command::parse_with_argument(command, player_number),
        }
    }

    fn parse_with_argument(
        mut command: String,
        player_number: game::PlayerNumber,
    ) -> Result<Command, String> {
        let tmp = command.clone();
        let bytes = tmp.as_bytes();
        if bytes.len() < 3 {
            return Err(format!("Unresolved command: {}.", command));
        }

        let haiyama_sensitive = if let '!' = bytes[1] as char {
            false
        } else {
            true
        };

        match bytes[0] as char {
            '+' => {
                command.remove(0);
                if !haiyama_sensitive {
                    command.remove(0);
                }
                let hai_vec = game::Hai::from_string_unordered(&command, player_number)?;
                if hai_vec.len() == 1 {
                    Ok(Command::GameOperation(game::Operation::Tehai(
                        game::TehaiOperation::Add {
                            hai: hai_vec[0],
                            haiyama_sensitive,
                        },
                    )))
                } else {
                    Err("Can only add one hai when use '+' operator.".to_string())
                }
            }
            '-' => {
                command.remove(0);
                if !haiyama_sensitive {
                    command.remove(0);
                }
                let hai_vec = game::Hai::from_string_unordered(&command, player_number)?;
                if hai_vec.len() == 1 {
                    Ok(Command::GameOperation(game::Operation::Tehai(
                        game::TehaiOperation::Discard(hai_vec[0]),
                    )))
                } else {
                    Err("Can only discard one hai when use '-' operator.".to_string())
                }
            }
            '*' => {
                command.remove(0);
                let pos = if haiyama_sensitive {
                    1
                } else {
                    command.remove(0);
                    2
                };
                command.remove(0);
                match bytes[pos] as char {
                    '+' => {
                        let hai_vec = game::Hai::from_string_unordered(&command, player_number)?;
                        Ok(Command::GameOperation(game::Operation::Haiyama {
                            kind: game::HaiyamaOperation::Add(hai_vec),
                            haiyama_sensitive,
                        }))
                    }
                    '-' => {
                        let hai_vec = game::Hai::from_string_unordered(&command, player_number)?;
                        Ok(Command::GameOperation(game::Operation::Haiyama {
                            kind: game::HaiyamaOperation::Discard(hai_vec),
                            haiyama_sensitive,
                        }))
                    }
                    _ => return Err(format!("Unresolved command: {}.", command)),
                }
            }
            '>' => {
                command.remove(0);
                if !haiyama_sensitive {
                    command.remove(0);
                }
                let mut hai_vec = game::Hai::from_string_unordered(&command, player_number)?;
                match hai_vec.len() {
                    3 => {
                        let mentsu = game::Mentsu::new(&hai_vec, player_number);
                        if let Some(mentsu) = mentsu {
                            match mentsu {
                                game::Mentsu::Juntsu(..) => Ok(Command::GameOperation(
                                    game::Operation::Tehai(game::TehaiOperation::Naku {
                                        kind: game::Naku::Chii {
                                            juntsu: mentsu,
                                            nakihai: hai_vec[2],
                                        },
                                        haiyama_sensitive,
                                    }),
                                )),
                                game::Mentsu::Koutsu(..) => Ok(Command::GameOperation(
                                    game::Operation::Tehai(game::TehaiOperation::Naku {
                                        kind: game::Naku::Pon(mentsu),
                                        haiyama_sensitive,
                                    }),
                                )),
                                _ => Err("Logic error: Code never reach here.".to_string()),
                            }
                        } else {
                            Err(format!("'{}' is not a valid mentsu.", command))
                        }
                    }
                    4 => {
                        if hai_vec[0] == hai_vec[1]
                            && hai_vec[0] == hai_vec[2]
                            && hai_vec[0] == hai_vec[3]
                        {
                            Ok(Command::GameOperation(game::Operation::Tehai(
                                game::TehaiOperation::Naku {
                                    kind: game::Naku::Kan(game::Kan::Unknown {
                                        kantsu: game::Mentsu::Kantsu(hai_vec[0]),
                                        rinshanhai: None,
                                    }),
                                    haiyama_sensitive,
                                },
                            )))
                        } else {
                            Err(format!("'{}' is not a valid mentsu.", command))
                        }
                    }
                    5 => {
                        hai_vec.sort();
                        let (kantsuhai, rinshanhai) = if hai_vec[0] == hai_vec[1]
                            && hai_vec[0] == hai_vec[2]
                            && hai_vec[0] == hai_vec[3]
                            && hai_vec[0] != hai_vec[4]
                        {
                            (hai_vec[0], hai_vec[4])
                        } else if hai_vec[4] == hai_vec[1]
                            && hai_vec[4] == hai_vec[2]
                            && hai_vec[4] == hai_vec[3]
                            && hai_vec[4] != hai_vec[0]
                        {
                            (hai_vec[4], hai_vec[0])
                        } else {
                            return Err(format!("'{}' is not a valid mentsu.", command));
                        };
                        Ok(Command::GameOperation(game::Operation::Tehai(
                            game::TehaiOperation::Naku {
                                kind: game::Naku::Kan(game::Kan::Unknown {
                                    kantsu: game::Mentsu::Kantsu(kantsuhai),
                                    rinshanhai: Some(rinshanhai),
                                }),
                                haiyama_sensitive,
                            },
                        )))
                    }
                    _ => return Err(format!("Unresolved command: {}.", command)),
                }
            }
            _ => Ok(Command::TehaiInput(game::Tehai::new(
                command,
                player_number,
            )?)),
        }
    }
}
