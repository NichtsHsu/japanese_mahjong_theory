pub mod input {
    use crate::mahjong::*;

    pub fn parse(string: String) -> Tehai {
        use Hai::*;

        let mut menzen = vec![];
        let mut fuuro = vec![];
        let mut hai_stash = vec![];
        let mut menzen_stash = vec![];
        let mut on_mentsu = false;

        let push_into_hai_vec = |tile_type,
                                 index,
                                 stash: &mut Vec<char>,
                                 output: &mut Vec<Hai>|
         -> Result<(), String> {
            if stash.len() == 0 {
                Err(format!(
                    "Unused type character '{}' at index {}.",
                    tile_type, index
                ))
            } else {
                for tile in stash.iter() {
                    output.push(match tile_type {
                        'm' => Manzu(*tile as u8 - 48),
                        'p' => Pinzu(*tile as u8 - 48),
                        's' => Souzu(*tile as u8 - 48),
                        'z' => Jihai(*tile as u8 - 48),
                        _ => Manzu(0), // Never reach here.
                    })
                }
                stash.clear();
                Ok(())
            }
        };

        let mut push_into_fuuro = |index, stash: &mut Vec<Hai>| -> Result<(), String> {
            if let Some(meld) = check_mentsu(stash) {
                fuuro.push(meld);
                Ok(())
            } else {
                Err(format!("Not a valid meld on '[]' before index {}.", index))
            }
        };

        for (id, ch) in string.chars().enumerate() {
            match ch {
                'm' | 'p' | 's' | 'z' => {
                    if on_mentsu {
                        if let Err(error) =
                            push_into_hai_vec(ch, id, &mut hai_stash, &mut menzen_stash)
                        {
                            return Tehai::new(Err(error), fuuro);
                        }
                    } else {
                        if let Err(error) = push_into_hai_vec(ch, id, &mut hai_stash, &mut menzen) {
                            return Tehai::new(Err(error), fuuro);
                        }
                    }
                }
                '1'..='9' => hai_stash.push(ch),
                '[' => {
                    if on_mentsu {
                        return Tehai::new(
                            Err(format!("Second '[' found at index {}.", id)),
                            fuuro,
                        );
                    }
                    if hai_stash.len() > 0 {
                        return Tehai::new(
                            Err(format!(
                                "Need 'm' 'p' 's' 'z' but find '[' at index {}.",
                                id
                            )),
                            fuuro,
                        );
                    };
                    on_mentsu = true;
                }
                ']' => {
                    if !on_mentsu {
                        return Tehai::new(
                            Err(format!("Unmatched ']' found at index {}.", id)),
                            fuuro,
                        );
                    }
                    if hai_stash.len() > 0 {
                        return Tehai::new(
                            Err(format!(
                                "Need 'm' 'p' 's' 'z' but find ']' at index {}.",
                                id
                            )),
                            fuuro,
                        );
                    };
                    if let Err(error) = push_into_fuuro(id, &mut menzen_stash) {
                        return Tehai::new(Err(error), fuuro);
                    }
                    on_mentsu = false;
                }
                _ => {
                    return Tehai::new(
                        Err(format!("Unknown character '{}' at index {}.", ch, id)),
                        fuuro,
                    )
                }
            }
        }

        menzen.sort();
        Tehai::new(Ok(menzen), fuuro)
    }

    /// Check if tiles in range.
    /// # Examples
    /// ```rust
    /// use mahjong::Hai::*;
    /// assert_eq!(check_hai_in_range(vec![Manzu(1), Souzu(9), Jihai(7)]), true);
    /// assert_eq!(check_hai_in_range(vec![Manzu(4), Jihai(8)]), false);
    /// ```
    pub fn check_hai_in_range(hai_vec: &Vec<Hai>) -> bool {
        use Hai::*;
        for hai in hai_vec.iter() {
            match hai {
                Manzu(num) | Pinzu(num) | Souzu(num) => {
                    if *num < 1 || *num > 9 {
                        return false;
                    }
                }
                Jihai(num) => {
                    if *num < 1 || *num > 7 {
                        return false;
                    }
                }
            }
        }

        true
    }

    /// Check what mentsu type of input.
    /// # Examples
    /// ```rust
    /// use mahjong::Hai::*;
    /// use mahjong::Mentsu::*;
    /// assert_eq!(check_mentsu(vec![Manzu(1), Manzu(2), Manzu(3)]), Juntsu(Manzu(1), Manzu(2), Manzu(3)));
    /// assert_eq!(check_mentsu(vec![Pinzu(7), Pinzu(7), Pinzu(7)]), Koutsu(Pinzu(7)));
    /// assert_eq!(check_mentsu(vec![Jihai(8), Jihai(8), Jihai(8)]), None);
    /// ```
    pub fn check_mentsu(hai_vec: &Vec<Hai>) -> Option<Mentsu> {
        use Hai::*;
        use Mentsu::*;

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
        if !check_hai_in_range(hai_vec) {
            None
        } else if hai_vec.len() == 4 {
            if hai_vec[0] == hai_vec[1] && hai_vec[0] == hai_vec[2] && hai_vec[0] == hai_vec[3] {
                Some(Kantsu(hai_vec[0]))
            } else {
                None
            }
        } else if hai_vec.len() == 3 {
            if hai_vec[0] == hai_vec[1] && hai_vec[0] == hai_vec[2] {
                Some(Koutsu(hai_vec[0]))
            } else {
                match (hai_vec[0], hai_vec[1], hai_vec[2]) {
                    (Manzu(a), Manzu(b), Manzu(c)) => {
                        if let Some((a, b, c)) = check_juntsu(a, b, c) {
                            Some(Juntsu(Manzu(a), Manzu(b), Manzu(c)))
                        } else {
                            None
                        }
                    }
                    (Pinzu(a), Pinzu(b), Pinzu(c)) => {
                        if let Some((a, b, c)) = check_juntsu(a, b, c) {
                            Some(Juntsu(Pinzu(a), Pinzu(b), Pinzu(c)))
                        } else {
                            None
                        }
                    }
                    (Souzu(a), Souzu(b), Souzu(c)) => {
                        if let Some((a, b, c)) = check_juntsu(a, b, c) {
                            Some(Juntsu(Souzu(a), Souzu(b), Souzu(c)))
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }
        } else {
            None
        }
    }
}
