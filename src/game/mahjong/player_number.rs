/// Number of players. Support 4-players mode and
/// 3-players mode yet.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PlayerNumber {
    Three,
    Four,
}

impl std::fmt::Display for PlayerNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PlayerNumber::Three => "3".to_string(),
                PlayerNumber::Four => "4".to_string(),
            }
        )
    }
}
