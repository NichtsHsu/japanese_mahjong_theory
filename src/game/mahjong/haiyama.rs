use super::{Hai, PlayerNumber};
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

    pub fn reinitialize(&mut self, player_number: PlayerNumber) {
        self.map.clear();
        for hai in Hai::all_type(player_number) {
            self.map.insert(hai, 4);
        }
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

    /// Add one hai to haiyama, without limit.
    pub fn force_add(&mut self, hai: &Hai) {
        let number = self.map[hai];
        self.map.insert(*hai, number + 1);
    }
}

impl<'a> Index<&'a Hai> for Haiyama {
    type Output = <BTreeMap<Hai, u8> as Index<&'a Hai>>::Output;

    fn index(&self, hai: &'a Hai) -> &Self::Output {
        self.map.index(hai)
    }
}
