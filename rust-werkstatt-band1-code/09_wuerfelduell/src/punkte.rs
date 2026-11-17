use std::fmt;
use std::ops::{Add, AddAssign};

/// Punkte eines Spielers. Ein eigener Typ, damit sie nicht mit anderen Zahlen
/// verwechselt werden.
///
/// # Examples
///
/// ```
/// use wuerfelduell::Punkte;
///
/// let mut stand = Punkte::default();
/// stand += Punkte(14);
/// stand += Punkte(16);
/// assert_eq!(stand, Punkte(30));
/// assert_eq!(stand.to_string(), "30 P");
/// ```
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default,
)]
pub struct Punkte(pub u32);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addieren() {
        assert_eq!(Punkte(3) + Punkte(4), Punkte(7));
    }

    #[test]
    fn standardwert_ist_null() {
        assert_eq!(Punkte::default(), Punkte(0));
    }

    #[test]
    fn anzeige_beachtet_die_breite() {
        assert_eq!(format!("[{:>6}]", Punkte(5)), "[   5 P]");
        assert_eq!(format!("[{:<6}]", Punkte(5)), "[5 P   ]");
    }

    #[test]
    fn vergleich() {
        assert!(Punkte(10) < Punkte(11));
    }
}
