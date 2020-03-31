use super::{Hai, Mentsu, PlayerNumber};
use std::collections::HashMap;
use serde_json::json;

/// Tiles on hand.
///
/// # Japanese
/// * Tehai: 手牌
/// * juntehai: 純手牌
/// * fuuro: 副露
///
/// # Member
/// * juntehai: Vec of hai which not formed mentsu.
/// * fuuro: Mentsu which already formed.
///
/// # Examples
/// ```rust
/// let mut input = String::new();
/// io::stdin().read_line(&mut input).expect("error: unable to read user input");
/// println!("{}", mahjong::Tehai::from(input.trim().to_string()));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tehai {
    pub juntehai: Vec<Hai>,
    pub fuuro: Vec<Mentsu>,
}

impl Tehai {
    /// Create tehai from string.
    ///
    /// # Input
    /// You can input tiles out of order, and use [] represent formed melds. All spaces will be ignored.
    /// (they will not be considered for shanten number).
    /// * stanard: `1m2m3m4m4m5m4p4p4p5p8s[1z1z1z]`
    /// * shorter: `123445m4445p8s[111z]`
    /// * with spaces: `123445m 4445p 8s [111z]`
    /// * chaos: `45p 8s14 4m[11 1z]2 5m44p 3m`
    ///
    /// # Examples
    /// ```rust
    /// let tehai = Tehai::new("45p8s144m[111z]25m44p3m".to_string(), PlayerNumber::Four);
    /// ```
    pub fn new(string: String, player_number: PlayerNumber) -> Result<Self, String> {
        let mut juntehai = vec![];
        let mut fuuro = vec![];
        let mut char_stash: Vec<char> = vec![];
        let mut hai_in_mentsu_stash: Vec<Hai> = vec![];
        let mut in_mentsu = false;

        fn handle_char_stash(
            hai_type: char,
            hai_type_char_index: usize,
            player_number: PlayerNumber,
            char_stash: &mut Vec<char>,
            output: &mut Vec<Hai>,
        ) -> Result<(), String> {
            if char_stash.len() == 0 {
                Err(format!(
                    "Unused type character '{}' at index {}.",
                    hai_type, hai_type_char_index
                ))
            } else {
                for tile in char_stash.iter() {
                    let hai = match hai_type {
                        'm' => Hai::Manzu(*tile as u8 - 48),
                        'p' => Hai::Pinzu(*tile as u8 - 48),
                        's' => Hai::Souzu(*tile as u8 - 48),
                        'z' => Hai::Jihai(*tile as u8 - 48),
                        _ => Hai::Manzu(0), // Never reach here.
                    };
                    if hai.is_valid(player_number) {
                        output.push(hai);
                    } else {
                        char_stash.clear();
                        return Err(format!("'{}' is invalid hai.", hai.to_string()));
                    }
                }
                char_stash.clear();
                Ok(())
            }
        }

        fn handle_hai_in_mentsu_stash(
            char_index: usize,
            player_number: PlayerNumber,
            hai_in_mentsu_stash: &mut Vec<Hai>,
            output: &mut Vec<Mentsu>,
        ) -> Result<(), String> {
            if let Some(mentsu) = Mentsu::new(hai_in_mentsu_stash, player_number) {
                output.push(mentsu);
                hai_in_mentsu_stash.clear();
                Ok(())
            } else {
                Err(format!(
                    "Not a valid meld on '[]' before index {}.",
                    char_index
                ))
            }
        }

        for (index, chr) in string.chars().enumerate() {
            match chr {
                'm' | 'p' | 's' | 'z' => {
                    if in_mentsu {
                        handle_char_stash(
                            chr,
                            index,
                            player_number,
                            &mut char_stash,
                            &mut hai_in_mentsu_stash,
                        )?;
                    } else {
                        handle_char_stash(
                            chr,
                            index,
                            player_number,
                            &mut char_stash,
                            &mut juntehai,
                        )?;
                    }
                }
                '1'..='9' => char_stash.push(chr),
                '[' => {
                    if in_mentsu {
                        return Err(format!("Second '[' found at index {}.", index));
                    }
                    if char_stash.len() > 0 {
                        return Err(format!(
                            "Need 'm' 'p' 's' 'z' but find '[' at index {}.",
                            index
                        ));
                    };
                    in_mentsu = true;
                }
                ']' => {
                    if !in_mentsu {
                        return Err(format!("Unmatched ']' found at index {}.", index));
                    }
                    if char_stash.len() > 0 {
                        return Err(format!(
                            "Need 'm' 'p' 's' 'z' but find ']' at index {}.",
                            index
                        ));
                    };
                    handle_hai_in_mentsu_stash(
                        index,
                        player_number,
                        &mut hai_in_mentsu_stash,
                        &mut fuuro,
                    )?;
                    in_mentsu = false;
                }
                // Ignore all spaces.
                ' ' => (),
                _ => {
                    return Err(format!("Unknown character '{}' at index {}.", chr, index));
                }
            }
        }

        if char_stash.len() > 0 {
            return Err(format!(
                "No type specified for '{:?}' at the end of input string.",
                char_stash
            ));
        }

        juntehai.sort();
        let tehai = Self { juntehai, fuuro };

        match tehai.check_hai_number() {
            Ok(_) => Ok(tehai),
            Err(hai) => Err(format!("Fifth {} found.", hai.to_string())),
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        let mut juntehai_string_vec = vec![];
        for hai in &self.juntehai {
            juntehai_string_vec.push(hai.to_string());
        }
        let mut fuuro_json_vec = vec![];
        for mentsu in &self.fuuro {
            fuuro_json_vec.push(mentsu.to_json());
        }
        json!({
           "juntehai": juntehai_string_vec,
           "fuuro": fuuro_json_vec
        })
    }

    fn check_hai_number(&self) -> Result<(), Hai> {
        let mut tehai_map: HashMap<Hai, u8> = HashMap::new();

        let mut check_count = |hai| -> bool {
            if tehai_map.contains_key(hai) {
                let count = tehai_map[hai] + 1;
                if count > 4 {
                    return false;
                }
                tehai_map.insert(*hai, count);
            } else {
                tehai_map.insert(*hai, 1);
            }
            return true;
        };

        for hai in self.juntehai.iter() {
            if !check_count(hai) {
                return Err(*hai);
            }
        }
        for mentsu in self.fuuro.iter() {
            match mentsu {
                Mentsu::Juntsu(a, b, c) => {
                    for hai in vec![a, b, c] {
                        if !check_count(hai) {
                            return Err(*hai);
                        }
                    }
                }
                Mentsu::Koutsu(hai) => {
                    for _ in 0..3 {
                        if !check_count(hai) {
                            return Err(*hai);
                        }
                    }
                }
                Mentsu::Kantsu(hai) => {
                    for _ in 0..4 {
                        if !check_count(hai) {
                            return Err(*hai);
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

impl std::fmt::Display for Tehai {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut format_string = String::new();

        for hai in &self.juntehai {
            format_string += &hai.to_string();
        }
        for mentsu in &self.fuuro {
            format_string += &mentsu.to_string();
        }

        write!(f, "{}", format_string)
    }
}
