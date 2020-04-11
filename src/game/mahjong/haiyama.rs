use super::{Hai, PlayerNumber};
use serde_json::json;
use std::{collections::BTreeMap, ops::Index};

/// The haiyama struct.
///
/// # Japanese
/// * Haiyama: 牌山
#[derive(Clone, Debug)]
pub struct Haiyama {
    map: BTreeMap<Hai, u8>,
}

impl Haiyama {
    /// Create a new haiyama with 4 of each type of hai.
    pub fn new(player_number: PlayerNumber) -> Self {
        let mut map = BTreeMap::new();
        for hai in Hai::all_type(player_number) {
            map.insert(hai, 4);
        }
        Self { map }
    }

    /// Add one hai to haiyama, limited to 4.
    pub fn add(&mut self, hai: &Hai) -> Result<(), String> {
        let number = self.map[hai];
        if number < 4 {
            self.map.insert(*hai, number + 1);
            Ok(())
        } else {
            Err(format!(
                "Already 4 '{}' in haiyama, cannot add more one.",
                hai.to_string()
            ))
        }
    }

    /// Add a vec of hai to haiyama, limited to 4.
    ///
    /// # Parameters
    /// * auto_restore: If ture, haiyama will restore to original state
    /// when error occured.
    pub fn add_with_vec(&mut self, hai_vec: &Vec<Hai>, auto_restore: bool) -> Result<(), String> {
        let backup = if auto_restore {
            self.map.clone()
        } else {
            BTreeMap::new()
        };
        for hai in hai_vec {
            if let Err(error) = self.add(hai) {
                if auto_restore {
                    self.map = backup;
                }
                return Err(error);
            }
        }
        Ok(())
    }

    /// Discard one hai from haiyama.
    pub fn discard(&mut self, hai: &Hai) -> Result<(), String> {
        let number = self.map[hai];
        if number > 0 {
            self.map.insert(*hai, number - 1);
            Ok(())
        } else {
            Err(format!(
                "Already no '{}' in haiyama, cannot discard more one.",
                hai.to_string()
            ))
        }
    }

    /// Discard a vec of hai from haiyama.
    ///
    /// # Parameters
    /// * auto_restore: If ture, haiyama will restore to original state
    /// when error occured.
    pub fn discard_with_vec(
        &mut self,
        hai_vec: &Vec<Hai>,
        auto_restore: bool,
    ) -> Result<(), String> {
        let backup = if auto_restore {
            self.map.clone()
        } else {
            BTreeMap::new()
        };
        for hai in hai_vec {
            if let Err(_) = self.discard(hai) {
                if auto_restore {
                    self.map = backup;
                }
                return Err(format!(
                    "Not enough '{}' in haiyama to discard.",
                    hai.to_string()
                ));
            }
        }

        Ok(())
    }

    /// Print self to json.
    pub fn to_json(&self) -> serde_json::Value {
        let mut json_vec = vec![];
        for (hai, number) in &self.map {
            json_vec.push(json!({
                hai.to_string(): number,
            }));
        }
        json!(json_vec)
    }
}

impl<'a> Index<&'a Hai> for Haiyama {
    type Output = <BTreeMap<Hai, u8> as Index<&'a Hai>>::Output;

    fn index(&self, hai: &'a Hai) -> &Self::Output {
        self.map.index(hai)
    }
}

impl std::fmt::Display for Haiyama {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut haiyama_string = String::new();
        for (hai, number) in &self.map {
            haiyama_string += &hai.to_string();
            haiyama_string += ":";
            haiyama_string += &number.to_string();
            match hai {
                Hai::Manzu(9) | Hai::Pinzu(9) | Hai::Souzu(9) => haiyama_string += "\n  ",
                _ => haiyama_string += " ",
            }
        }
        write!(f, "{}", haiyama_string)
    }
}
