use lazy_static::lazy_static;
pub use mut_static::MutStatic;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Players {
    Three,
    Four,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OutputFormat {
    Standard,
    Json,
}

lazy_static! {
    pub static ref PLAYERS_NUMBER: MutStatic<Players> = MutStatic::new();
    pub static ref OUTPUT_FORMAT: MutStatic<OutputFormat> = MutStatic::new();
    pub static ref INTERACTIVE: MutStatic<bool> = MutStatic::new();
}

impl ToString for Players {
    fn to_string(&self) -> String {
        match self {
            Players::Three => 3.to_string(),
            Players::Four => 4.to_string(),
        }
    }
}
