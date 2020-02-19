use crate::mahjong;

pub fn parse_input_tiles(string: String) -> mahjong::Hand {
    let mut on_hand_tiles = vec![];
    let mut seizing_melds = vec![];
    let mut tile_stash = vec![];
    let mut meld_stash = vec![];
    let mut on_meld = false;

    let push_into_tiles = |tile_type,
                           index,
                           stash: &mut Vec<char>,
                           output: &mut Vec<mahjong::Tile>|
     -> Result<(), String> {
        if stash.len() == 0 {
            Err(format!(
                "Unused type character '{}' at index {}.",
                tile_type, index
            ))
        } else {
            for tile in stash.iter() {
                output.push(match tile_type {
                    'm' => mahjong::Tile::Character(*tile as u8 - 48),
                    'p' => mahjong::Tile::Dot(*tile as u8 - 48),
                    's' => mahjong::Tile::Bamboo(*tile as u8 - 48),
                    'z' => mahjong::Tile::Honor(*tile as u8 - 48),
                    _ => mahjong::Tile::Character(0), // Never reach here.
                })
            }
            stash.clear();
            Ok(())
        }
    };

    let mut push_into_meld = |index, stash: &mut Vec<mahjong::Tile>| -> Result<(), String> {
        if let Some(meld) = check_meld(stash) {
            seizing_melds.push(meld);
            Ok(())
        } else {
            Err(format!("Not a valid meld on '[]' before index {}.", index))
        }
    };

    for (id, ch) in string.chars().enumerate() {
        match ch {
            'm' | 'p' | 's' | 'z' => {
                if on_meld {
                    if let Err(error) = push_into_tiles(ch, id, &mut tile_stash, &mut meld_stash) {
                        return mahjong::Hand::new(Err(error), seizing_melds);
                    }
                } else {
                    if let Err(error) = push_into_tiles(ch, id, &mut tile_stash, &mut on_hand_tiles)
                    {
                        return mahjong::Hand::new(Err(error), seizing_melds);
                    }
                }
            }
            '1'..='9' => tile_stash.push(ch),
            '[' => {
                if on_meld {
                    return mahjong::Hand::new(
                        Err(format!("Second '[' found at index {}.", id)),
                        seizing_melds,
                    );
                }
                if tile_stash.len() > 0 {
                    return mahjong::Hand::new(
                        Err(format!(
                            "Need 'm' 'p' 's' 'z' but find '[' at index {}.",
                            id
                        )),
                        seizing_melds,
                    );
                };
                on_meld = true;
            }
            ']' => {
                if !on_meld {
                    return mahjong::Hand::new(
                        Err(format!("Unmatched ']' found at index {}.", id)),
                        seizing_melds,
                    );
                }
                if tile_stash.len() > 0 {
                    return mahjong::Hand::new(
                        Err(format!(
                            "Need 'm' 'p' 's' 'z' but find ']' at index {}.",
                            id
                        )),
                        seizing_melds,
                    );
                };
                if let Err(error) = push_into_meld(id, &mut meld_stash) {
                    return mahjong::Hand::new(Err(error), seizing_melds);
                }
                on_meld = false;
            }
            _ => {
                return mahjong::Hand::new(
                    Err(format!("Unknown character '{}' at index {}.", ch, id)),
                    seizing_melds,
                )
            }
        }
    }

    on_hand_tiles.sort();
    mahjong::Hand::new(Ok(on_hand_tiles), seizing_melds)
}

pub fn check_meld(tiles: &Vec<mahjong::Tile>) -> Option<mahjong::Meld> {
    use mahjong::Meld::*;
    use mahjong::Tile::*;

    fn check_chow(mut a: u8, mut b: u8, mut c: u8) -> Option<(u8, u8, u8)> {
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

    if tiles.len() == 4 {
        if tiles[0] == tiles[1] && tiles[0] == tiles[2] && tiles[0] == tiles[3] {
            Some(Kong(tiles[0]))
        } else {
            None
        }
    } else if tiles.len() == 3 {
        if tiles[0] == tiles[1] && tiles[0] == tiles[2] {
            Some(Pong(tiles[0]))
        } else {
            match (tiles[0], tiles[1], tiles[2]) {
                (Character(a), Character(b), Character(c)) => {
                    if let Some((a, b, c)) = check_chow(a, b, c) {
                        Some(Chow(Character(a), Character(b), Character(c)))
                    } else {
                        None
                    }
                }
                (Dot(a), Dot(b), Dot(c)) => {
                    if let Some((a, b, c)) = check_chow(a, b, c) {
                        Some(Chow(Dot(a), Dot(b), Dot(c)))
                    } else {
                        None
                    }
                }
                (Bamboo(a), Bamboo(b), Bamboo(c)) => {
                    if let Some((a, b, c)) = check_chow(a, b, c) {
                        Some(Chow(Bamboo(a), Bamboo(b), Bamboo(c)))
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
