use super::{Hai, Haiyama, Mentsu, PlayerNumber, Tehai};
use std::collections::BTreeSet;

#[derive(Clone, Debug)]
pub struct GameManager {
    haiyama: Haiyama,
    tehai: Option<Tehai>,
    sutehai_type: BTreeSet<Hai>,
    state: State,
    history: Vec<(Operation, State)>,
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
    Add(Hai),
    Discard(Hai),
    Naku { kind: Naku, bound_check: bool },
}

/// Valid operation for game manager.
#[derive(Clone, Debug)]
pub enum Operation {
    Haiyama {
        kind: HaiyamaOperation,
        bound_check: bool,
    },
    Tehai(TehaiOperation),
}

/// Game state.
#[derive(Copy, Clone, Debug)]
enum State {
    WaitForInitialization,
    FourteenJuntehai,
    ThirteenJuntehai,
    WaitForRinshanhai,
}

impl GameManager {
    pub fn new(player_number: PlayerNumber) -> Self {
        Self {
            haiyama: Haiyama::new(player_number),
            tehai: None,
            sutehai_type: BTreeSet::new(),
            state: State::WaitForInitialization,
            history: vec![],
        }
    }

    pub fn haiyama(&self) -> &Haiyama {
        &self.haiyama
    }

    pub fn sutehai_type(&self) -> &BTreeSet<Hai> {
        &self.sutehai_type
    }

    pub fn operate(&mut self, mut op: Operation) {
        // TODO!
    }
}
