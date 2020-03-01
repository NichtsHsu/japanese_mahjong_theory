use crate::mahjong::*;

pub struct Game {
    yama: Haiyama,
    tehai: Option<Tehai>,
}

impl Game {
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
