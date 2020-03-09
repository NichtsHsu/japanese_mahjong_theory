use lazy_static::lazy_static;
pub use mut_static::MutStatic;

use crate::game::Game;

/// Players number.
///
/// # Enum
/// * Three: 3-players mode, no 2m~8m.
/// * Four: standard 4-players mode.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Players {
    Three,
    Four,
}

/// Output format.
///
/// # Enum
/// * Standard: normal console output.
/// * Json: format by json for back-end mode.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OutputFormat {
    Standard,
    Json,
}

/// Interactive State
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InteractiveState {
    Noninteractive,
    WaitForFirstInput,
    FullTiles,
    LackOneTile,
    WaitForRinshanInput,
}

// Global variables
lazy_static! {
    pub static ref PLAYERS_NUMBER: MutStatic<Players> = MutStatic::new();
    pub static ref OUTPUT_FORMAT: MutStatic<OutputFormat> = MutStatic::new();
    pub static ref INTERACTIVE: MutStatic<InteractiveState> = MutStatic::new();
    pub static ref GAME: MutStatic<Game> = MutStatic::from(Game::new());
}

impl ToString for Players {
    fn to_string(&self) -> String {
        match self {
            Players::Three => 3.to_string(),
            Players::Four => 4.to_string(),
        }
    }
}