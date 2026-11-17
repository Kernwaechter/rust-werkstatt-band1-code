/// Was ein Spieler können muss, damit das Duell mit ihm funktioniert.
pub trait Spieler {
    /// Der Name für die Ausgabe.
    fn name(&self) -> &str;

    /// Gibt die Positionen (ab 0) der Würfel zurück, die neu geworfen werden sollen.
    fn neu_wuerfeln(&mut self, wurf: &[u32]) -> Vec<usize>;

    /// Ein Satz zur Vorstellung. Wer will, überschreibt ihn.
    fn vorstellung(&self) -> String {
        format!("Spieler {}", self.name())
    }
}

/// Ein Computerspieler: Er wirft alle Würfel neu, die unter seiner Schwelle liegen.
pub struct Computer {
    name: String,
    schwelle: u32,
}

impl Computer {
    /// Ein neuer Computerspieler. Bei Schwelle 4 werden 1, 2 und 3 neu geworfen.
    pub fn new(name: &str, schwelle: u32) -> Self {
        Computer {
            name: name.to_string(),
            schwelle,
        }
    }
}

impl Spieler for Computer {
    fn name(&self) -> &str {
        &self.name
    }

    fn neu_wuerfeln(&mut self, wurf: &[u32]) -> Vec<usize> {
        wurf.iter()
            .enumerate()
            .filter(|(_, &augen)| augen < self.schwelle)
            .map(|(index, _)| index)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutiger_computer_wirft_kleine_neu() {
        let mut aria = Computer::new("Aria", 4);
        assert_eq!(aria.neu_wuerfeln(&[3, 2, 5]), vec![0, 1]);
    }

    #[test]
    fn vorsichtiger_computer_wirft_nur_einsen_neu() {
        let mut gorn = Computer::new("Gorn", 2);
        assert_eq!(gorn.neu_wuerfeln(&[1, 6, 1]), vec![0, 2]);
    }

    #[test]
    fn hoher_wurf_bleibt_stehen() {
        let mut aria = Computer::new("Aria", 4);
        assert!(aria.neu_wuerfeln(&[6, 5, 4]).is_empty());
    }

    #[test]
    fn vorstellung_nennt_den_namen() {
        let aria = Computer::new("Aria", 4);
        assert_eq!(aria.vorstellung(), "Spieler Aria");
    }
}
