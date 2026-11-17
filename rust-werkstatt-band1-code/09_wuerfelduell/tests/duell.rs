use wuerfelduell::{duell, Computer, Punkte, Seite, Zufall};

#[test]
fn aria_gegen_gorn_mit_festem_startwert() {
    let mut zufall = Zufall::new(88172645463325252);
    let mut aria = Computer::new("Aria", 4);
    let mut gorn = Computer::new("Gorn", 2);

    let ergebnis = duell(&mut aria, &mut gorn, &mut zufall, 3);

    assert_eq!(ergebnis.stand_links, Punkte(43));
    assert_eq!(ergebnis.stand_rechts, Punkte(47));
    assert_eq!(ergebnis.sieger(), Some(Seite::Rechts));
}

#[test]
fn erste_runde_stimmt_mit_der_handrechnung_ueberein() {
    let mut zufall = Zufall::new(88172645463325252);
    let mut aria = Computer::new("Aria", 4);
    let mut gorn = Computer::new("Gorn", 2);

    let ergebnis = duell(&mut aria, &mut gorn, &mut zufall, 1);
    let (zug_aria, zug_gorn) = &ergebnis.runden[0];

    assert_eq!(zug_aria.erster_wurf, [3, 2, 5]);
    assert_eq!(zug_aria.endwurf, [4, 5, 5]);
    assert_eq!(zug_aria.punkte, Punkte(14));
    assert_eq!(zug_gorn.endwurf, [6, 4, 6]);
    assert_eq!(zug_gorn.punkte, Punkte(16));
}
