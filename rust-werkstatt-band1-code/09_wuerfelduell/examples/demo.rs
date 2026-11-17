use wuerfelduell::{duell, Computer, Seite, Spieler, Zufall};

fn main() {
    let mut zufall = Zufall::new(88172645463325252);
    let mut aria = Computer::new("Aria", 4);
    let mut gorn = Computer::new("Gorn", 2);

    println!("{} gegen {}", aria.vorstellung(), gorn.vorstellung());
    let ergebnis = duell(&mut aria, &mut gorn, &mut zufall, 3);

    println!(
        "Endstand: {} {} : {} {}",
        aria.name(),
        ergebnis.stand_links,
        ergebnis.stand_rechts,
        gorn.name()
    );
    match ergebnis.sieger() {
        Some(Seite::Links) => println!("{} gewinnt.", aria.name()),
        Some(Seite::Rechts) => println!("{} gewinnt.", gorn.name()),
        None => println!("Unentschieden."),
    }
}
