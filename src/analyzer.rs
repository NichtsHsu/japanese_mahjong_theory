//! This mod include analyzers of tiles for their shanten number and waiting tiles.

/// This mod analyze input string.
pub mod input {
    use crate::mahjong::*;

    /// Parse a string to instance of Tehai.
    ///
    /// # Input
    /// You can input tiles out of order, and use [] represent formed melds
    /// (they will not be considered for shanten number).
    /// * stanard: `1m2m3m4m4m5m4p4p4p5p8s[1z1z1z]`
    /// * shorter: `123445m4445p8s[111z]`
    /// * out of order: `45p8s144m[111z]25m44p3m`
    ///
    /// Note that only 3*k+2 tiles out of '[]'(2, 5, 8, 11, 14, 17, 20...) can be parse.
    ///
    /// # Examples
    /// ```rust
    /// let tehai = analyzer::input::parse("45p8s144m[111z]25m44p3m".to_string());
    /// ```
    ///
    /// # Suggestion
    /// Use mahjong::Tehai::from() instead.
    /// ```rust
    /// let tehai = mahjong::Tehai::from("45p8s144m[111z]25m44p3m".to_string());
    /// ```
    pub fn parse(string: String) -> Tehai {
        use Hai::*;

        let mut menzen = vec![];
        let mut fuuro = vec![];
        let mut hai_stash = vec![];
        let mut menzen_stash = vec![];
        let mut on_mentsu = false;

        let push_into_hai_vec = |tile_type,
                                 index,
                                 stash: &mut Vec<char>,
                                 output: &mut Vec<Hai>|
         -> Result<(), String> {
            if stash.len() == 0 {
                Err(format!(
                    "Unused type character '{}' at index {}.",
                    tile_type, index
                ))
            } else {
                for tile in stash.iter() {
                    output.push(match tile_type {
                        'm' => Manzu(*tile as u8 - 48),
                        'p' => Pinzu(*tile as u8 - 48),
                        's' => Souzu(*tile as u8 - 48),
                        'z' => Jihai(*tile as u8 - 48),
                        _ => Manzu(0), // Never reach here.
                    })
                }
                stash.clear();
                Ok(())
            }
        };

        let mut push_into_fuuro = |index, stash: &mut Vec<Hai>| -> Result<(), String> {
            if let Some(meld) = check_mentsu(stash) {
                fuuro.push(meld);
                Ok(())
            } else {
                Err(format!("Not a valid meld on '[]' before index {}.", index))
            }
        };

        for (id, ch) in string.chars().enumerate() {
            match ch {
                'm' | 'p' | 's' | 'z' => {
                    if on_mentsu {
                        if let Err(error) =
                            push_into_hai_vec(ch, id, &mut hai_stash, &mut menzen_stash)
                        {
                            return Tehai::new(Err(error), fuuro);
                        }
                    } else {
                        if let Err(error) = push_into_hai_vec(ch, id, &mut hai_stash, &mut menzen) {
                            return Tehai::new(Err(error), fuuro);
                        }
                    }
                }
                '1'..='9' => hai_stash.push(ch),
                '[' => {
                    if on_mentsu {
                        return Tehai::new(
                            Err(format!("Second '[' found at index {}.", id)),
                            fuuro,
                        );
                    }
                    if hai_stash.len() > 0 {
                        return Tehai::new(
                            Err(format!(
                                "Need 'm' 'p' 's' 'z' but find '[' at index {}.",
                                id
                            )),
                            fuuro,
                        );
                    };
                    on_mentsu = true;
                }
                ']' => {
                    if !on_mentsu {
                        return Tehai::new(
                            Err(format!("Unmatched ']' found at index {}.", id)),
                            fuuro,
                        );
                    }
                    if hai_stash.len() > 0 {
                        return Tehai::new(
                            Err(format!(
                                "Need 'm' 'p' 's' 'z' but find ']' at index {}.",
                                id
                            )),
                            fuuro,
                        );
                    };
                    if let Err(error) = push_into_fuuro(id, &mut menzen_stash) {
                        return Tehai::new(Err(error), fuuro);
                    }
                    on_mentsu = false;
                }
                _ => {
                    return Tehai::new(
                        Err(format!("Unknown character '{}' at index {}.", ch, id)),
                        fuuro,
                    )
                }
            }
        }

        // Check if 3*k+2 tiles on menzen.
        if menzen.len() % 3 != 2 {
            Tehai::new(Err(format!("The number of tiles on hand must be 2*k+2, such as 8, 11, 14, even 17, but {} provided.", menzen.len())), fuuro)
        } else {
            menzen.sort();
            Tehai::new(Ok(menzen), fuuro)
        }
    }

    /// Check if tiles in range.
    ///
    /// # Examples
    /// ```rust
    /// use mahjong::Hai::*;
    /// assert_eq!(check_hai_in_range(vec![Manzu(1), Souzu(9), Jihai(7)]), true);
    /// assert_eq!(check_hai_in_range(vec![Manzu(4), Jihai(8)]), false);
    /// ```
    pub fn check_hai_in_range(hai_vec: &Vec<Hai>) -> bool {
        use Hai::*;
        for hai in hai_vec.iter() {
            match hai {
                Manzu(num) | Pinzu(num) | Souzu(num) => {
                    if *num < 1 || *num > 9 {
                        return false;
                    }
                }
                Jihai(num) => {
                    if *num < 1 || *num > 7 {
                        return false;
                    }
                }
            }
        }

        true
    }

    /// Check what mentsu type of input.
    ///
    /// # Examples
    /// ```rust
    /// use mahjong::Hai::*;
    /// use mahjong::Mentsu::*;
    /// assert_eq!(check_mentsu(vec![Manzu(1), Manzu(2), Manzu(3)]), Juntsu(Manzu(1), Manzu(2), Manzu(3)));
    /// assert_eq!(check_mentsu(vec![Pinzu(7), Pinzu(7), Pinzu(7)]), Koutsu(Pinzu(7)));
    /// assert_eq!(check_mentsu(vec![Jihai(8), Jihai(8), Jihai(8)]), None);
    /// ```
    pub fn check_mentsu(hai_vec: &Vec<Hai>) -> Option<Mentsu> {
        use Hai::*;
        use Mentsu::*;

        fn check_juntsu(mut a: u8, mut b: u8, mut c: u8) -> Option<(u8, u8, u8)> {
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
        if !check_hai_in_range(hai_vec) {
            None
        } else if hai_vec.len() == 4 {
            if hai_vec[0] == hai_vec[1] && hai_vec[0] == hai_vec[2] && hai_vec[0] == hai_vec[3] {
                Some(Kantsu(hai_vec[0]))
            } else {
                None
            }
        } else if hai_vec.len() == 3 {
            if hai_vec[0] == hai_vec[1] && hai_vec[0] == hai_vec[2] {
                Some(Koutsu(hai_vec[0]))
            } else {
                match (hai_vec[0], hai_vec[1], hai_vec[2]) {
                    (Manzu(a), Manzu(b), Manzu(c)) => {
                        if let Some((a, b, c)) = check_juntsu(a, b, c) {
                            Some(Juntsu(Manzu(a), Manzu(b), Manzu(c)))
                        } else {
                            None
                        }
                    }
                    (Pinzu(a), Pinzu(b), Pinzu(c)) => {
                        if let Some((a, b, c)) = check_juntsu(a, b, c) {
                            Some(Juntsu(Pinzu(a), Pinzu(b), Pinzu(c)))
                        } else {
                            None
                        }
                    }
                    (Souzu(a), Souzu(b), Souzu(c)) => {
                        if let Some((a, b, c)) = check_juntsu(a, b, c) {
                            Some(Juntsu(Souzu(a), Souzu(b), Souzu(c)))
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
}

/// This mod calculate the shanten number of a hand of tiles.
pub mod shanten {
    use crate::mahjong::*;

    pub fn shanten_number(tehai: &Tehai) -> Result<(i32, Vec<Decomposer>), String> {
        use Hai::*;

        let menzen_vec = tehai.menzen.as_ref()?;
        let mut min_shanten_number = ((menzen_vec.len() / 3) * 2) as i32;
        let mut min_shanten_decomposers = vec![];

        let mut push_into_decomposers = |decomposer: Decomposer| {
            if decomposer.shanten_number(menzen_vec.len()) == min_shanten_number {
                min_shanten_decomposers.push(decomposer);
            } else if decomposer.shanten_number(menzen_vec.len()) < min_shanten_number {
                min_shanten_number = decomposer.shanten_number(menzen_vec.len());
                min_shanten_decomposers.clear();
                min_shanten_decomposers.push(decomposer);
            }
        };

        // Analyze Mentsute
        {
            let mut decomposers_vec = vec![];
            split(tehai, &mut decomposers_vec, &mut Decomposer::new(), 0)?;
            for mut decomposer in decomposers_vec {
                decomposer.hourakei = Hourakei::Mentsute;
                push_into_decomposers(decomposer);
            }
        }

        // Analyze Chiitoitsu
        if menzen_vec.len() % 6 == 2 {
            let mut decomposer = Decomposer::new();
            decomposer.hourakei = Hourakei::Chiitoitsu;

            let mut menzen_iter = menzen_vec.iter();
            let mut last_hai_used = false;

            if let Some(mut last_hai) = menzen_iter.next() {
                loop {
                    if let Some(cur) = menzen_iter.next() {
                        if cur == last_hai {
                            if !last_hai_used {
                                last_hai_used = true;
                                decomposer.toitsu(Toitsu { 0: *cur });
                            } else {
                                decomposer.ukihai(Ukihai { 0: *cur });
                            }
                        } else {
                            if !last_hai_used {
                                decomposer.chiitoutsu(*last_hai);
                            }
                            last_hai = cur;
                            last_hai_used = false;
                        }
                    } else {
                        if !last_hai_used {
                            decomposer.chiitoutsu(*last_hai);
                        }
                        break;
                    }
                }

                push_into_decomposers(decomposer);
            }
        }

        // Analyze Kokushimusou
        if menzen_vec.len() == 14 {
            const YAOCHUUPAI: [Hai; 13] = [
                Manzu(1),
                Manzu(9),
                Pinzu(1),
                Pinzu(9),
                Souzu(1),
                Souzu(9),
                Jihai(1),
                Jihai(2),
                Jihai(3),
                Jihai(4),
                Jihai(5),
                Jihai(6),
                Jihai(7),
            ];

            let mut decomposer = Decomposer::new();
            let mut toitsu_included = false;
            let mut yaochuupai_iter_changed = true;
            decomposer.hourakei = Hourakei::Kokushimusou;

            let mut yaochuupai_iter = YAOCHUUPAI.iter();
            let mut menzen_iter = menzen_vec.iter();
            let mut yaochuupai_value = yaochuupai_iter.next();
            let mut menzen_value = menzen_iter.next();

            while yaochuupai_value != None && menzen_value != None {
                if let (Some(lhs), Some(rhs)) = (yaochuupai_value, menzen_value) {
                    if lhs < rhs {
                        yaochuupai_value = yaochuupai_iter.next();
                        yaochuupai_iter_changed = true;
                    } else if lhs > rhs {
                        decomposer.ukihai(Ukihai { 0: *rhs });
                        menzen_value = menzen_iter.next();
                    } else if lhs == rhs {
                        if yaochuupai_iter_changed {
                            decomposer.kokushimusou(*rhs);
                        } else if !toitsu_included {
                            toitsu_included = true;
                            decomposer.kokushimusou(*rhs);
                        } else {
                            decomposer.ukihai(Ukihai { 0: *rhs });
                        }
                        yaochuupai_iter_changed = false;
                        menzen_value = menzen_iter.next();
                    }
                }
            }
            push_into_decomposers(decomposer);
        }

        Ok((min_shanten_number, min_shanten_decomposers))
    }

    /// Type of tiles when winning.
    ///
    /// # Note
    /// * Only 14 tiles can be Kokushimusou.
    /// * Only 6*k+2 tiles can be Chiitoitsu. 8 tiles -> four pairs,
    /// 14 tiles -> seven pairs, 20 tiles -> ten pairs ...
    ///
    /// # Japanese
    /// * Hourakei: 和了形
    /// * Mentsute: 面子手
    /// * Chiitoitsu: 七対子
    /// * Kokushimusou: 国士無双
    #[derive(Copy, Clone, Debug)]
    pub enum Hourakei {
        Mentsute,
        Chiitoitsu,
        Kokushimusou,
    }

    /// Decompose tiles to mentsu, toitsu, taatsu and ukihai.
    ///
    /// # Note
    /// When hourakei is Kokushimusou, ukihai_vec only record 1m9m1p9p1s9s1z2z3z4z5z6z7z
    /// and at most a pair of same tiles.
    #[derive(Clone, Debug)]
    pub struct Decomposer {
        mentsu_vec: Vec<Mentsu>,
        toitsu_vec: Vec<Toitsu>,
        taatsu_vec: Vec<Taatsu>,
        ukihai_vec: Vec<Ukihai>,
        chiitoutsu_kokushimusou_valid_tile_vec: Vec<Hai>,
        hourakei: Hourakei,
    }

    impl Decomposer {
        fn new() -> Decomposer {
            Decomposer {
                mentsu_vec: vec![],
                toitsu_vec: vec![],
                taatsu_vec: vec![],
                ukihai_vec: vec![],
                chiitoutsu_kokushimusou_valid_tile_vec: vec![],
                hourakei: Hourakei::Mentsute,
            }
        }

        fn mentsu(&mut self, mentsu: Mentsu) -> &mut Self {
            self.mentsu_vec.push(mentsu);
            self
        }

        fn toitsu(&mut self, toitsu: Toitsu) -> &mut Self {
            self.toitsu_vec.push(toitsu);
            self
        }

        fn taatsu(&mut self, taatsu: Taatsu) -> &mut Self {
            self.taatsu_vec.push(taatsu);
            self
        }

        fn ukihai(&mut self, ukihai: Ukihai) -> &mut Self {
            self.ukihai_vec.push(ukihai);
            self
        }

        fn chiitoutsu(&mut self, hai: Hai) -> &mut Self {
            self.chiitoutsu_kokushimusou_valid_tile_vec.push(hai);
            self
        }

        fn kokushimusou(&mut self, hai: Hai) -> &mut Self {
            self.chiitoutsu_kokushimusou_valid_tile_vec.push(hai);
            self
        }

        pub fn mentsu_vec(&self) -> &Vec<Mentsu> {
            &self.mentsu_vec
        }

        pub fn toitsu_vec(&self) -> &Vec<Toitsu> {
            &self.toitsu_vec
        }

        pub fn taatsu_vec(&self) -> &Vec<Taatsu> {
            &self.taatsu_vec
        }

        pub fn ukihai_vec(&self) -> &Vec<Ukihai> {
            &self.ukihai_vec
        }

        pub fn hourakei(&self) -> Hourakei {
            self.hourakei
        }

        pub fn shanten_number(&self, hai_number: usize) -> i32 {
            match self.hourakei {
                Hourakei::Mentsute => {
                    let max_mentsu_toitsu_taatsu = (hai_number + 1) / 3;
                    let toitsu_num = std::cmp::min(
                        max_mentsu_toitsu_taatsu - self.mentsu_vec().len(),
                        self.toitsu_vec.len(),
                    );
                    let taatsu_num = std::cmp::min(
                        max_mentsu_toitsu_taatsu - self.mentsu_vec().len() - toitsu_num,
                        self.taatsu_vec().len(),
                    );
                    return ((hai_number / 3) * 2) as i32
                        - 2 * self.mentsu_vec.len() as i32
                        - toitsu_num as i32
                        - taatsu_num as i32;
                }
                Hourakei::Chiitoitsu => {
                    return hai_number as i32
                        - 1
                        - 2 * self.toitsu_vec.len() as i32
                        - std::cmp::min(
                            self.chiitoutsu_kokushimusou_valid_tile_vec.len(),
                            7 - self.toitsu_vec.len(),
                        ) as i32;
                }
                Hourakei::Kokushimusou => {
                    return 13 - self.chiitoutsu_kokushimusou_valid_tile_vec.len() as i32;
                }
            }
        }
    }

    impl std::fmt::Display for Decomposer {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut mentsu_string = String::new();
            let mut toitsu_string = String::new();
            let mut taatsu_string = String::new();
            let mut chiitoutsu_kokushimusou_valid_tile_string = String::new();
            let mut ukihai_string = String::new();
            let hourakei_string;

            if self.mentsu_vec.len() > 0 {
                mentsu_string = String::from("面子:");

                for mentsu in &self.mentsu_vec {
                    mentsu_string += &mentsu.to_string();
                    mentsu_string += " ";
                }
                mentsu_string += "\n";
            }

            if self.toitsu_vec.len() > 0 {
                toitsu_string = String::from("对子:");

                for toitsu in &self.toitsu_vec {
                    toitsu_string += &toitsu.to_string();
                    toitsu_string += " ";
                }
                toitsu_string += "\n";
            }

            if self.taatsu_vec.len() > 0 {
                taatsu_string = String::from("搭子:");

                for taatsu in &self.taatsu_vec {
                    taatsu_string += &taatsu.to_string();
                    taatsu_string += " ";
                }
                taatsu_string += "\n";
            }

            if self.ukihai_vec.len() > 0 {
                ukihai_string = String::from("浮牌:");

                for ukihai in &self.ukihai_vec {
                    ukihai_string += &ukihai.to_string();
                    ukihai_string += " ";
                }
                ukihai_string += "\n";
            }

            if self.chiitoutsu_kokushimusou_valid_tile_vec.len() > 0 {
                chiitoutsu_kokushimusou_valid_tile_string =
                    String::from("七対子/国士無双の有効牌:");

                for chiitoutsu_kokushimusou_valid_tile in
                    &self.chiitoutsu_kokushimusou_valid_tile_vec
                {
                    chiitoutsu_kokushimusou_valid_tile_string +=
                        &chiitoutsu_kokushimusou_valid_tile.to_string();
                    chiitoutsu_kokushimusou_valid_tile_string += " ";
                }
                chiitoutsu_kokushimusou_valid_tile_string += "\n";
            }

            match &self.hourakei {
                Hourakei::Mentsute => hourakei_string = "和了形:面子手\n".to_string(),
                Hourakei::Chiitoitsu => hourakei_string = "和了形:七对子\n".to_string(),
                Hourakei::Kokushimusou => hourakei_string = "和了形:国士无双\n".to_string(),
            }

            write!(
                f,
                "{}{}{}{}{}{}",
                hourakei_string,
                mentsu_string,
                toitsu_string,
                taatsu_string,
                chiitoutsu_kokushimusou_valid_tile_string,
                ukihai_string
            )
        }
    }

    /// # Reference
    /// * http://choco.properties/2019/06/22/%E6%97%A5%E9%BA%BB%E6%8A%98%E8%85%BE%E7%AC%94%E8%AE%B0-02-%E5%90%91%E5%90%AC%E6%95%B0%E7%9A%84%E5%88%A4%E6%96%AD/
    /// * Original author: 天羽ちよこ
    fn split(
        tehai: &Tehai,
        decomposers_vec: &mut Vec<Decomposer>,
        decomposer: &mut Decomposer,
        depth: usize,
    ) -> Result<(), String> {
        use Mentsu::*;
        fn remove_once<T: Eq>(container: &mut Vec<T>, item: &T) {
            for (index, cur) in container.iter().enumerate() {
                if cur == item {
                    container.remove(index);
                    break;
                }
            }
        }

        fn handle_ukihai(
            tehai: &Tehai,
            decomposers_vec: &mut Vec<Decomposer>,
            decomposer: &mut Decomposer,
            ukihai: Hai,
            depth: usize,
        ) -> Result<(), String> {
            let mut tehai = tehai.clone();
            decomposer.ukihai(Ukihai { 0: ukihai });
            let mut menzen_vec = tehai.menzen?;
            remove_once(&mut menzen_vec, &ukihai);
            tehai.menzen = Ok(menzen_vec);
            split(&tehai, decomposers_vec, decomposer, depth)
        }

        fn handle_taatsu(
            tehai: &Tehai,
            decomposers_vec: &mut Vec<Decomposer>,
            decomposer: &mut Decomposer,
            lhs: Hai,
            rhs: Hai,
            depth: usize,
        ) -> Result<(), String> {
            let mut tehai = tehai.clone();
            decomposer.taatsu(Taatsu { 0: lhs, 1: rhs });
            let mut menzen_vec = tehai.menzen?;
            remove_once(&mut menzen_vec, &lhs);
            remove_once(&mut menzen_vec, &rhs);
            tehai.menzen = Ok(menzen_vec);
            split(&tehai, decomposers_vec, decomposer, depth)
        }

        fn handle_toitsu(
            tehai: &Tehai,
            decomposers_vec: &mut Vec<Decomposer>,
            decomposer: &mut Decomposer,
            toitsu: Hai,
            depth: usize,
        ) -> Result<(), String> {
            let mut tehai = tehai.clone();
            decomposer.toitsu(Toitsu { 0: toitsu });
            let mut menzen_vec = tehai.menzen?;
            remove_once(&mut menzen_vec, &toitsu);
            remove_once(&mut menzen_vec, &toitsu);
            tehai.menzen = Ok(menzen_vec);
            split(&tehai, decomposers_vec, decomposer, depth)
        }

        fn handle_juntsu(
            tehai: &Tehai,
            decomposers_vec: &mut Vec<Decomposer>,
            decomposer: &mut Decomposer,
            first: Hai,
            second: Hai,
            third: Hai,
            depth: usize,
        ) -> Result<(), String> {
            let mut tehai = tehai.clone();
            decomposer.mentsu(Juntsu(first, second, third));
            let mut menzen_vec = tehai.menzen?;
            remove_once(&mut menzen_vec, &first);
            remove_once(&mut menzen_vec, &second);
            remove_once(&mut menzen_vec, &third);
            tehai.menzen = Ok(menzen_vec);
            split(&tehai, decomposers_vec, decomposer, depth)
        }

        fn handle_koutsu(
            tehai: &Tehai,
            decomposers_vec: &mut Vec<Decomposer>,
            decomposer: &mut Decomposer,
            koutsu: Hai,
            depth: usize,
        ) -> Result<(), String> {
            let mut tehai = tehai.clone();
            decomposer.mentsu(Koutsu(koutsu));
            let mut menzen_vec = tehai.menzen?;
            remove_once(&mut menzen_vec, &koutsu);
            remove_once(&mut menzen_vec, &koutsu);
            remove_once(&mut menzen_vec, &koutsu);
            tehai.menzen = Ok(menzen_vec);
            split(&tehai, decomposers_vec, decomposer, depth)
        }

        let menzen_vec = tehai.menzen.as_ref()?;
        if menzen_vec.len() == 1 {
            decomposer.ukihai(Ukihai { 0: menzen_vec[0] });
        }
        if menzen_vec.len() <= 1 {
            decomposers_vec.push(decomposer.clone());
            return Ok(());
        }

        let current = menzen_vec[0];
        let next = menzen_vec[1];
        let next_next = menzen_vec.get(2);

        if current == next {
            handle_toitsu(
                tehai,
                decomposers_vec,
                &mut decomposer.clone(),
                current,
                depth + 1,
            )?;
        }

        if let Some(&next_next) = next_next {
            if current == next && current == next_next {
                handle_koutsu(
                    tehai,
                    decomposers_vec,
                    &mut decomposer.clone(),
                    current,
                    depth + 1,
                )?;
            }
        }

        match current {
            Hai::Jihai(_) => (),
            _ => {
                let current_plus_one = current.next(false);
                if let Some(current_plus_one) = current_plus_one {
                    let filtered: Vec<&Hai> = menzen_vec
                        .iter()
                        .filter(|&x| x == &current_plus_one)
                        .collect();
                    if filtered.len() > 0 {
                        handle_taatsu(
                            tehai,
                            decomposers_vec,
                            &mut decomposer.clone(),
                            current,
                            current_plus_one,
                            depth + 1,
                        )?;
                        let current_plus_two = current_plus_one.next(false);
                        if let Some(current_plus_two) = current_plus_two {
                            let filtered: Vec<&Hai> = menzen_vec
                                .iter()
                                .filter(|&x| x == &current_plus_two)
                                .collect();
                            if filtered.len() > 0 {
                                handle_juntsu(
                                    tehai,
                                    decomposers_vec,
                                    &mut decomposer.clone(),
                                    current,
                                    current_plus_one,
                                    current_plus_two,
                                    depth + 1,
                                )?;
                            }
                        }
                    }
                }
            }
        };

        handle_ukihai(
            tehai,
            decomposers_vec,
            &mut decomposer.clone(),
            current,
            depth + 1,
        )
    }
}
