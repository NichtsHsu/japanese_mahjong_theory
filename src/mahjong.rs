//! This mod defines all structures for an entire mahjong game.

/// Type of a tile.
/// # Abbreviation
/// * Manzu -> m
/// * Pinzu -> p
/// * Souzu -> s
/// * Jihai -> z
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Hai {
    Manzu(u8),
    Pinzu(u8),
    Souzu(u8),
    Jihai(u8),
}

/// Type of a meld.
/// # Examples
/// * Juntsu: 2s 3s 4s
/// * Koutsu: 1z 1z 1z
/// * Kantsu: 6m 6m 6m 6m
#[derive(Copy, Clone, Debug)]
pub enum Mentsu {
    Juntsu(Hai, Hai, Hai),
    Koutsu(Hai),
    Kantsu(Hai),
}

/// Two different tiles wait for one.
/// # Examples
/// * 1s 2s wait for 3s
/// * 4p 6p wait for 5p
/// * 7m 8m wait for 6m and 9m
pub struct Taatsu(Hai);

// Tow same tiles.
pub struct Toitsu(Hai);

/// Tiles on hand.
///
/// # Member
/// * menzen : tiles not formed melds by seizing another's discard.
/// * fuuro : melds formed by seizing another's discard.
/// 
/// # Examples
/// ```rust
/// let mut input = String::new();
/// io::stdin().read_line(&mut input).expect("error: unable to read user input");
/// println!("{}", mahjong::Tehai::from(input.trim().to_string()));
/// ```
#[derive(Debug, Clone)]
pub struct Tehai {
    pub menzen: Result<Vec<Hai>, String>,
    pub fuuro: Vec<Mentsu>,
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

impl Tehai {
    pub fn new(menzen: Result<Vec<Hai>, String>, fuuro: Vec<Mentsu>) -> Self {
        Tehai { menzen, fuuro }
    }
}

impl From<String> for Tehai {
    fn from(string: String) -> Self {
        crate::analyzer::input::parse(string)
    }
}

/// Simple output for tiles.
/// # Example
/// `1m2m3m4p4p4p7p5s6s6s7s[1z1z1z]`
impl std::fmt::Display for Tehai {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut format_string = String::new();

        match &self.menzen {
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
