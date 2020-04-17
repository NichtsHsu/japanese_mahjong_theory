use super::{Hai, Haiyama, MachiCondition, Mentsu, PlayerNumber, Tehai};
use serde_json::json;
use std::collections::BTreeSet;

/// The game manager.
/// Include everything that a complete mahjong game need.
#[derive(Clone, Debug)]
pub struct GameManager {
    haiyama: Haiyama,
    tehai: Option<Tehai>,
    sutehai_type: BTreeSet<Hai>,
    pub state: State,
    player_number: PlayerNumber,
    history: Vec<(Operation, State, BTreeSet<Hai>)>,
}

/// Type of kan.
///
/// # Japanese
/// * Daiminkan: 大明槓
/// * Kakan: 加槓
/// * Ankan: 暗槓
/// * kantsu: 槓子
/// * rinshanhai: 嶺上牌
#[derive(Clone, Debug)]
pub enum Kan {
    Daiminkan {
        kantsu: Mentsu,
        rinshanhai: Option<Hai>,
    },
    Kakan {
        kantsu: Mentsu,
        rinshanhai: Option<Hai>,
    },
    Ankan {
        kantsu: Mentsu,
        rinshanhai: Option<Hai>,
    },
    Unknown {
        kantsu: Mentsu,
        rinshanhai: Option<Hai>,
    },
}

/// Type of naku.
///
/// # Japanese
/// * Naku: 鳴く
/// * Chii: チー
/// * Pon: ポン
/// * Kan: カン
/// * nakihai: 鳴き牌
#[derive(Clone, Debug)]
pub enum Naku {
    Chii { juntsu: Mentsu, nakihai: Hai },
    Pon(Mentsu),
    Kan(Kan),
}

/// Operation on haiyama.
#[derive(Clone, Debug)]
pub enum HaiyamaOperation {
    Add(Vec<Hai>),
    Discard(Vec<Hai>),
}

/// Operation on tehai.
#[derive(Clone, Debug)]
pub enum TehaiOperation {
    Initialize(Tehai),
    Add { hai: Hai, haiyama_sensitive: bool },
    Discard(Hai),
    Naku { kind: Naku, haiyama_sensitive: bool },
}

/// Valid operation for game manager.
#[derive(Clone, Debug)]
pub enum Operation {
    Haiyama {
        kind: HaiyamaOperation,
        haiyama_sensitive: bool,
    },
    Tehai(TehaiOperation),
}

/// Game state.
#[derive(Copy, Clone, Debug)]
pub enum State {
    WaitToInit,
    FullHai,
    LackOneHai,
    WaitForRinshanhai,
}

impl Kan {
    pub fn to_json(&self) -> serde_json::Value {
        let (tp, kantsu, rinshanhai) = match self {
            Kan::Daiminkan { kantsu, rinshanhai } => ("daiminkan", kantsu, rinshanhai),
            Kan::Kakan { kantsu, rinshanhai } => ("kakan", kantsu, rinshanhai),
            Kan::Ankan { kantsu, rinshanhai } => ("ankan", kantsu, rinshanhai),
            Kan::Unknown { kantsu, rinshanhai } => ("unknown", kantsu, rinshanhai),
        };
        match rinshanhai {
            Some(hai) => json!({
                "type": tp,
                "kantsu": kantsu.to_json(),
                "rinshanhai": hai.to_string()
            }),
            None => json!({
                "type": tp,
                "kantsu": kantsu.to_json(),
                "rinshanhai": null
            }),
        }
    }
}

impl Naku {
    pub fn to_json(&self) -> serde_json::Value {
        match self {
            Naku::Chii { juntsu, nakihai } => json!({
                "type": "chii",
                "juntsu": juntsu.to_json(),
                "nakihai": nakihai.to_string(),
            }),
            Naku::Pon(koutsu) => json!({
                "type": "pon",
                "koutsu": koutsu.to_json(),
            }),
            Naku::Kan(kan) => json!({
                "type": "kan",
                "kan": kan.to_json(),
            }),
        }
    }
}

impl TehaiOperation {
    pub fn to_json(&self) -> serde_json::Value {
        match self {
            TehaiOperation::Initialize(tehai) => json!({
                "operation": "initialze",
                "tehai": tehai.to_json(),
            }),
            TehaiOperation::Add {
                hai,
                haiyama_sensitive,
            } => json!({
                "operation": "add",
                "hai": hai.to_string(),
                "haiyama_sensitive": haiyama_sensitive,
            }),
            TehaiOperation::Discard(hai) => json!({
                "operation": "discard",
                "hai": hai.to_string(),
            }),
            TehaiOperation::Naku {
                kind,
                haiyama_sensitive,
            } => json!({
                "operation": "naku",
                "naku": kind.to_json(),
                "haiyama_sensitive": haiyama_sensitive,
            }),
        }
    }
}

impl HaiyamaOperation {
    pub fn to_json(&self) -> serde_json::Value {
        match self {
            HaiyamaOperation::Add(hai_vec) => {
                let mut hai_string_vec = vec![];
                for i in hai_vec {
                    hai_string_vec.push(i.to_string());
                }
                json!({
                    "operation": "add",
                    "hai": hai_string_vec,
                })
            }
            HaiyamaOperation::Discard(hai_vec) => {
                let mut hai_string_vec = vec![];
                for i in hai_vec {
                    hai_string_vec.push(i.to_string());
                }
                json!({
                    "operation": "discard",
                    "hai": hai_string_vec,
                })
            }
        }
    }
}

impl Operation {
    pub fn to_json(&self) -> serde_json::Value {
        match self {
            Operation::Tehai(tehai_operation) => json!({
                "object": "tehai",
                "operation": tehai_operation.to_json(),
            }),
            Operation::Haiyama {
                kind,
                haiyama_sensitive,
            } => json!({
                "object": "tehai",
                "operation": kind.to_json(),
                "haiyama_sensitive": haiyama_sensitive,
            }),
        }
    }
}

impl GameManager {
    /// Create a instance of GameManager.
    pub fn new(player_number: PlayerNumber) -> Self {
        Self {
            haiyama: Haiyama::new(player_number),
            tehai: None,
            sutehai_type: BTreeSet::new(),
            state: State::WaitToInit,
            player_number,
            history: vec![],
        }
    }

    pub fn reinitialize(&mut self, player_number: PlayerNumber) -> &mut Self {
        *self = Self::new(player_number);
        self
    }

    /// Return a reference of haiyama
    pub fn haiyama(&self) -> &Haiyama {
        &self.haiyama
    }

    /// Return a reference of the set within sutehai.
    pub fn sutehai_type(&self) -> &BTreeSet<Hai> {
        &self.sutehai_type
    }

    /// Return operation history.
    pub fn history(&self) -> &Vec<(Operation, State, BTreeSet<Hai>)> {
        &self.history
    }

    pub fn tehai(&self) -> Option<&Tehai> {
        return self.tehai.as_ref();
    }

    pub fn tehai_analyze(&self) -> Result<(i32, Vec<MachiCondition>), String> {
        let tehai = self.tehai.as_ref().ok_or("Not initialized.".to_string())?;
        tehai.analyze(self.player_number, Some(&self))
    }

    /// Main function to control the game.
    pub fn operate(&mut self, mut op: Operation) -> Result<(), String> {
        let last_state = self.state;
        match last_state {
            State::WaitToInit => self.operate_wait_to_init(&op)?,
            State::FullHai => self.operate_full_hai(&mut op)?,
            State::LackOneHai => self.operate_lack_one_hai(&mut op)?,
            State::WaitForRinshanhai => self.operate_wait_for_rinshanhai(&op)?,
        }
        self.history
            .push((op, last_state, self.sutehai_type.clone()));
        Ok(())
    }

    pub fn back(&mut self, haiyama_sensitive: bool) -> Result<(Operation, State), String> {
        let (op, last_state, sutehai_type) = self
            .history
            .pop()
            .ok_or("No more operation history.".to_string())?;
        match match last_state {
            State::WaitToInit => self.back_wait_to_init(&op, haiyama_sensitive),
            State::FullHai => self.back_full_hai(&op, haiyama_sensitive),
            State::LackOneHai => self.back_lack_one_hai(&op, haiyama_sensitive),
            State::WaitForRinshanhai => self.back_wait_for_rinshanhai(&op, haiyama_sensitive),
        } {
            Ok(_) => {
                self.state = last_state;
                self.sutehai_type = sutehai_type;
                Ok((op, last_state))
            }
            Err(error) => {
                self.history.push((op, last_state, sutehai_type));
                Err(error)
            }
        }
    }

    /// Print self to json.
    pub fn to_json(&self) -> serde_json::Value {
        let mut sutehai_type_string_vec = vec![];
        for hai in self.sutehai_type.iter() {
            sutehai_type_string_vec.push(hai.to_string());
        }

        let tehai_json = match &self.tehai {
            Some(tehai) => tehai.to_json(),
            None => json!("Not initialized."),
        };

        json!({
            "haiyama": self.haiyama.to_json(),
            "sutehai_type": json!(sutehai_type_string_vec),
            "tehai": tehai_json,
        })
    }

    fn operate_wait_to_init(&mut self, op: &Operation) -> Result<(), String> {
        fn operate_tehai_init(self_: &mut GameManager, tehai: &Tehai) -> Result<(), String> {
            if tehai.fuuro.len() != 0 {
                return Err("Cannot initialized with fuuro.".to_string());
            }
            match tehai.juntehai.len() {
                13 => self_.state = State::LackOneHai,
                14 => self_.state = State::FullHai,
                num @ _ => {
                    return Err(format!(
                        "Cannot initialize tehai with {} juntehai, only 13 and 14 are supported.",
                        num
                    ))
                }
            }
            if let Err(error) = self_.haiyama.discard_with_vec(&tehai.juntehai, true) {
                self_.state = State::WaitToInit;
                return Err(error);
            }
            self_.tehai = Some(tehai.clone());

            Ok(())
        }

        match op {
            Operation::Tehai(TehaiOperation::Initialize(tehai)) => {
                operate_tehai_init(self, tehai)?;
            }
            Operation::Haiyama {
                kind: HaiyamaOperation::Add(hai_vec),
                haiyama_sensitive,
            } => {
                if let Err(error) = self.haiyama.add_with_vec(hai_vec, *haiyama_sensitive) {
                    if *haiyama_sensitive {
                        return Err(error);
                    }
                }
            }
            Operation::Haiyama {
                kind: HaiyamaOperation::Discard(hai_vec),
                haiyama_sensitive,
            } => {
                if let Err(error) = self.haiyama.discard_with_vec(hai_vec, *haiyama_sensitive) {
                    if *haiyama_sensitive {
                        return Err(error);
                    }
                }
            }
            _ => {
                return Err(format!(
                    "Unsupported opretion '{:?}' at state '{:?}'.",
                    op, self.state
                ))
            }
        }

        Ok(())
    }

    fn operate_full_hai(&mut self, op: &mut Operation) -> Result<(), String> {
        match &*op {
            Operation::Tehai(TehaiOperation::Discard(hai)) => {
                self.tehai.as_mut().unwrap().discard(hai)?;
                self.state = State::LackOneHai;
            }
            Operation::Tehai(TehaiOperation::Naku {
                kind: Naku::Kan(Kan::Unknown { kantsu, rinshanhai }),
                haiyama_sensitive,
            }) => {
                let haiyama_backup = self.haiyama.clone();
                let state_backup = self.state;
                let tehai_backup = self.tehai.clone();
                if let Some(rinshanhai) = rinshanhai {
                    if let Err(error) = self.haiyama.discard(rinshanhai) {
                        if *haiyama_sensitive {
                            return Err(error);
                        }
                    }
                    self.state = State::FullHai;
                } else {
                    self.state = State::WaitForRinshanhai;
                }

                match self.tehai.as_mut().unwrap().kan(kantsu, rinshanhai) {
                    Ok(kan) => {
                        if let Kan::Ankan { .. } | Kan::Kakan { .. } = &kan {
                            *op = Operation::Tehai(TehaiOperation::Naku {
                                kind: Naku::Kan(kan),
                                haiyama_sensitive: *haiyama_sensitive,
                            })
                        } else {
                            self.haiyama = haiyama_backup;
                            self.state = state_backup;
                            self.tehai = tehai_backup;
                            return Err(
                                "Logic error: Tehai currently is not able to kan.".to_string()
                            );
                        }
                    }
                    Err(error) => {
                        self.haiyama = haiyama_backup;
                        self.state = state_backup;
                        return Err(error);
                    }
                }
            }
            Operation::Haiyama {
                kind: HaiyamaOperation::Add(hai_vec),
                haiyama_sensitive,
            } => {
                if let Err(error) = self.haiyama.add_with_vec(hai_vec, *haiyama_sensitive) {
                    if *haiyama_sensitive {
                        return Err(error);
                    }
                }
            }
            Operation::Haiyama {
                kind: HaiyamaOperation::Discard(hai_vec),
                haiyama_sensitive,
            } => {
                if let Err(error) = self.haiyama.discard_with_vec(hai_vec, *haiyama_sensitive) {
                    if *haiyama_sensitive {
                        return Err(error);
                    }
                }
            }
            _ => {
                return Err(format!(
                    "Unsupported opretion '{:?}' at state '{:?}'.",
                    op, self.state
                ))
            }
        }

        Ok(())
    }

    fn operate_lack_one_hai(&mut self, op: &mut Operation) -> Result<(), String> {
        match &*op {
            Operation::Tehai(TehaiOperation::Add {
                hai,
                haiyama_sensitive,
            }) => {
                if let Err(error) = self.haiyama.discard(hai) {
                    if *haiyama_sensitive {
                        return Err(error);
                    }
                }
                self.tehai.as_mut().unwrap().juntehai.push(*hai);
                self.tehai.as_mut().unwrap().juntehai.sort();
                self.state = State::FullHai;
            }
            Operation::Tehai(TehaiOperation::Naku {
                kind: Naku::Chii { juntsu, nakihai },
                haiyama_sensitive,
            }) => {
                let haiyama_backup = self.haiyama.clone();
                if let Err(error) = self.haiyama.discard(nakihai) {
                    if *haiyama_sensitive {
                        return Err(error);
                    }
                }
                if let Err(error) = self.tehai.as_mut().unwrap().chii(juntsu, nakihai) {
                    self.haiyama = haiyama_backup;
                    return Err(error);
                }
                self.state = State::FullHai;
            }
            Operation::Tehai(TehaiOperation::Naku {
                kind: Naku::Pon(koutsu @ Mentsu::Koutsu(hai)),
                haiyama_sensitive,
            }) => {
                let haiyama_backup = self.haiyama.clone();
                if let Err(error) = self.haiyama.discard(hai) {
                    if *haiyama_sensitive {
                        return Err(error);
                    }
                }
                if let Err(error) = self.tehai.as_mut().unwrap().pon(koutsu) {
                    self.haiyama = haiyama_backup;
                    return Err(error);
                }
                self.state = State::FullHai;
            }
            Operation::Tehai(TehaiOperation::Naku {
                kind:
                    Naku::Kan(Kan::Unknown {
                        kantsu: kantsu @ Mentsu::Kantsu(hai),
                        rinshanhai,
                    }),
                haiyama_sensitive,
            }) => {
                let haiyama_backup = self.haiyama.clone();
                let state_backup = self.state;
                let tehai_backup = self.tehai.clone();
                if let Err(error) = self.haiyama.discard(hai) {
                    if *haiyama_sensitive {
                        return Err(error);
                    }
                }
                if let Some(rinshanhai) = rinshanhai {
                    if let Err(error) = self.haiyama.discard(rinshanhai) {
                        if *haiyama_sensitive {
                            self.haiyama = haiyama_backup;
                            return Err(error);
                        }
                    }
                    self.state = State::FullHai;
                } else {
                    self.state = State::WaitForRinshanhai;
                }

                match self.tehai.as_mut().unwrap().kan(kantsu, rinshanhai) {
                    Ok(kan) => {
                        if let Kan::Daiminkan { .. } = &kan {
                            *op = Operation::Tehai(TehaiOperation::Naku {
                                kind: Naku::Kan(kan),
                                haiyama_sensitive: *haiyama_sensitive,
                            })
                        } else {
                            self.haiyama = haiyama_backup;
                            self.state = state_backup;
                            self.tehai = tehai_backup;
                            return Err(
                                "Logic error: Tehai currently is not able to kan.".to_string()
                            );
                        }
                    }
                    Err(error) => {
                        self.haiyama = haiyama_backup;
                        self.state = state_backup;
                        return Err(error);
                    }
                }
            }
            Operation::Haiyama {
                kind: HaiyamaOperation::Add(hai_vec),
                haiyama_sensitive,
            } => {
                if let Err(error) = self.haiyama.add_with_vec(hai_vec, *haiyama_sensitive) {
                    if *haiyama_sensitive {
                        return Err(error);
                    }
                }
            }
            Operation::Haiyama {
                kind: HaiyamaOperation::Discard(hai_vec),
                haiyama_sensitive,
            } => {
                if let Err(error) = self.haiyama.discard_with_vec(hai_vec, *haiyama_sensitive) {
                    if *haiyama_sensitive {
                        return Err(error);
                    }
                }
            }
            _ => {
                return Err(format!(
                    "Unsupported opretion '{:?}' at state '{:?}'.",
                    op, self.state
                ))
            }
        }

        Ok(())
    }

    fn operate_wait_for_rinshanhai(&mut self, op: &Operation) -> Result<(), String> {
        match op {
            Operation::Tehai(TehaiOperation::Add {
                hai,
                haiyama_sensitive,
            }) => {
                if let Err(error) = self.haiyama.discard(hai) {
                    if *haiyama_sensitive {
                        return Err(error);
                    }
                }
                self.tehai.as_mut().unwrap().juntehai.push(*hai);
                self.tehai.as_mut().unwrap().juntehai.sort();
                self.state = State::FullHai;
            }
            Operation::Haiyama {
                kind: HaiyamaOperation::Add(hai_vec),
                haiyama_sensitive,
            } => {
                if let Err(error) = self.haiyama.add_with_vec(hai_vec, *haiyama_sensitive) {
                    if *haiyama_sensitive {
                        return Err(error);
                    }
                }
            }
            Operation::Haiyama {
                kind: HaiyamaOperation::Discard(hai_vec),
                haiyama_sensitive,
            } => {
                if let Err(error) = self.haiyama.discard_with_vec(hai_vec, *haiyama_sensitive) {
                    if *haiyama_sensitive {
                        return Err(error);
                    }
                }
            }
            _ => {
                return Err(format!(
                    "Unsupported opretion '{:?}' at state '{:?}'.",
                    op, self.state
                ))
            }
        }
        Ok(())
    }

    fn back_wait_to_init(&mut self, op: &Operation, haiyama_sensitive: bool) -> Result<(), String> {
        match op {
            Operation::Tehai(TehaiOperation::Initialize(tehai)) => {
                if let Err(error) = self
                    .haiyama
                    .add_with_vec(&tehai.juntehai, haiyama_sensitive)
                {
                    if haiyama_sensitive {
                        return Err(error);
                    }
                }
                self.tehai = None;
                self.state = State::WaitToInit;
            }
            Operation::Haiyama {
                kind: HaiyamaOperation::Add(hai_vec),
                ..
            } => {
                if let Err(error) = self.haiyama.discard_with_vec(hai_vec, haiyama_sensitive) {
                    if haiyama_sensitive {
                        return Err(error);
                    }
                }
            }
            Operation::Haiyama {
                kind: HaiyamaOperation::Discard(hai_vec),
                ..
            } => {
                if let Err(error) = self.haiyama.add_with_vec(hai_vec, haiyama_sensitive) {
                    if haiyama_sensitive {
                        return Err(error);
                    }
                }
            }
            _ => return Err(format!("Logic error: confused with impossible state.")),
        }
        Ok(())
    }

    fn back_full_hai(&mut self, op: &Operation, haiyama_sensitive: bool) -> Result<(), String> {
        match op {
            Operation::Tehai(TehaiOperation::Discard(hai)) => {
                self.tehai.as_mut().unwrap().juntehai.push(*hai);
                self.tehai.as_mut().unwrap().juntehai.sort();
            }
            Operation::Tehai(TehaiOperation::Naku {
                kind: Naku::Kan(kan),
                ..
            }) => {
                let backup = self.haiyama.clone();
                if let Kan::Daiminkan {
                    kantsu: Mentsu::Kantsu(hai),
                    ..
                } = kan
                {
                    if let Err(error) = self.haiyama.add(hai) {
                        if haiyama_sensitive {
                            return Err(error);
                        }
                    }
                }
                if let Some(rinshanhai) = match kan {
                    Kan::Daiminkan { rinshanhai, .. } => rinshanhai,
                    Kan::Kakan { rinshanhai, .. } => rinshanhai,
                    Kan::Ankan { rinshanhai, .. } => rinshanhai,
                    _ => {
                        self.haiyama = backup;
                        return Err(
                            "Logic error: Tehai::de_kan() can not accept Kan::Unknown.".to_string()
                        );
                    }
                } {
                    if let Err(error) = self.haiyama.add(rinshanhai) {
                        if haiyama_sensitive {
                            self.haiyama = backup;
                            return Err(error);
                        }
                    }
                }
                if let Err(error) = self.tehai.as_mut().unwrap().de_kan(kan) {
                    self.haiyama = backup;
                    return Err(error);
                }
            }
            Operation::Haiyama {
                kind: HaiyamaOperation::Add(hai_vec),
                ..
            } => {
                if let Err(error) = self.haiyama.discard_with_vec(hai_vec, haiyama_sensitive) {
                    if haiyama_sensitive {
                        return Err(error);
                    }
                }
            }
            Operation::Haiyama {
                kind: HaiyamaOperation::Discard(hai_vec),
                ..
            } => {
                if let Err(error) = self.haiyama.add_with_vec(hai_vec, haiyama_sensitive) {
                    if haiyama_sensitive {
                        return Err(error);
                    }
                }
            }
            _ => return Err(format!("Logic error: confused with impossible state.")),
        }
        Ok(())
    }

    fn back_lack_one_hai(&mut self, op: &Operation, haiyama_sensitive: bool) -> Result<(), String> {
        match op {
            Operation::Tehai(TehaiOperation::Add { hai, .. }) => {
                self.tehai.as_mut().unwrap().discard(hai)?;
            }
            Operation::Tehai(TehaiOperation::Naku {
                kind:
                    Naku::Chii {
                        juntsu: juntsu @ Mentsu::Juntsu(..),
                        nakihai,
                    },
                ..
            }) => {
                let backup = self.haiyama.clone();

                if let Err(error) = self.haiyama.add(nakihai) {
                    if haiyama_sensitive {
                        return Err(error);
                    }
                }
                if let Err(error) = self.tehai.as_mut().unwrap().de_chii(juntsu, nakihai) {
                    self.haiyama = backup;
                    return Err(error);
                }
            }
            Operation::Tehai(TehaiOperation::Naku {
                kind: Naku::Pon(koutsu @ Mentsu::Koutsu(hai)),
                ..
            }) => {
                let backup = self.haiyama.clone();
                if let Err(error) = self.haiyama.add(hai) {
                    if haiyama_sensitive {
                        return Err(error);
                    }
                }
                if let Err(error) = self.tehai.as_mut().unwrap().de_pon(koutsu) {
                    self.haiyama = backup;
                    return Err(error);
                }
            }
            Operation::Tehai(TehaiOperation::Naku {
                kind: Naku::Kan(kan),
                ..
            }) => {
                let backup = self.haiyama.clone();
                if let Kan::Daiminkan {
                    kantsu: Mentsu::Kantsu(hai),
                    ..
                } = kan
                {
                    if let Err(error) = self.haiyama.add(hai) {
                        if haiyama_sensitive {
                            return Err(error);
                        }
                    }
                }
                if let Some(rinshanhai) = match kan {
                    Kan::Daiminkan { rinshanhai, .. } => rinshanhai,
                    Kan::Kakan { rinshanhai, .. } => rinshanhai,
                    Kan::Ankan { rinshanhai, .. } => rinshanhai,
                    _ => {
                        self.haiyama = backup;
                        return Err(
                            "Logic error: Tehai::de_kan() can not accept Kan::Unknown.".to_string()
                        );
                    }
                } {
                    if let Err(error) = self.haiyama.add(rinshanhai) {
                        if haiyama_sensitive {
                            self.haiyama = backup;
                            return Err(error);
                        }
                    }
                }
                if let Err(error) = self.tehai.as_mut().unwrap().de_kan(kan) {
                    self.haiyama = backup;
                    return Err(error);
                }
            }
            Operation::Haiyama {
                kind: HaiyamaOperation::Add(hai_vec),
                ..
            } => {
                if let Err(error) = self.haiyama.discard_with_vec(hai_vec, haiyama_sensitive) {
                    if haiyama_sensitive {
                        return Err(error);
                    }
                }
            }
            Operation::Haiyama {
                kind: HaiyamaOperation::Discard(hai_vec),
                ..
            } => {
                if let Err(error) = self.haiyama.add_with_vec(hai_vec, haiyama_sensitive) {
                    if haiyama_sensitive {
                        return Err(error);
                    }
                }
            }
            _ => return Err(format!("Logic error: confused with impossible state.")),
        }
        Ok(())
    }

    fn back_wait_for_rinshanhai(
        &mut self,
        op: &Operation,
        haiyama_sensitive: bool,
    ) -> Result<(), String> {
        match op {
            Operation::Tehai(TehaiOperation::Add { hai, .. }) => {
                self.tehai.as_mut().unwrap().discard(hai)?;
            }
            Operation::Haiyama {
                kind: HaiyamaOperation::Add(hai_vec),
                ..
            } => {
                if let Err(error) = self.haiyama.discard_with_vec(hai_vec, haiyama_sensitive) {
                    if haiyama_sensitive {
                        return Err(error);
                    }
                }
            }
            Operation::Haiyama {
                kind: HaiyamaOperation::Discard(hai_vec),
                ..
            } => {
                if let Err(error) = self.haiyama.add_with_vec(hai_vec, haiyama_sensitive) {
                    if haiyama_sensitive {
                        return Err(error);
                    }
                }
            }
            _ => return Err(format!("Logic error: confused with impossible state.")),
        }
        Ok(())
    }
}

impl std::fmt::Display for GameManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sutehai_type_string = "".to_string();
        if self.sutehai_type.len() == 0 {
            sutehai_type_string += "無し";
        } else {
            for hai in self.sutehai_type.iter() {
                sutehai_type_string += &hai.to_string();
                sutehai_type_string += " ";
            }
        }

        write!(
            f,
            "牌山:\n  {}\n捨て牌の種類:\n  {}\n手牌:\n  {}\n状態:\n  {:?}",
            self.haiyama.to_string(),
            sutehai_type_string,
            match &self.tehai {
                Some(tehai) => tehai.to_string(),
                None => "Not initialized.".to_string(),
            },
            self.state
        )
    }
}
