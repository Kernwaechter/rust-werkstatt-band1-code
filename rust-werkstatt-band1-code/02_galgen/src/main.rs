use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

const WOERTER: [&str; 8] = [
    "drache",
    "zauberstab",
    "ritterburg",
    "nebelwald",
    "feuerball",
    "märchenwald",
    "goldschatz",
    "räuberhöhle",
];

const MAX_FEHLER: usize = 7;

#[derive(PartialEq, Debug)]
enum Ergebnis {
    Treffer,
    Fehlschlag,
    SchonGeraten,
}

enum Eingabe {
    Buchstabe(char),
    Ungueltig,
    Ende,
}

struct Spiel {
    wort: Vec<char>,
    geraten: Vec<char>,
    fehler: usize,
}

impl Spiel {
    fn new(wort: &str) -> Self {
        Spiel {
            wort: wort.to_lowercase().chars().collect(),
            geraten: Vec::new(),
            fehler: 0,
        }
    }

    fn raten(&mut self, buchstabe: char) -> Ergebnis {
        if self.geraten.contains(&buchstabe) {
            return Ergebnis::SchonGeraten;
        }
        self.geraten.push(buchstabe);
        if self.wort.contains(&buchstabe) {
            Ergebnis::Treffer
        } else {
            self.fehler += 1;
            Ergebnis::Fehlschlag
        }
    }

    fn maske(&self) -> String {
        let mut text = String::new();
        for &b in &self.wort {
            if self.geraten.contains(&b) {
                text.push(b);
            } else {
                text.push('_');
            }
            text.push(' ');
        }
        text.trim_end().to_string()
    }

    fn gewonnen(&self) -> bool {
        self.wort.iter().all(|b| self.geraten.contains(b))
    }

    fn verloren(&self) -> bool {
        self.fehler >= MAX_FEHLER
    }

    fn galgen(&self) -> String {
        let f = self.fehler;
        let balken = if f >= 3 { "____" } else { "    " };
        let seil = if f >= 3 { "|" } else { " " };
        let pfosten = if f >= 2 { "|" } else { " " };
        let kopf = if f >= 4 { "O" } else { " " };
        let arm_l = if f >= 6 { "/" } else { " " };
        let koerper = if f >= 5 { "|" } else { " " };
        let arm_r = if f >= 6 { "\\" } else { " " };
        let bein_l = if f >= 7 { "/" } else { " " };
        let bein_r = if f >= 7 { "\\" } else { " " };
        let boden = if f >= 1 { "======" } else { "      " };
        let l1 = format!(" {pfosten}{balken}");
        let l2 = format!(" {pfosten}   {seil}");
        let l3 = format!(" {pfosten}   {kopf}");
        let l4 = format!(" {pfosten}  {arm_l}{koerper}{arm_r}");
        let l5 = format!(" {pfosten}  {bein_l} {bein_r}");
        format!("{l1}\n{l2}\n{l3}\n{l4}\n{l5}\n{boden}")
    }
}

fn zufallswort() -> String {
    let micros = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_micros())
        .unwrap_or(0);
    let index = usize::try_from(micros).unwrap_or(0) % WOERTER.len();
    WOERTER[index].to_string()
}

fn buchstabe_lesen() -> Eingabe {
    let mut zeile = String::new();
    match io::stdin().read_line(&mut zeile) {
        Ok(0) | Err(_) => return Eingabe::Ende,
        Ok(_) => {}
    }
    let mut zeichen = zeile.trim().chars();
    match (zeichen.next(), zeichen.next()) {
        (Some(b), None) if b.is_alphabetic() => {
            Eingabe::Buchstabe(b.to_lowercase().next().unwrap_or(b))
        }
        _ => Eingabe::Ungueltig,
    }
}

fn main() {
    let mut spiel = Spiel::new(&zufallswort());
    println!("=== Galgenmännchen ===");

    while !spiel.gewonnen() && !spiel.verloren() {
        println!("\n{}", spiel.galgen());
        println!("\nWort: {}", spiel.maske());
        println!("Fehler: {} von {}", spiel.fehler, MAX_FEHLER);
        println!("Dein Buchstabe:");

        match buchstabe_lesen() {
            Eingabe::Buchstabe(b) => match spiel.raten(b) {
                Ergebnis::Treffer => println!("Treffer!"),
                Ergebnis::Fehlschlag => println!("Leider nein."),
                Ergebnis::SchonGeraten => {
                    println!("Den hattest du schon.")
                }
            },
            Eingabe::Ungueltig => {
                println!("Bitte genau einen Buchstaben eingeben.")
            }
            Eingabe::Ende => {
                println!("Eingabe zu Ende, das Spiel wird beendet.");
                return;
            }
        }
    }

    println!("\n{}", spiel.galgen());
    let loesung: String = spiel.wort.iter().collect();
    if spiel.gewonnen() {
        println!("\nGewonnen! Das Wort war {loesung}.");
    } else {
        println!("\nVerloren. Das Wort war {loesung}.");
    }
}
