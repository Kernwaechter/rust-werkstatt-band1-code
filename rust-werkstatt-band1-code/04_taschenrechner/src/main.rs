use std::error::Error;
use std::fmt;
use std::io;
use std::num::ParseIntError;

#[derive(Debug)]
enum RechenFehler {
    LeereEingabe,
    UnbekanntesZeichen(String),
    ZahlUngueltig { text: String, grund: ParseIntError },
    ZuWenigWerte { operator: char, vorhanden: usize },
    ZuVieleWerte(usize),
    DivisionDurchNull,
    Ueberlauf(char),
}

impl fmt::Display for RechenFehler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RechenFehler::LeereEingabe => {
                write!(f, "es wurde nichts eingegeben")
            }
            RechenFehler::UnbekanntesZeichen(text) => {
                write!(
                    f,
                    "„{text}“ ist kein gültiges Rechenzeichen"
                )
            }
            RechenFehler::ZahlUngueltig { text, .. } => {
                write!(f, "„{text}“ ist keine gültige Zahl")
            }
            RechenFehler::ZuWenigWerte { operator, vorhanden } => {
                write!(f, "für „{operator}“ fehlen Zahlen ")?;
                write!(f, "(nur {vorhanden} vorhanden)")
            }
            RechenFehler::ZuVieleWerte(anzahl) => {
                write!(f, "am Ende bleiben {anzahl} Werte übrig, ")?;
                write!(f, "erwartet war einer")
            }
            RechenFehler::DivisionDurchNull => {
                write!(f, "durch null kann man nicht teilen")
            }
            RechenFehler::Ueberlauf(operator) => {
                write!(f, "das Ergebnis von „{operator}“ ist zu groß")
            }
        }
    }
}

impl Error for RechenFehler {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            RechenFehler::ZahlUngueltig { grund, .. } => Some(grund),
            _ => None,
        }
    }
}

fn ist_zahl(token: &str) -> bool {
    let ziffern = token.strip_prefix('-').unwrap_or(token);
    !ziffern.is_empty() && ziffern.chars().all(|c| c.is_ascii_digit())
}

fn rechne(eingabe: &str) -> Result<i64, RechenFehler> {
    let mut stapel: Vec<i64> = Vec::new();
    let mut gesehen = false;

    for token in eingabe.split_whitespace() {
        gesehen = true;

        if ist_zahl(token) {
            let zahl = token
                .parse::<i64>()
                .map_err(|grund| RechenFehler::ZahlUngueltig {
                    text: token.to_string(),
                    grund,
                })?;
            stapel.push(zahl);
            continue;
        }

        let operator = match token {
            "+" => '+',
            "-" => '-',
            "*" => '*',
            "/" => '/',
            _ => {
                return Err(RechenFehler::UnbekanntesZeichen(
                    token.to_string(),
                ))
            }
        };

        let rechts = stapel.pop();
        let links = stapel.pop();
        let (links, rechts) = match (links, rechts) {
            (Some(l), Some(r)) => (l, r),
            _ => {
                let vorhanden = usize::from(links.is_some())
                    + usize::from(rechts.is_some());
                return Err(RechenFehler::ZuWenigWerte {
                    operator,
                    vorhanden,
                });
            }
        };

        let ergebnis = match operator {
            '+' => links.checked_add(rechts),
            '-' => links.checked_sub(rechts),
            '*' => links.checked_mul(rechts),
            _ => {
                if rechts == 0 {
                    return Err(RechenFehler::DivisionDurchNull);
                }
                links.checked_div(rechts)
            }
        };
        stapel.push(
            ergebnis.ok_or(RechenFehler::Ueberlauf(operator))?,
        );
    }

    if !gesehen {
        return Err(RechenFehler::LeereEingabe);
    }
    match stapel.len() {
        1 => Ok(stapel[0]),
        anzahl => Err(RechenFehler::ZuVieleWerte(anzahl)),
    }
}

fn main() {
    println!(
        "=== Taschenrechner (umgekehrte polnische Notation) ==="
    );
    println!("Beispiel: 3 4 + 2 *  ergibt 14. Mit „ende“ beenden.");

    loop {
        let mut zeile = String::new();
        match io::stdin().read_line(&mut zeile) {
            Ok(0) | Err(_) => break,
            Ok(_) => {}
        }
        if zeile.trim() == "ende" {
            break;
        }

        match rechne(&zeile) {
            Ok(wert) => println!("= {wert}"),
            Err(fehler) => {
                println!("Fehler: {fehler}");
                if let Some(ursache) = fehler.source() {
                    println!("  Ursache: {ursache}");
                }
            }
        }
    }
}
