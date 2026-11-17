use std::cmp::Ordering;
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_VERSUCHE: u32 = 7;

fn geheime_zahl() -> u32 {
    let micros = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_micros())
        .unwrap_or(42);
    micros % 100 + 1
}

fn main() {
    println!("=== Zahlenraten ===");
    println!("Ich denke mir eine Zahl von 1 bis 100.");
    println!("Du hast {MAX_VERSUCHE} Versuche.");

    let geheim = geheime_zahl();
    let mut versuche = 0;

    loop {
        if versuche == MAX_VERSUCHE {
            println!("Leider verloren. Die Zahl war {geheim}.");
            break;
        }
        println!("\nVersuch {}: Deine Zahl?", versuche + 1);
        let mut eingabe = String::new();
        let gelesen = io::stdin()
            .read_line(&mut eingabe)
            .expect("Eingabe konnte nicht gelesen werden");
        if gelesen == 0 {
            println!("Keine Eingabe mehr. Bis zum nächsten Mal!");
            break;
        }
        let tipp: u32 = match eingabe.trim().parse() {
            Ok(zahl) => zahl,
            Err(_) => {
                println!("Das ist keine Zahl. Versuch es nochmal.");
                continue;
            }
        };
        versuche += 1;
        match tipp.cmp(&geheim) {
            Ordering::Less => println!("Zu klein."),
            Ordering::Greater => println!("Zu groß."),
            Ordering::Equal => {
                println!("Richtig! Geschafft in {versuche} Zügen.");
                break;
            }
        }
    }
}
