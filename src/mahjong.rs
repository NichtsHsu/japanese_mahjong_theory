//! This mod defines all structures for an entire mahjong game.

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// Type of a tile.
///
/// # Japanese
/// * Hai: 牌
/// * Manzu: 萬子
/// * Pinzu: 筒子
/// * Souzu: 索子
/// * Jihai: 字牌
///
/// # Abbreviation
/// * Manzu: m
/// * Pinzu: p
/// * Souzu: s
/// * Jihai: z
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Hai {
    Manzu(u8),
    Pinzu(u8),
    Souzu(u8),
    Jihai(u8),
}

/// Type of a meld.
///
/// # Japanese
/// Mentsu: 面子
/// Juntsu: 順子
/// Koutsu: 刻子
/// Kantsu: 槓子
///
/// # Examples
/// * Juntsu: 2s 3s 4s
/// * Koutsu: 1z 1z 1z
/// * Kantsu: 6m 6m 6m 6m
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Mentsu {
    Juntsu(Hai, Hai, Hai),
    Koutsu(Hai),
    Kantsu(Hai),
}

/// Two different tiles wait for one.
///
/// # Japanese
/// * Taatsu: 搭子
///
/// # Examples
/// * 1s 2s wait for 3s
/// * 4p 6p wait for 5p
/// * 7m 8m wait for 6m and 9m
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Taatsu(pub Hai, pub Hai);

/// Two same tiles.
///
/// # Japanese
/// * Toitsu: 対子
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Toitsu(pub Hai);

/// An isolated tile.
///
/// # Japanese
/// * Ukihai: 浮き牌
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Ukihai(pub Hai);

/// Array of 1m 9m 1p 9p 1s 9s 1z 2z 3z 4z 5z 6z 7z.
/// 
/// # Japanese
/// * yaochuupai: 么九牌 
pub const YAOCHUUPAI: [Hai; 13] = [
    Hai::Manzu(1),
    Hai::Manzu(9),
    Hai::Pinzu(1),
    Hai::Pinzu(9),
    Hai::Souzu(1),
    Hai::Souzu(9),
    Hai::Jihai(1),
    Hai::Jihai(2),
    Hai::Jihai(3),
    Hai::Jihai(4),
    Hai::Jihai(5),
    Hai::Jihai(6),
    Hai::Jihai(7),
];

/// Tiles on hand.
///
/// # Japanese
/// * Tehai: 手牌
/// * menzen: 門前
/// * fuuro: 副露
///
/// # Member
/// * menzen: tiles not formed melds by seizing another's discard.
/// * fuuro: melds formed by seizing another's discard.
///
/// # Examples
/// ```rust
/// let mut input = String::new();
/// io::stdin().read_line(&mut input).expect("error: unable to read user input");
/// println!("{}", mahjong::Tehai::from(input.trim().to_string()));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tehai {
    pub menzen: Result<Vec<Hai>, String>,
    pub fuuro: Vec<Mentsu>,
}

/// Tiles on wall.
///
/// # Japanese
/// * Haiyama: 牌山
/// * nokori: 残り
/// * sutehai: 捨て牌
pub struct Haiyama {
    pub nokori: HashMap<Hai, u8>,
    pub sutehai_type: HashSet<Hai>,
}

impl Hai {
    /// Get next tile.
    ///
    /// # Parameters
    /// * dora_loop: if dora_loop is false, next of 9m, 9p, 9s, 7z will be None.
    /// Otherwise, next of 9m, 9p, 9s are 1m, 1p, 1s, next of 4z is 1z, next of 7z is 5z.
    ///
    /// # Examples
    /// ```rust
    /// assert_eq!(Hai::Manzu(4).next(false), Some(Hai::Manzu(5)));
    /// assert_eq!(Hai::Pinzu(9).next(false), None);
    /// assert_eq!(Hai::Jihai(4).next(true), Some(Hai::Jihai(1)));
    /// ```
    pub fn next(&self, dora_loop: bool) -> Option<Hai> {
        match self {
            Hai::Manzu(num) => {
                if *num != 9 {
                    Some(Hai::Manzu(*num + 1))
                } else if dora_loop {
                    Some(Hai::Manzu(1))
                } else {
                    None
                }
            }
            Hai::Pinzu(num) => {
                if *num != 9 {
                    Some(Hai::Pinzu(*num + 1))
                } else if dora_loop {
                    Some(Hai::Pinzu(1))
                } else {
                    None
                }
            }
            Hai::Souzu(num) => {
                if *num != 9 {
                    Some(Hai::Souzu(*num + 1))
                } else if dora_loop {
                    Some(Hai::Souzu(1))
                } else {
                    None
                }
            }
            Hai::Jihai(num) => {
                if dora_loop {
                    if *num == 4 {
                        Some(Hai::Jihai(1))
                    } else if *num == 7 {
                        Some(Hai::Jihai(5))
                    } else {
                        Some(Hai::Jihai(*num + 1))
                    }
                } else if *num != 7 {
                    Some(Hai::Jihai(*num + 1))
                } else {
                    None
                }
            }
        }
    }
}

impl ToString for Hai {
    fn to_string(&self) -> String {
        match self {
            Hai::Manzu(num) => format!("{}m", num),
            Hai::Pinzu(num) => format!("{}p", num),
            Hai::Souzu(num) => format!("{}s", num),
            Hai::Jihai(num) => format!("{}z", num),
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

impl Tehai {
    pub fn new(menzen: Result<Vec<Hai>, String>, fuuro: Vec<Mentsu>) -> Self {
        let mut tehai = Tehai { menzen, fuuro };
        tehai.check_hai_in_four();
        tehai
    }

    pub fn check_hai_in_four(&mut self) -> &mut Self {
        if let Ok(menzen_vec) = self.menzen.clone() {
            let mut tehai_map: HashMap<Hai, u8> = HashMap::new();

            let mut check_count = |menzen: &mut Result<_, _>, hai| -> bool {
                if tehai_map.contains_key(hai) {
                    let count = tehai_map[hai] + 1;
                    if count > 4 {
                        *menzen = Err(format!(
                            "Invalid because of fifth tile {}.",
                            hai.to_string()
                        ));
                        return false;
                    }
                    tehai_map.insert(*hai, count);
                } else {
                    tehai_map.insert(*hai, 1);
                }
                return true;
            };

            for item in menzen_vec.iter() {
                if !check_count(&mut self.menzen, item) {
                    return self;
                }
            }
            for mentsu in self.fuuro.iter() {
                match mentsu {
                    Mentsu::Juntsu(a, b, c) => {
                        if !check_count(&mut self.menzen, a)
                            || !check_count(&mut self.menzen, b)
                            || !check_count(&mut self.menzen, c)
                        {
                            return self;
                        }
                    }
                    Mentsu::Koutsu(item) => {
                        for _ in 0..3 {
                            if !check_count(&mut self.menzen, item) {
                                return self;
                            }
                        }
                    }
                    Mentsu::Kantsu(item) => {
                        for _ in 0..4 {
                            if !check_count(&mut self.menzen, item) {
                                return self;
                            }
                        }
                    }
                }
            }
        }
        self
    }
}

impl From<String> for Tehai {
    fn from(string: String) -> Self {
        crate::analyzer::input::parse(string)
    }
}

/// Simple output for tiles.
///
/// # Example
/// `1m2m3m4p4p4p7p5s6s6s7s[1z1z1z]`
impl std::fmt::Display for Tehai {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut format_string = String::new();

        match self.menzen.as_ref() {
            Ok(menzen_vec) => {
                for hai in menzen_vec.iter() {
                    format_string += &hai.to_string();
                }
            }
            Err(error) => format_string = error.clone(),
        };
        for mentsu in &self.fuuro {
            format_string += &mentsu.to_string();
        }

        write!(f, "{}", format_string)
    }
}
