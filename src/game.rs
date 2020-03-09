use crate::{global, mahjong::*};
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug)]
pub enum InteractiveOperation {
    Initialize(Tehai),
    Add(Hai),
    Discard(Hai),
    HaiyamaAdd(Vec<Hai>),
    HaiyamaDiscard(Vec<Hai>),
    Chii(Mentsu, Hai),
    Pon(Mentsu),
    Kan(Mentsu, Option<Hai>),
    Daiminkan(Mentsu, Option<Hai>),
    Kakan(Mentsu, Option<Hai>),
    Ankan(Mentsu, Option<Hai>),
}

pub struct Game {
    haiyama: BTreeMap<Hai, u8>,
    tehai: Option<Tehai>,
    sutehai_type: BTreeSet<Hai>,
    operation_stack: Vec<(InteractiveOperation, global::InteractiveState)>,
}

impl Game {
    /// Create empty game instance.
    pub fn new() -> Self {
        Game {
            haiyama: BTreeMap::new(),
            tehai: None,
            sutehai_type: BTreeSet::new(),
            operation_stack: vec![],
        }
    }

    pub fn initialize(&mut self) -> &mut Self {
        for hai in Hai::gen_all_type() {
            self.haiyama.insert(hai, 4);
        }
        self.sutehai_type.clear();
        self.tehai = None;
        self.operation_stack.clear();
        self
    }

    pub fn tehai(&mut self) -> Option<&Tehai> {
        self.tehai.as_ref()
    }
    pub fn operate(&mut self, mut op: InteractiveOperation) -> Result<(), String> {
        fn haiyama_discard(haiyama: &mut BTreeMap<Hai, u8>, hai: &Hai) -> bool {
            let count = haiyama[hai];
            if count > 0 {
                haiyama.insert(*hai, count - 1);
                true
            } else {
                false
            }
        }
        
        let mut state = global::INTERACTIVE.write().unwrap();
        let last_state = *state;
        match last_state {
            global::InteractiveState::Noninteractive => {
                //Never reach here logically.
                panic!("Logic error: Noninteracive state at interactive mode.")
            }
            global::InteractiveState::WaitForFirstInput => match &op {
                InteractiveOperation::Initialize(tehai) => {
                    self.tehai = Some(tehai.clone());
                    *state = global::InteractiveState::FullTiles;
                }
                InteractiveOperation::HaiyamaAdd(hai_vec) => {
                    for hai in hai_vec {
                        self.haiyama.insert(*hai, self.haiyama[hai] + 1);
                    }
                }
                InteractiveOperation::HaiyamaDiscard(hai_vec) => {
                    let backup = self.haiyama.clone();
                    for hai in hai_vec {
                        if !haiyama_discard(&mut self.haiyama, hai) {
                            self.haiyama = backup;
                            return Err(format!("Cannot discard fifth {}.", hai.to_string()));
                        }
                    }
                }
                _ => {
                    return Err(format!(
                    "Only initialize, *- and *+ operations are supported at current state '{:?}'.",
                    last_state
                ))
                }
            },
            // Clone op for change op from Kan to Daiminkan/Kakan/Ankan.
            global::InteractiveState::FullTiles => match &op.clone() {
                InteractiveOperation::Discard(hai) => {
                    if let Some(tehai) = self.tehai.as_mut() {
                        match tehai.menzen.as_mut() {
                            Ok(menzen_vec) => {
                                let mut index = 9999;
                                for (i, item) in menzen_vec.iter().enumerate() {
                                    if item == hai {
                                        index = i;
                                        break;
                                    }
                                }
                                if index == 9999 {
                                    return Err(format!(
                                        "No enough {} to discard.",
                                        hai.to_string()
                                    ));
                                }
                                menzen_vec.remove(index);
                                *state = global::InteractiveState::LackOneTile;
                            }
                            Err(error) => return Err(error.clone()),
                        }
                    } else {
                        return Err("Not initialized.".to_string());
                    }
                }
                InteractiveOperation::HaiyamaAdd(hai_vec) => {
                    for hai in hai_vec {
                        self.haiyama.insert(*hai, self.haiyama[hai] + 1);
                    }
                }
                InteractiveOperation::HaiyamaDiscard(hai_vec) => {
                    let backup = self.haiyama.clone();
                    for hai in hai_vec {
                        if !haiyama_discard(&mut self.haiyama, hai) {
                            self.haiyama = backup;
                            return Err(format!("Cannot discard fifth {}.", hai.to_string()));
                        }
                    }
                }
                InteractiveOperation::Kan(kantsu, rinshan) => {
                    if let Mentsu::Kantsu(hai) = kantsu {
                        if let Some(tehai) = self.tehai.as_mut() {
                            match tehai.menzen.as_mut() {
                                Ok(menzen_vec) => {
                                    let mut hai_num = 0;
                                    let mut exist_koutsu = false;
                                    let mut exist_koutsu_index = 0;
                                    for i in menzen_vec.iter() {
                                        if i == hai {
                                            hai_num += 1;
                                        }
                                    }
                                    for (index, mentsu) in tehai.fuuro.iter().enumerate() {
                                        match mentsu {
                                            Mentsu::Kantsu(i) => {
                                                if i == hai {
                                                    exist_koutsu = true;
                                                    exist_koutsu_index = index;
                                                    break;
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    if hai_num == 1 && exist_koutsu {
                                        let mut index = 0;
                                        for (i, item) in menzen_vec.iter().enumerate() {
                                            if item == hai {
                                                index = i;
                                            }
                                        }
                                        menzen_vec.remove(index);
                                        tehai.fuuro[exist_koutsu_index] = *kantsu;
                                        op = InteractiveOperation::Kakan(*kantsu, *rinshan);
                                    } else if hai_num == 4 && !exist_koutsu {
                                        for _ in 0..4 {
                                            let mut index = 0;
                                            for (i, item) in menzen_vec.iter().enumerate() {
                                                if item == hai {
                                                    index = i;
                                                }
                                            }
                                            menzen_vec.remove(index);
                                        }
                                        tehai.fuuro.push(*kantsu);
                                        op = InteractiveOperation::Ankan(*kantsu, *rinshan);
                                    } else {
                                        return Err(format!(
                                            "No enough {} to take a kan.",
                                            hai.to_string()
                                        ));
                                    }
                                    if let Some(rinshanhai) = rinshan {
                                        let backup = self.haiyama.clone();
                                        if !haiyama_discard(&mut self.haiyama, rinshanhai) {
                                            self.haiyama = backup;
                                            return Err(format!(
                                                "Cannot discard fifth {}.",
                                                hai.to_string()
                                            ));
                                        }
                                        *state = global::InteractiveState::FullTiles;
                                        menzen_vec.push(*rinshanhai);
                                        menzen_vec.sort();
                                    } else {
                                        *state = global::InteractiveState::WaitForRinshanInput;
                                    }
                                }
                                Err(error) => return Err(error.clone()),
                            }
                        } else {
                            // Never reach here logically.
                            return Err(
                                "Logic error: Uninitialized while must be initialized.".to_string()
                            );
                        }
                    } else {
                        // Never reach here logically.
                        return Err("Logic error: Not a juntsu while must be juntsu.".to_string());
                    }
                }
                _ => {
                    return Err(format!(
                    "Only -, *-, *+ and >(kan) operations are supported at current state '{:?}'.",
                    last_state
                ))
                }
            },
            global::InteractiveState::LackOneTile => match &op.clone() {
                InteractiveOperation::HaiyamaAdd(hai_vec) => {
                    for hai in hai_vec {
                        self.haiyama.insert(*hai, self.haiyama[hai] + 1);
                    }
                }
                InteractiveOperation::HaiyamaDiscard(hai_vec) => {
                    let backup = self.haiyama.clone();
                    for hai in hai_vec {
                        if !haiyama_discard(&mut self.haiyama, hai) {
                            self.haiyama = backup;
                            return Err(format!("Cannot discard fifth {}.", hai.to_string()));
                        }
                    }
                }
                InteractiveOperation::Chii(juntsu, hai) => {
                    if let Mentsu::Juntsu(a, b, c) = juntsu {
                        let backup = self.haiyama.clone();
                        if !haiyama_discard(&mut self.haiyama, hai) {
                            self.haiyama = backup;
                            return Err(format!("Cannot discard fifth {}.", hai.to_string()));
                        }
                        if let Some(tehai) = self.tehai.as_mut() {
                            match tehai.menzen.as_mut() {
                                Ok(menzen_vec) => {
                                    for i in vec![a, b, c] {
                                        if i == hai {
                                            continue;
                                        }
                                        let mut index = 9999;
                                        for (j, item) in menzen_vec.iter().enumerate() {
                                            if item == i {
                                                index = j;
                                                break;
                                            }
                                        }
                                        if index == 9999 {
                                            return Err(format!(
                                                "No enough {} to discard.",
                                                hai.to_string()
                                            ));
                                        }
                                        menzen_vec.remove(index);
                                    }
                                    tehai.fuuro.push(*juntsu);
                                    *state = global::InteractiveState::FullTiles;
                                }
                                Err(error) => return Err(error.clone()),
                            }
                        } else {
                            // Never reach here logically.
                            return Err(
                                "Logic error: Uninitialized while must be initialized.".to_string()
                            );
                        }
                    } else {
                        // Never reach here logically.
                        return Err("Logic error: Not a juntsu while must be juntsu.".to_string());
                    }
                }
                InteractiveOperation::Pon(koutsu) => {
                    if let Mentsu::Koutsu(hai) = koutsu {
                        let backup = self.haiyama.clone();
                        if !haiyama_discard(&mut self.haiyama, hai) {
                            self.haiyama = backup;
                            return Err(format!("Cannot discard fifth {}.", hai.to_string()));
                        }
                        if let Some(tehai) = self.tehai.as_mut() {
                            match tehai.menzen.as_mut() {
                                Ok(menzen_vec) => {
                                    for _ in 0..2 {
                                        let mut index = 9999;
                                        for (i, item) in menzen_vec.iter().enumerate() {
                                            if item == hai {
                                                index = i;
                                                break;
                                            }
                                        }
                                        if index == 9999 {
                                            return Err(format!(
                                                "No enough {} to discard.",
                                                hai.to_string()
                                            ));
                                        }
                                        menzen_vec.remove(index);
                                    }
                                    tehai.fuuro.push(*koutsu);
                                    *state = global::InteractiveState::FullTiles;
                                }
                                Err(error) => return Err(error.clone()),
                            }
                        } else {
                            // Never reach here logically.
                            return Err(
                                "Logic error: Uninitialized while must be initialized.".to_string()
                            );
                        }
                    } else {
                        // Never reach here logically.
                        return Err("Logic error: Not a koutsu while must be koutsu.".to_string());
                    }
                }
                InteractiveOperation::Kan(kantsu, rinshan) => {
                    if let Mentsu::Kantsu(hai) = kantsu {
                        if let Some(tehai) = self.tehai.as_mut() {
                            match tehai.menzen.as_mut() {
                                Ok(menzen_vec) => {
                                    let mut hai_num = 0;
                                    for i in menzen_vec.iter() {
                                        if i == hai {
                                            hai_num += 1;
                                        }
                                    }
                                    let backup = self.haiyama.clone();
                                    if hai_num == 3 {
                                        if !haiyama_discard(&mut self.haiyama, hai) {
                                            self.haiyama = backup;
                                            return Err(format!(
                                                "Cannot discard fifth {}.",
                                                hai.to_string()
                                            ));
                                        }
                                        for _ in 0..3 {
                                            let mut index = 0;
                                            for (i, item) in menzen_vec.iter().enumerate() {
                                                if item == hai {
                                                    index = i;
                                                }
                                            }
                                            menzen_vec.remove(index);
                                        }
                                        tehai.fuuro.push(*kantsu);
                                        op = InteractiveOperation::Daiminkan(*kantsu, *rinshan);
                                    } else {
                                        return Err(format!(
                                            "Cannot discard fifth {}.",
                                            hai.to_string()
                                        ));
                                    }
                                    if let Some(rinshanhai) = rinshan {
                                        if !haiyama_discard(&mut self.haiyama, rinshanhai) {
                                            self.haiyama = backup;
                                            return Err(format!(
                                                "Cannot discard fifth {}.",
                                                hai.to_string()
                                            ));
                                        }
                                        *state = global::InteractiveState::FullTiles;
                                        menzen_vec.push(*rinshanhai);
                                        menzen_vec.sort();
                                    } else {
                                        *state = global::InteractiveState::WaitForRinshanInput;
                                    }
                                }
                                Err(error) => return Err(error.clone()),
                            }
                        } else {
                            // Never reach here logically.
                            return Err(
                                "Logic error: Uninitialized while must be initialized.".to_string()
                            );
                        }
                    } else {
                        // Never reach here logically.
                        return Err("Logic error: Not a juntsu while must be juntsu.".to_string());
                    }
                }
                _ => (),
            },
            global::InteractiveState::WaitForRinshanInput => {}
        }
        self.operation_stack.push((op, last_state));
        Ok(())
    }
    pub fn back(&mut self) {
        if let Some((op, state)) = self.operation_stack.pop() {}
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
        let state = *global::INTERACTIVE.read().unwrap();
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
            "牌山:\n  {}\n捨て牌の種類:\n  {}\n手牌:\n  {}\n状態:\n  {:?}",
            haiyama_string,
            sutehai_string,
            match &self.tehai {
                Some(tehai) => tehai.to_string(),
                None => "Not initialized.".to_string(),
            },
            state
        )
    }
}
