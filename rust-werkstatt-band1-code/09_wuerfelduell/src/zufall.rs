use std::time::{SystemTime, UNIX_EPOCH};

/// Ein kleiner Zufallsgenerator (Xorshift). Mit demselben Startwert liefert er
/// immer dieselbe Folge, das macht ihn testbar.
///
/// # Examples
///
/// ```
/// use wuerfelduell::Zufall;
///
/// let mut zufall = Zufall::new(88172645463325252);
/// assert_eq!(zufall.wuerfel(), 3);
/// ```
pub struct Zufall(u64);

impl Zufall {
    /// Startet mit einem festen Wert. Die 0 ist nicht erlaubt und wird zur 1.
    pub fn new(start: u64) -> Self {
        Zufall(start.max(1))
    }

    /// Startet mit der aktuellen Uhrzeit.
    pub fn aus_uhr() -> Self {
        let micros = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_micros())
            .unwrap_or(1);
        Zufall::new(u64::try_from(micros).unwrap_or(1) | 1)
    }

    /// Würfelt eine Zahl von 1 bis 6.
    pub fn wuerfel(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        u32::try_from(self.0 % 6).unwrap_or(0) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feste_folge_beim_festen_start() {
        let mut zufall = Zufall::new(88172645463325252);
        let erste: Vec<u32> =
            (0..10).map(|_| zufall.wuerfel()).collect();
        assert_eq!(erste, [3, 2, 5, 4, 5, 6, 4, 6, 1, 6]);
    }

    #[test]
    fn alle_wuerfe_liegen_zwischen_eins_und_sechs() {
        let mut zufall = Zufall::new(42);
        for _ in 0..10_000 {
            let augen = zufall.wuerfel();
            assert!(
                (1..=6).contains(&augen),
                "unmöglicher Wurf: {augen}"
            );
        }
    }

    #[test]
    fn null_als_start_bleibt_nicht_haengen() {
        let mut zufall = Zufall::new(0);
        let a = zufall.wuerfel();
        let b = zufall.wuerfel();
        let c = zufall.wuerfel();
        assert!(a != b || b != c, "der Generator steht still");
    }
}
