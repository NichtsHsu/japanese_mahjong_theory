/// Number of players. Support 4-players mode and
/// 3-players mode yet.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PlayerNumber {
    Three,
    Four,
}

impl ToString for PlayerNumber {
    fn to_string(&self) -> String {
        match self {
            Three => "3".to_string(),
            Four => "4".to_string(),
        }
    }
}