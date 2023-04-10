extern crate japanese_mahjong_theory;
use japanese_mahjong_theory::{Hai, Mentsu, PlayerNumber, Tehai};

#[test]
fn test_hai() {
    assert_eq!(
        Hai::Manzu(1).next(PlayerNumber::Four, false),
        Some(Hai::Manzu(2))
    );
    assert_eq!(
        Hai::Manzu(1).next(PlayerNumber::Four, true),
        Some(Hai::Manzu(2))
    );
    assert_eq!(
        Hai::Manzu(1).next(PlayerNumber::Three, true),
        Some(Hai::Manzu(9))
    );
    assert_eq!(Hai::Manzu(1).previous(PlayerNumber::Four, false), None);
    assert_eq!(
        Hai::Manzu(1).previous(PlayerNumber::Four, true),
        Some(Hai::Manzu(9))
    );
    assert_eq!(
        Hai::Jihai(1).previous(PlayerNumber::Four, true),
        Some(Hai::Jihai(4))
    );
    assert_eq!(
        Hai::Jihai(7).next(PlayerNumber::Four, true),
        Some(Hai::Jihai(5))
    );
}

#[test]
fn test_tehai_input() {
    let tehai = Tehai::new("99m2p [5555z] 1z12m 2p45s35m", PlayerNumber::Four).unwrap();
    assert_eq!(
        tehai.juntehai,
        Hai::from_string_unordered("123599m22p45s1z", PlayerNumber::Four).unwrap()
    );
    assert_eq!(tehai.fuuro, vec![Mentsu::Kantsu(Hai::Jihai(5))]);
}

#[test]
fn test_kokushimusou() {
    let tehai = Tehai::new("129m19p19s1234567z", PlayerNumber::Four).unwrap();
    let (shanten, machi) = tehai.analyze(PlayerNumber::Four, None).unwrap();
    assert_eq!(shanten, 0);
    assert_eq!(machi.len(), 1);
    assert_eq!(machi[0].sutehai, Hai::Manzu(2));
    assert_eq!(machi[0].machihai.len(), 13);
    assert_eq!(machi[0].machihai.iter().fold(0, |x, (_, &y)| x + y), 39);
    let tehai = Tehai::new("12m999p9s12345667z", PlayerNumber::Four).unwrap();
    let (shanten, machi) = tehai.analyze(PlayerNumber::Four, None).unwrap();
    assert_eq!(shanten, 2);
    assert_eq!(machi.len(), 3);
    assert_eq!(machi[0].sutehai, Hai::Manzu(2));
    assert_eq!(machi[1].sutehai, Hai::Pinzu(9));
    assert_eq!(machi[2].sutehai, Hai::Jihai(6));
    assert_eq!(machi[0].machihai.len(), 3);
    assert_eq!(machi[1].machihai.len(), 3);
    assert_eq!(machi[2].machihai.len(), 3);
    assert_eq!(machi[0].machihai.iter().fold(0, |x, (_, &y)| x + y), 12);
    assert_eq!(machi[1].machihai.iter().fold(0, |x, (_, &y)| x + y), 12);
    assert_eq!(machi[2].machihai.iter().fold(0, |x, (_, &y)| x + y), 12);
}

#[test]
fn test_chiitoutsu() {
    let tehai = Tehai::new("112233m4478p3557s", PlayerNumber::Four).unwrap();
    let (shanten, machi) = tehai.analyze(PlayerNumber::Four, None).unwrap();
    assert_eq!(shanten, 1);
    assert_eq!(machi.len(), 5);
    assert_eq!(machi[0].sutehai, Hai::Souzu(3));
    assert_eq!(machi[1].sutehai, Hai::Souzu(7));
    assert_eq!(machi[2].sutehai, Hai::Pinzu(7));
    assert_eq!(machi[3].sutehai, Hai::Pinzu(8));
    assert_eq!(machi[4].sutehai, Hai::Souzu(5));
    assert_eq!(machi[0].machihai.len(), 7);
    assert_eq!(machi[1].machihai.len(), 7);
    assert_eq!(machi[2].machihai.len(), 5);
    assert_eq!(machi[3].machihai.len(), 5);
    assert_eq!(machi[4].machihai.len(), 4);
    assert_eq!(machi[0].machihai.iter().fold(0, |x, (_, &y)| x + y), 21);
    assert_eq!(machi[1].machihai.iter().fold(0, |x, (_, &y)| x + y), 21);
    assert_eq!(machi[2].machihai.iter().fold(0, |x, (_, &y)| x + y), 17);
    assert_eq!(machi[3].machihai.iter().fold(0, |x, (_, &y)| x + y), 17);
    assert_eq!(machi[4].machihai.iter().fold(0, |x, (_, &y)| x + y), 15);
}
