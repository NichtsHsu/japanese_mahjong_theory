use crate::mahjong::*;
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};

pub struct Game {
    haiyama: BTreeMap<Hai, u8>,
    tehai: Option<Tehai>,
    sutehai_type: BTreeSet<Hai>,
}

impl Game {
    /// Create empty game instance.
    pub fn new() -> Self {
        Game {
            haiyama: BTreeMap::new(),
            tehai: None,
            sutehai_type: BTreeSet::new(),
        }
    }

    pub fn initialize(&mut self) -> &mut Self {
        for hai in Hai::gen_all_type() {
            self.haiyama.insert(hai, 4);
        }
        self.sutehai_type.clear();
        self.tehai = None;
        self
    }

    pub fn set_tehai(&mut self, tehai: Tehai) -> &mut Self {
        self.tehai = Some(tehai);
        self
    }

    pub fn tehai(&mut self) -> Option<&mut Tehai> {
        self.tehai.as_mut()
    }

    pub fn to_json(&self) -> serde_json::Value {
        if self.haiyama.len() == 0 {
            json!({
                "error": "Not initialized!"
            })
        } else {
            let mut haiyama_json_vec = vec![];
            for (hai, number) in self.haiyama.iter() {
                haiyama_json_vec.push(json!({
                    hai.to_string(): number,
                }));
            }
            let haiyama_json = json!(haiyama_json_vec);

            let mut sutehai_type_string_vec = vec![];
            for hai in self.sutehai_type.iter() {
                sutehai_type_string_vec.push(hai.to_string());
            }
            let sutehai_json = json!(sutehai_type_string_vec);

            let tehai_json = match &self.tehai {
                Some(tehai) => tehai.to_json(),
                None => json!("Not initialized."),
            };

            json!({
                "haiyama": haiyama_json,
                "sutehai": sutehai_json,
                "tehai": tehai_json,
            })
        }
    }
}

impl ToString for Game {
    fn to_string(&self) -> String {
        if self.haiyama.len() == 0 {
            return "Not Initialized.".to_string();
        }

        let mut haiyama_string = String::new();
        for (hai, number) in self.haiyama.iter() {
            haiyama_string += &hai.to_string();
            haiyama_string += ":";
            haiyama_string += &number.to_string();
            match hai {
                Hai::Manzu(9) | Hai::Pinzu(9) | Hai::Souzu(9) => haiyama_string += "\n  ",
                _ => haiyama_string += " ",
            }
        }
        let mut sutehai_string = "".to_string();
        if self.sutehai_type.len() == 0 {
            sutehai_string += "無し";
        } else {
            for hai in self.sutehai_type.iter() {
                sutehai_string += &hai.to_string();
                sutehai_string += " ";
            }
        }
        format!(
            "牌山:\n  {}\n捨て牌の種類:\n  {}\n手牌:\n  {}",
            haiyama_string,
            sutehai_string,
            match &self.tehai {
                Some(tehai) => tehai.to_string(),
                None => "Not initialized.".to_string(),
            }
        )
    }
}
