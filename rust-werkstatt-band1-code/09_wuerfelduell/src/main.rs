use std::io;

use wuerfelduell::{duell, Computer, Ergebnis, Seite, Spieler, Zufall};

struct Mensch {
    name: String,
}

impl Spieler for Mensch {
    fn name(&self) -> &str {
        &self.name
    }

    fn neu_wuerfeln(&mut self, wurf: &[u32]) -> Vec<usize> {
        println!(
            "Welche Würfel neu? Positionen 1 bis {}, leer lassen für keinen:",
            wurf.len()
        );
        let mut zeile = String::new();
        if io::stdin().read_line(&mut zeile).is_err() {
            return Vec::new();
        }
        zeile
            .split_whitespace()
            .filter_map(|text| text.parse::<usize>().ok())
            .filter(|&position| position >= 1 && position <= wurf.len())
            .map(|position| position - 1)
            .collect()
    }
}

fn zeige(wurf: &[u32]) -> String {
    let teile: Vec<String> =
        wurf.iter().map(|a| a.to_string()).collect();
    teile.join(" ")
}

fn ausgeben(ergebnis: &Ergebnis, links: &str, rechts: &str) {
    for (nummer, (zug_l, zug_r)) in ergebnis.runden.iter().enumerate() {
        println!("--- Runde {} ---", nummer + 1);
        for zug in [zug_l, zug_r] {
            println!(
                "{:<8} {}  ->  {}  = {:>5}",
                zug.name,
                zeige(&zug.erster_wurf),
                zeige(&zug.endwurf),
                zug.punkte
            );
        }
    }
    println!(
        "Endstand: {} {} : {} {}",
        links, ergebnis.stand_links, ergebnis.stand_rechts, rechts
    );
    match ergebnis.sieger() {
        Some(Seite::Links) => println!("{links} gewinnt."),
        Some(Seite::Rechts) => println!("{rechts} gewinnt."),
        None => println!("Unentschieden."),
    }
}

fn lies_zeile() -> String {
    let mut zeile = String::new();
    let _ = io::stdin().read_line(&mut zeile);
    zeile
}

fn main() {
    println!("=== Würfelduell ===");
    println!("Spielst du selbst? (j/n)");
    let selbst = lies_zeile().trim().to_lowercase() == "j";

    let mut zufall = Zufall::aus_uhr();
    let mut links: Box<dyn Spieler> = if selbst {
        Box::new(Mensch {
            name: String::from("Du"),
        })
    } else {
        Box::new(Computer::new("Aria", 4))
    };
    let mut rechts: Box<dyn Spieler> =
        Box::new(Computer::new("Gorn", 3));
    println!("{} gegen {}", links.vorstellung(), rechts.vorstellung());

    let ergebnis =
        duell(links.as_mut(), rechts.as_mut(), &mut zufall, 5);
    ausgeben(&ergebnis, links.name(), rechts.name());
}
