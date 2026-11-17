use std::collections::HashMap;

const TEXT: &str = "\
Der Drache schlief tief im Berg. \
Der Held schlich leise heran, \
denn der Drache hatte ein feines Gehör. \
Als der Drache erwachte, floh der Held \
in den Wald, und der Wald schwieg.";

fn woerter(text: &str) -> Vec<&str> {
    text.split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()))
        .filter(|w| !w.is_empty())
        .collect()
}

struct Statistik<'a> {
    woerter: Vec<&'a str>,
}

impl<'a> Statistik<'a> {
    fn new(text: &'a str) -> Self {
        Statistik {
            woerter: woerter(text),
        }
    }

    fn anzahl(&self) -> usize {
        self.woerter.len()
    }

    fn laengstes(&self) -> Option<&'a str> {
        self.woerter
            .iter()
            .copied()
            .max_by_key(|w| w.chars().count())
    }

    fn haeufigste(&self, n: usize) -> Vec<(String, u32)> {
        let mut summen: HashMap<String, u32> = HashMap::new();
        for w in &self.woerter {
            *summen.entry(w.to_lowercase()).or_insert(0) += 1;
        }
        let mut paare: Vec<(String, u32)> =
            summen.into_iter().collect();
        paare.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        paare.truncate(n);
        paare
    }
}

fn main() {
    let text = String::from(TEXT);
    let statistik = Statistik::new(&text);

    println!("{} Wörter", statistik.anzahl());
    if let Some(wort) = statistik.laengstes() {
        println!(
            "Längstes Wort: {wort} ({} Buchstaben)",
            wort.chars().count()
        );
    }
    println!("Häufigste Wörter:");
    for (wort, anzahl) in statistik.haeufigste(5) {
        println!("{anzahl:>3}  {wort}");
    }
}
