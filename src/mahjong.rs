//! This mod defines all structures for an entire mahjong game.

/// Type of a tile.
///
/// # Glossary between English and Japanese
/// | English | Japanese | Romaji |
/// | ---- | ---- | ---- |
/// | Tile | 牌 | hai |
/// | Character| 萬子 | manzu |
/// | Dot | 筒子 | pinzu |
/// | Bamboo | 索子 | souzu |
/// | Honor | 字牌 | jihai |
#[derive(Copy, Clone, Debug, std::cmp::PartialEq, std::cmp::Eq, std::cmp::PartialOrd, std::cmp::Ord)]
pub enum Tile {
    Character(u8),
    Dot(u8),
    Bamboo(u8),
    Honor(u8),
}

/// Type of a meld.
///
/// # Glossary between English and Japanese
/// | English | Japanese | Romaji |
/// | ---- | ---- | ---- |
/// | Meld | 面子 | mentsu |
/// | Chow | 順子 | juntsu |
/// | Pong | 刻子 | koutsu |
/// | Kong | 槓子 | kantsu |
#[derive(Copy, Clone, Debug)]
pub enum Meld {
    Chow(Tile, Tile, Tile),
    Pong(Tile),
    Kong(Tile),
}

/// Eyes.
///
/// # Glossary between English and Japanese
/// | English | Japanese | Romaji |
/// | ---- | ---- | ---- |
/// | Eyes | 雀頭 | jantou |
pub struct Eyes(Tile);

/// Tiles on hand.
///
/// # Member
/// * on_hand_tiles : tiles not formed melds by seizing another's discard.
/// * seizing_melds : melds formed by seizing another's discard.
#[derive(Debug, Clone)]
pub struct Hand {
    on_hand_tiles: Result<Vec<Tile>, String>,
    seizing_melds: Vec<Meld>,
}

impl ToString for Tile {
    fn to_string(&self) -> String {
        match self {
            Tile::Character(num) => format!("{}m", num),
            Tile::Dot(num) => format!("{}p", num),
            Tile::Bamboo(num) => format!("{}s", num),
            Tile::Honor(num) => format!("{}z", num),
        }
    }
}

impl ToString for Meld {
    fn to_string(&self) -> String {
        match self {
            Meld::Chow(a, b, c) => format!("[{}{}{}]", a.to_string(), b.to_string(), c.to_string()),
            Meld::Pong(a) => {
                let tile = a.to_string();
                format!("[{}{}{}]", tile, tile, tile)
            }
            Meld::Kong(a) => {
                let tile = a.to_string();
                format!("[{}{}{}{}]", tile, tile, tile, tile)
            }
        }
    }
}

impl Hand {
    pub fn new(on_hand_tiles: Result<Vec<Tile>, String>, seizing_melds: Vec<Meld>) -> Self {
        Hand {
            on_hand_tiles,
            seizing_melds,
        }
    }

    pub fn empty() -> Self {
        Hand {
            on_hand_tiles: Err("No tiles on hand!".to_string()),
            seizing_melds: vec![],
        }
    }
}

impl From<String> for Hand {
    fn from(string: String) -> Self {
        crate::analyzer::parse_input_tiles(string)
    }
}

impl ToString for Hand {
    fn to_string(&self) -> String {
        let mut format_string = String::new();
        match &self.on_hand_tiles {
            Ok(tiles) => {
                for tile in tiles.iter() {
                    format_string += &tile.to_string();
                }
            }
            Err(error) => return error.clone(),
        };
        for meld in &self.seizing_melds {
            format_string += &meld.to_string();
        }

        format_string
    }
}
