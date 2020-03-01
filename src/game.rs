use crate::mahjong::*;
use serde_json::json;

pub struct Game {
    yama: Haiyama,
    tehai: Option<Tehai>,
}

impl Game {
    /// Create empty game instance.
    pub fn new() -> Self {
        Game {
            yama: Haiyama::new(),
            tehai: None,
        }
    }

    pub fn initialize(&mut self) -> &mut Self {
        self.yama.initialize();
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
        let [nokori_json, sutehai_json] = self.yama.to_json();
        let tehai_json = match &self.tehai {
            Some(tehai) => tehai.to_json(),
            None => json!("Not initialized."),
        };
        json!({
            "haiyama": nokori_json,
            "sutehai": sutehai_json,
            "tehai": tehai_json,
        })
    }
}

impl ToString for Game {
    fn to_string(&self) -> String {
        format!(
            "{}\n手牌:\n  {}",
            self.yama.to_string(),
            match &self.tehai {
                Some(tehai) => tehai.to_string(),
                None => "Not initialized.".to_string(),
            }
        )
    }
}
