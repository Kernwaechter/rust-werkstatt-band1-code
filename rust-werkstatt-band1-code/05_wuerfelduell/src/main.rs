use std::fmt;
use std::io;
use std::ops::{Add, AddAssign};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
struct Punkte(u32);

impl Add for Punkte {
    type Output = Punkte;

    fn add(self, andere: Punkte) -> Punkte {
        Punkte(self.0 + andere.0)
    }
}

impl AddAssign for Punkte {
    fn add_assign(&mut self, andere: Punkte) {
        self.0 += andere.0;
    }
}

impl fmt::Display for Punkte {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(&format!("{} P", self.0))
    }
}

struct Zufall(u64);

impl Zufall {
    fn aus_uhr() -> Self {
        let micros = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_micros())
            .unwrap_or(1);
        Zufall(u64::try_from(micros).unwrap_or(1) | 1)
    }

    fn wuerfel(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        u32::try_from(self.0 % 6).unwrap_or(0) + 1
    }
}

fn wertung(wurf: &[u32]) -> Punkte {
    let summe: u32 = wurf.iter().sum();
    let pasch = wurf.len() == 3 && wurf.iter().all(|&a| a == wurf[0]);
    if pasch {
        Punkte(summe) + Punkte(10)
    } else {
        Punkte(summe)
    }
}

trait Spieler {
    fn name(&self) -> &str;
    fn neu_wuerfeln(&mut self, wurf: &[u32]) -> Vec<usize>;

    fn vorstellung(&self) -> String {
        format!("Spieler {}", self.name())
    }
}

struct Computer {
    name: String,
    schwelle: u32,
}

impl Spieler for Computer {
    fn name(&self) -> &str {
        &self.name
    }

    fn neu_wuerfeln(&mut self, wurf: &[u32]) -> Vec<usize> {
        wurf.iter()
            .enumerate()
            .filter(|&(_, &augen)| augen < self.schwelle)
            .map(|(index, _)| index)
            .collect()
    }
}

struct Mensch {
    name: String,
}

impl Spieler for Mensch {
    fn name(&self) -> &str {
        &self.name
    }

    fn neu_wuerfeln(&mut self, wurf: &[u32]) -> Vec<usize> {
        println!(
            "Welche Würfel neu? (1 bis {}, Enter für keine):",
            wurf.len()
        );
        let mut zeile = String::new();
        if io::stdin().read_line(&mut zeile).is_err() {
            return Vec::new();
        }
        zeile
            .split_whitespace()
            .filter_map(|text| text.parse::<usize>().ok())
            .filter(|&pos| pos >= 1 && pos <= wurf.len())
            .map(|position| position - 1)
            .collect()
    }
}

fn zeige(wurf: &[u32]) -> String {
    let teile: Vec<String> =
        wurf.iter().map(|a| a.to_string()).collect();
    teile.join(" ")
}

fn zug(spieler: &mut dyn Spieler, zufall: &mut Zufall) -> Punkte {
    let mut wurf: Vec<u32> =
        (0..3).map(|_| zufall.wuerfel()).collect();
    let erster = zeige(&wurf);
    for index in spieler.neu_wuerfeln(&wurf) {
        if index < wurf.len() {
            wurf[index] = zufall.wuerfel();
        }
    }
    let punkte = wertung(&wurf);
    println!(
        "{:<8} {}  ->  {}  = {:>5}",
        spieler.name(),
        erster,
        zeige(&wurf),
        punkte
    );
    punkte
}

fn duell(
    a: &mut dyn Spieler,
    b: &mut dyn Spieler,
    zufall: &mut Zufall,
    runden: u32,
) {
    println!("{} gegen {}", a.vorstellung(), b.vorstellung());
    let mut stand_a = Punkte::default();
    let mut stand_b = Punkte::default();

    for runde in 1..=runden {
        println!("--- Runde {runde} ---");
        let pa = zug(a, zufall);
        let pb = zug(b, zufall);
        stand_a += pa;
        stand_b += pb;
    }

    println!(
        "Endstand: {} {} : {} {}",
        a.name(),
        stand_a,
        stand_b,
        b.name()
    );
    if stand_a > stand_b {
        println!("{} gewinnt.", a.name());
    } else if stand_b > stand_a {
        println!("{} gewinnt.", b.name());
    } else {
        println!("Unentschieden.");
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
        Box::new(Computer {
            name: String::from("Aria"),
            schwelle: 4,
        })
    };
    let mut rechts: Box<dyn Spieler> = Box::new(Computer {
        name: String::from("Gorn"),
        schwelle: 3,
    });

    duell(links.as_mut(), rechts.as_mut(), &mut zufall, 5);
}
