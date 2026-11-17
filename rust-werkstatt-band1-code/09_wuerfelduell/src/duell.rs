use crate::wertung::wertung;
use crate::{Punkte, Spieler, Zufall};

/// Ein Zug: erster Wurf, Wurf nach dem Neuwurf und die Punkte dafür.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zug {
    pub name: String,
    pub erster_wurf: Vec<u32>,
    pub endwurf: Vec<u32>,
    pub punkte: Punkte,
}

/// Wer gewonnen hat.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Seite {
    Links,
    Rechts,
}

/// Das Ergebnis eines Duells: alle Runden und der Endstand.
#[derive(Debug)]
pub struct Ergebnis {
    pub runden: Vec<(Zug, Zug)>,
    pub stand_links: Punkte,
    pub stand_rechts: Punkte,
}

impl Ergebnis {
    /// Der Sieger, oder `None` bei Gleichstand.
    pub fn sieger(&self) -> Option<Seite> {
        use std::cmp::Ordering;
        match self.stand_links.cmp(&self.stand_rechts) {
            Ordering::Greater => Some(Seite::Links),
            Ordering::Less => Some(Seite::Rechts),
            Ordering::Equal => None,
        }
    }
}

/// Ein Spieler würfelt dreimal, darf Würfel neu werfen und bekommt seine Punkte.
pub fn zug(spieler: &mut dyn Spieler, zufall: &mut Zufall) -> Zug {
    let mut wurf: Vec<u32> = (0..3).map(|_| zufall.wuerfel()).collect();
    let erster_wurf = wurf.clone();
    for index in spieler.neu_wuerfeln(&wurf) {
        if index < wurf.len() {
            wurf[index] = zufall.wuerfel();
        }
    }
    let punkte = wertung(&wurf);
    Zug {
        name: spieler.name().to_string(),
        erster_wurf,
        endwurf: wurf,
        punkte,
    }
}

/// Lässt zwei Spieler gegeneinander antreten.
pub fn duell(
    links: &mut dyn Spieler,
    rechts: &mut dyn Spieler,
    zufall: &mut Zufall,
    runden: u32,
) -> Ergebnis {
    let mut ergebnis = Ergebnis {
        runden: Vec::new(),
        stand_links: Punkte::default(),
        stand_rechts: Punkte::default(),
    };
    for _ in 0..runden {
        let zug_links = zug(links, zufall);
        let zug_rechts = zug(rechts, zufall);
        ergebnis.stand_links += zug_links.punkte;
        ergebnis.stand_rechts += zug_rechts.punkte;
        ergebnis.runden.push((zug_links, zug_rechts));
    }
    ergebnis
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Computer;

    #[test]
    fn so_viele_runden_wie_verlangt() {
        let mut a = Computer::new("A", 4);
        let mut b = Computer::new("B", 3);
        let mut zufall = Zufall::new(7);
        let ergebnis = duell(&mut a, &mut b, &mut zufall, 5);
        assert_eq!(ergebnis.runden.len(), 5);
    }

    #[test]
    fn endstand_ist_die_summe_der_zuege() {
        let mut a = Computer::new("A", 4);
        let mut b = Computer::new("B", 3);
        let mut zufall = Zufall::new(7);
        let ergebnis = duell(&mut a, &mut b, &mut zufall, 4);
        let summe: u32 =
            ergebnis.runden.iter().map(|(l, _)| l.punkte.0).sum();
        assert_eq!(ergebnis.stand_links, Punkte(summe));
    }

    #[test]
    fn gleichstand_hat_keinen_sieger() {
        let ergebnis = Ergebnis {
            runden: Vec::new(),
            stand_links: Punkte(10),
            stand_rechts: Punkte(10),
        };
        assert_eq!(ergebnis.sieger(), None);
    }

    #[test]
    fn mehr_punkte_gewinnen() {
        let ergebnis = Ergebnis {
            runden: Vec::new(),
            stand_links: Punkte(10),
            stand_rechts: Punkte(11),
        };
        assert_eq!(ergebnis.sieger(), Some(Seite::Rechts));
    }
}
