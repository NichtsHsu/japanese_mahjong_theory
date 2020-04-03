use super::PlayerNumber;
use std::collections::BTreeSet;

/// Type of hai(tile).
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

impl Hai {
    /// Parse string to a vec of hai. Order of hai is equal with input string.
    pub fn from_string_unordered(string: &String, player_number: PlayerNumber) -> Result<Vec<Hai>, String> {
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
                for hai in char_stash.iter() {
                    let hai = match hai_type {
                        'm' => Hai::Manzu(*hai as u8 - 48),
                        'p' => Hai::Pinzu(*hai as u8 - 48),
                        's' => Hai::Souzu(*hai as u8 - 48),
                        'z' => Hai::Jihai(*hai as u8 - 48),
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

        let mut char_stash: Vec<char> = vec![];
        let mut hai_vec = vec![];
        
        for (index, chr) in string.chars().enumerate() {
            match chr {
                'm' | 'p' | 's' | 'z' => {
                    handle_char_stash(chr, index, player_number, &mut char_stash, &mut hai_vec)?;
                }
                '1'..='9' => char_stash.push(chr),
                // Ignore all spaces.
                ' ' => (),
                _ => {
                    return Err(format!("Unknown character '{}' at index {}.", chr, index));
                }
            }
        }

        Ok(hai_vec)
    }

    /// Return if valid -- it means 1\~9m, 1\~9p, 1\~9s, 1\~7z on 4-players mode
    /// and 1m, 9m, 1\~9p, 1\~9s, 1\~7z on 3-players mode.
    pub fn is_valid(&self, player_number: PlayerNumber) -> bool {
        match (self, player_number) {
            (Hai::Manzu(1..=9), PlayerNumber::Four)
            | (Hai::Manzu(1), PlayerNumber::Three)
            | (Hai::Manzu(9), PlayerNumber::Three)
            | (Hai::Pinzu(1..=9), _)
            | (Hai::Souzu(1..=9), _)
            | (Hai::Jihai(1..=7), _) => true,
            _ => false,
        }
    }

    /// Return ture when **all** hai in iterator is valid. Otherwise return false.
    pub fn check_iter_valid<'a, T>(iter: T, player_number: PlayerNumber) -> bool
    where
        T: Iterator<Item = &'a Self>,
    {
        for hai in iter {
            if !hai.is_valid(player_number) {
                return false;
            }
        }

        true
    }

    /// Return a BTreeSet including all yaochuupai -- 1m, 9m, 1p, 9p, 1s, 9s, 1\~7z.
    pub fn yaochuupai_type() -> BTreeSet<Hai> {
        let mut yaochuupai_vec = BTreeSet::new();

        yaochuupai_vec.insert(Hai::Manzu(1));
        yaochuupai_vec.insert(Hai::Manzu(9));
        yaochuupai_vec.insert(Hai::Pinzu(1));
        yaochuupai_vec.insert(Hai::Pinzu(9));
        yaochuupai_vec.insert(Hai::Souzu(1));
        yaochuupai_vec.insert(Hai::Souzu(9));
        for i in 1..=7 {
            yaochuupai_vec.insert(Hai::Jihai(i));
        }

        yaochuupai_vec
    }

    /// Return a BTreeSet including all valid types of hai.
    pub fn all_type(player_number: PlayerNumber) -> BTreeSet<Hai> {
        let mut all_hai_type = BTreeSet::new();

        match player_number {
            PlayerNumber::Four => {
                for index in 1u8..=9u8 {
                    all_hai_type.insert(Hai::Manzu(index));
                }
            }
            PlayerNumber::Three => {
                all_hai_type.insert(Hai::Manzu(1));
                all_hai_type.insert(Hai::Manzu(9));
            }
        };

        for index in 1u8..=9u8 {
            all_hai_type.insert(Hai::Pinzu(index));
            all_hai_type.insert(Hai::Souzu(index));
        }

        for index in 1u8..=7u8 {
            all_hai_type.insert(Hai::Jihai(index));
        }

        all_hai_type
    }

    /// Return previous hai. It means, like 1m for 2m.
    ///
    /// # Parameters
    /// * player_number: Number of players. No 2\~8m on 3-players mode.
    /// * dora_loop: If true, `Manzu(1).previous()`, `Pinzu(1).previous()` and
    /// `Souzu(1).previous()` will be `Some(Manzu(9))`, `Some(Pinzu(9))` and
    /// `Some(souzu(9))`, `Jihai(1).previous()` will be `Some(Jihai(4))`,
    /// `Jihai(5).previous()` will be `Some(Jihai(7))`. Otherwise, `Manzu(1).previous()`,
    /// `Pinzu(1).previous()`, `Souzu(1).previous()` and `Jihai(1).previous()`
    /// will all be `None`.
    ///
    /// # Japanese
    /// * dora: ドラ
    pub fn previous(&self, player_number: PlayerNumber, dora_loop: bool) -> Option<Hai> {
        match self {
            Hai::Manzu(num) => match player_number {
                PlayerNumber::Four => {
                    if *num != 1 {
                        Some(Hai::Manzu(*num - 1))
                    } else if dora_loop {
                        Some(Hai::Manzu(9))
                    } else {
                        None
                    }
                }
                PlayerNumber::Three => {
                    if dora_loop {
                        if *num == 1 {
                            Some(Hai::Manzu(9))
                        } else if *num == 9 {
                            Some(Hai::Manzu(1))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
            },
            Hai::Pinzu(num) => {
                if *num != 1 {
                    Some(Hai::Pinzu(*num - 1))
                } else if dora_loop {
                    Some(Hai::Pinzu(9))
                } else {
                    None
                }
            }
            Hai::Souzu(num) => {
                if *num != 1 {
                    Some(Hai::Souzu(*num - 1))
                } else if dora_loop {
                    Some(Hai::Souzu(9))
                } else {
                    None
                }
            }
            Hai::Jihai(num) => {
                if dora_loop {
                    if *num == 1 {
                        Some(Hai::Jihai(4))
                    } else if *num == 5 {
                        Some(Hai::Jihai(7))
                    } else {
                        Some(Hai::Jihai(*num - 1))
                    }
                } else if *num != 1 {
                    Some(Hai::Jihai(*num - 1))
                } else {
                    None
                }
            }
        }
    }

    /// Return next hai. It means, like 2m for 1m.
    ///
    /// # Parameters
    /// * player_number: Number of players. No 2\~8m on 3-players mode.
    /// * dora_loop: If true, `Manzu(9).next()`, `Pinzu(9).next()` and
    /// `Souzu(9).next()` will be `Some(Manzu(1))`, `Some(Pinzu(1))` and
    /// `Some(souzu(1))`, `Jihai(4).next()` will be `Some(Jihai(1))`,
    /// `Jihai(7).next()` will be `Some(Jihai(5))`. Otherwise, `Manzu(1).next()`,
    /// `Pinzu(9).next()`, `Souzu(9).next()` and `Jihai(9).next()` will
    /// all be `None`.
    ///
    /// # Japanese
    /// * dora: ドラ
    pub fn next(&self, player_number: PlayerNumber, dora_loop: bool) -> Option<Hai> {
        match self {
            Hai::Manzu(num) => match player_number {
                PlayerNumber::Four => {
                    if *num != 9 {
                        Some(Hai::Manzu(*num + 1))
                    } else if dora_loop {
                        Some(Hai::Manzu(1))
                    } else {
                        None
                    }
                }
                PlayerNumber::Three => {
                    if dora_loop {
                        if *num == 1 {
                            Some(Hai::Manzu(9))
                        } else if *num == 9 {
                            Some(Hai::Manzu(1))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
            },
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

impl std::fmt::Display for Hai {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Hai::Manzu(num) => format!("{}m", num),
                Hai::Pinzu(num) => format!("{}p", num),
                Hai::Souzu(num) => format!("{}s", num),
                Hai::Jihai(num) => format!("{}z", num),
            }
        )
    }
}
