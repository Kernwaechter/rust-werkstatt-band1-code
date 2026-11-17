use std::sync::{mpsc, Arc, Mutex};
use std::thread;

const BLOCK: u32 = 10_000;
const ENDE: u32 = 200_000;
const ARBEITER: usize = 4;

fn ist_prim(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }
    let mut teiler = 3;
    while teiler * teiler <= n {
        if n % teiler == 0 {
            return false;
        }
        teiler += 2;
    }
    true
}

fn zaehle(von: u32, bis: u32) -> usize {
    (von..bis).filter(|n| ist_prim(*n)).count()
}

fn main() {
    let blocks: Vec<u32> =
        (0..ENDE / BLOCK).rev().map(|i| i * BLOCK).collect();
    let stapel = Arc::new(Mutex::new(blocks));
    let (tx, rx) = mpsc::channel();

    for _ in 0..ARBEITER {
        let stapel = Arc::clone(&stapel);
        let tx = tx.clone();
        thread::spawn(move || loop {
            let naechster = stapel.lock().unwrap().pop();
            let Some(start) = naechster else { break };
            let anzahl = zaehle(start, start + BLOCK);
            tx.send((start, anzahl)).unwrap();
        });
    }
    drop(tx);

    let mut ergebnisse: Vec<(u32, usize)> = rx.iter().collect();
    ergebnisse.sort();

    for (start, anzahl) in &ergebnisse {
        println!(
            "{:>6} bis {:>6}: {:>4}",
            start,
            start + BLOCK,
            anzahl
        );
    }
    let gesamt: usize =
        ergebnisse.iter().map(|(_, anzahl)| anzahl).sum();
    println!("Zusammen: {gesamt}");
}
