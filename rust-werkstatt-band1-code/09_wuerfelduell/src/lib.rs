//! Würfelduell: die Regeln und die Spieler, getrennt von Tastatur und Bildschirm.
//!
//! Zwei Spieler würfeln pro Runde je drei Würfel und dürfen einmal neu werfen.
//! Die Bibliothek kennt weder `stdin` noch `println!`. Dadurch lässt sie sich
//! vollständig testen.

pub mod duell;
pub mod punkte;
pub mod spieler;
pub mod wertung;
pub mod zufall;

pub use duell::{duell, Ergebnis, Seite, Zug};
pub use punkte::Punkte;
pub use spieler::{Computer, Spieler};
pub use zufall::Zufall;
