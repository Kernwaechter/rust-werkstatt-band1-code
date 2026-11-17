use crate::Punkte;

/// Zählt die Punkte eines Wurfs: die Summe der Augen, bei drei gleichen
/// Würfeln (Pasch) zehn Punkte dazu.
///
/// # Examples
///
/// ```
/// use wuerfelduell::{wertung::wertung, Punkte};
///
/// assert_eq!(wertung(&[2, 3, 5]), Punkte(10));
/// assert_eq!(wertung(&[4, 4, 4]), Punkte(22));
/// ```
pub fn wertung(wurf: &[u32]) -> Punkte {
    let summe: u32 = wurf.iter().sum();
    let pasch = wurf.len() == 3 && wurf.iter().all(|&a| a == wurf[0]);
    if pasch {
        Punkte(summe) + Punkte(10)
    } else {
        Punkte(summe)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summe_ohne_pasch() {
        assert_eq!(wertung(&[1, 2, 6]), Punkte(9));
    }

    #[test]
    fn pasch_bringt_zehn_extra() {
        assert_eq!(wertung(&[6, 6, 6]), Punkte(28));
        assert_eq!(wertung(&[1, 1, 1]), Punkte(13));
    }

    #[test]
    fn zwei_gleiche_sind_kein_pasch() {
        assert_eq!(wertung(&[5, 5, 2]), Punkte(12));
    }

    #[test]
    fn leerer_wurf_ist_null() {
        assert_eq!(wertung(&[]), Punkte(0));
    }

    #[test]
    fn zwei_gleiche_wuerfel_allein_sind_kein_pasch() {
        assert_eq!(wertung(&[3, 3]), Punkte(6));
    }
}
