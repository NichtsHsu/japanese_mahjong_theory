use super::{Hai, PlayerNumber};
use serde_json::json;

/// Type of mentsu(meld).
///
/// # Japanese
/// Mentsu: 面子
/// Juntsu: 順子
/// Koutsu: 刻子
/// Kantsu: 槓子
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Mentsu {
    Juntsu(Hai, Hai, Hai),
    Koutsu(Hai),
    Kantsu(Hai),
}

/// Two different hai wait for one hai.
///
/// # Japanese
/// * Taatsu: 搭子
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Taatsu(pub Hai, pub Hai);

/// Two same hai.
///
/// # Japanese
/// * Toitsu: 対子
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Toitsu(pub Hai);

/// An isolated hai.
///
/// # Japanese
/// * Ukihai: 浮き牌
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Ukihai(pub Hai);

impl Mentsu {
    /// Create a mentsu from input vec of hai if they can make up a valid mentsu.
    pub fn new(hai_vec: &Vec<Hai>, player_number: PlayerNumber) -> Option<Self> {
        fn check_juntsu(mut a: u8, mut b: u8, mut c: u8) -> Option<(u8, u8, u8)> {
            if a > b {
                std::mem::swap(&mut a, &mut b)
            }
            if a > c {
                std::mem::swap(&mut a, &mut c)
            }
            if b > c {
                std::mem::swap(&mut b, &mut c)
            }
            if a + 1 == b && b + 1 == c {
                Some((a, b, c))
            } else {
                None
            }
        }

        if !Hai::check_iter_valid(hai_vec.iter(), player_number) {
            None
        } else if hai_vec.len() == 4 {
            if hai_vec[0] == hai_vec[1] && hai_vec[0] == hai_vec[2] && hai_vec[0] == hai_vec[3] {
                Some(Mentsu::Kantsu(hai_vec[0]))
            } else {
                None
            }
        } else if hai_vec.len() == 3 {
            if hai_vec[0] == hai_vec[1] && hai_vec[0] == hai_vec[2] {
                Some(Mentsu::Koutsu(hai_vec[0]))
            } else {
                match (hai_vec[0], hai_vec[1], hai_vec[2]) {
                    (Hai::Manzu(a), Hai::Manzu(b), Hai::Manzu(c)) => match player_number {
                        PlayerNumber::Four => {
                            let (a, b, c) = check_juntsu(a, b, c)?;
                            Some(Mentsu::Juntsu(Hai::Manzu(a), Hai::Manzu(b), Hai::Manzu(c)))
                        }
                        PlayerNumber::Three => None,
                    },
                    (Hai::Pinzu(a), Hai::Pinzu(b), Hai::Pinzu(c)) => {
                        let (a, b, c) = check_juntsu(a, b, c)?;
                        Some(Mentsu::Juntsu(Hai::Pinzu(a), Hai::Pinzu(b), Hai::Pinzu(c)))
                    }
                    (Hai::Souzu(a), Hai::Souzu(b), Hai::Souzu(c)) => {
                        let (a, b, c) = check_juntsu(a, b, c)?;
                        Some(Mentsu::Juntsu(Hai::Souzu(a), Hai::Souzu(b), Hai::Souzu(c)))
                    }
                    _ => None,
                }
            }
        } else {
            None
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        let mut hai_string_vec = vec![];
        match self {
            Mentsu::Juntsu(a, b, c) => {
                hai_string_vec.push(a.to_string());
                hai_string_vec.push(b.to_string());
                hai_string_vec.push(c.to_string());
                json!({
                    "type":"juntsu",
                    "hai":hai_string_vec
                })
            }
            Mentsu::Koutsu(a) => {
                for _ in 0..3 {
                    hai_string_vec.push(a.to_string());
                }
                json!({
                    "type":"koutsu",
                    "hai":hai_string_vec
                })
            }
            Mentsu::Kantsu(a) => {
                for _ in 0..4 {
                    hai_string_vec.push(a.to_string());
                }
                json!({
                    "type":"kantsu",
                    "hai":hai_string_vec
                })
            }
        }
    }
}

impl ToString for Mentsu {
    fn to_string(&self) -> String {
        match self {
            Mentsu::Juntsu(a, b, c) => {
                format!("[{}{}{}]", a.to_string(), b.to_string(), c.to_string())
            }
            Mentsu::Koutsu(a) => {
                let tile = a.to_string();
                format!("[{}{}{}]", tile, tile, tile)
            }
            Mentsu::Kantsu(a) => {
                let tile = a.to_string();
                format!("[{}{}{}{}]", tile, tile, tile, tile)
            }
        }
    }
}

impl ToString for Taatsu {
    fn to_string(&self) -> String {
        format!("{}{}", self.0.to_string(), self.1.to_string())
    }
}

impl ToString for Toitsu {
    fn to_string(&self) -> String {
        let tile = self.0.to_string();
        format!("{}{}", tile, tile)
    }
}

impl ToString for Ukihai {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
