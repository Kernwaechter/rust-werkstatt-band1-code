use std::collections::BTreeMap;

struct Zufall(u64);

impl Iterator for Zufall {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        Some(u32::try_from(self.0 % 6).unwrap_or(0) + 1)
    }
}

fn summe(wurf: &[u32]) -> u32 {
    wurf.iter().sum()
}

fn ist_pasch(wurf: &[u32]) -> bool {
    wurf.windows(2).all(|paar| paar[0] == paar[1])
}

fn main() {
    let augen: Vec<u32> =
        Zufall(88172645463325252).take(30_000).collect();
    let wuerfe: Vec<&[u32]> = augen.chunks(3).collect();
    println!("{} Würfe mit je drei Würfeln", wuerfe.len());

    let summen: Vec<u32> = wuerfe.iter().map(|w| summe(w)).collect();

    let (gesamt, anzahl) = summen
        .iter()
        .fold((0u32, 0u32), |(g, n), s| (g + s, n + 1));
    println!(
        "Durchschnitt: {:.2}",
        f64::from(gesamt) / f64::from(anzahl)
    );

    println!("Höchste Summe: {:?}", summen.iter().max());
    println!(
        "Erste 18: bei Wurf {:?}",
        summen.iter().position(|s| *s == 18)
    );

    let paesche = wuerfe.iter().filter(|w| ist_pasch(w)).count();
    println!("Päsche: {paesche}");

    let (hoch, niedrig): (Vec<u32>, Vec<u32>) =
        summen.iter().partition(|s| **s >= 11);
    println!(
        "Summe ab 11: {}, darunter: {}",
        hoch.len(),
        niedrig.len()
    );

    let verteilung = summen
        .iter()
        .fold(BTreeMap::new(), |mut karte, s| {
            *karte.entry(*s).or_insert(0u32) += 1;
            karte
        });

    println!("\nVerteilung der Summen (ein # = 25 Würfe):");
    for (summe, haeufigkeit) in &verteilung {
        let n = usize::try_from(*haeufigkeit / 25).unwrap_or(0);
        let balken = "#".repeat(n);
        println!("{summe:>2} {balken:<52} {haeufigkeit}");
    }

    let top = verteilung.iter().max_by_key(|(_, h)| **h);
    if let Some((summe, haeufigkeit)) = top {
        println!(
            "\nAm häufigsten: Summe {summe} ({haeufigkeit}-mal)"
        );
    }
}
