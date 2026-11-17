use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Person {
    name: String,
    eltern: RefCell<Weak<Person>>,
    kinder: RefCell<Vec<Rc<Person>>>,
}

impl Person {
    fn neu(name: &str) -> Rc<Person> {
        Rc::new(Person {
            name: name.to_string(),
            eltern: RefCell::new(Weak::new()),
            kinder: RefCell::new(Vec::new()),
        })
    }

    fn kind_hinzufuegen(eltern: &Rc<Person>, kind: Rc<Person>) {
        *kind.eltern.borrow_mut() = Rc::downgrade(eltern);
        eltern.kinder.borrow_mut().push(kind);
    }

    fn ahnen(&self) -> Vec<String> {
        let mut liste = Vec::new();
        let mut aktuell = self.eltern.borrow().upgrade();
        while let Some(person) = aktuell {
            liste.push(person.name.clone());
            aktuell = person.eltern.borrow().upgrade();
        }
        liste
    }

    fn nachkommen(&self) -> usize {
        self.kinder
            .borrow()
            .iter()
            .map(|kind| 1 + kind.nachkommen())
            .sum()
    }

    fn zeige(&self, tiefe: usize) {
        println!("{}{}", "  ".repeat(tiefe), self.name);
        for kind in self.kinder.borrow().iter() {
            kind.zeige(tiefe + 1);
        }
    }
}

impl Drop for Person {
    fn drop(&mut self) {
        println!("{} wird vergessen", self.name);
    }
}

fn main() {
    let brom = Person::neu("Brom");
    let aria = Person::neu("Aria");
    let gorn = Person::neu("Gorn");
    let mira = Person::neu("Mira");
    let tarn = Person::neu("Tarn");

    Person::kind_hinzufuegen(&aria, Rc::clone(&mira));
    Person::kind_hinzufuegen(&aria, Rc::clone(&tarn));
    Person::kind_hinzufuegen(&brom, Rc::clone(&aria));
    Person::kind_hinzufuegen(&brom, Rc::clone(&gorn));

    brom.zeige(0);
    println!("Vorfahren von Tarn: {:?}", tarn.ahnen());
    println!("Nachkommen von Brom: {}", brom.nachkommen());
    println!(
        "Aria: {} Besitzer, {} Rückverweise",
        Rc::strong_count(&aria),
        Rc::weak_count(&aria)
    );
    println!(
        "Brom: {} Besitzer, {} Rückverweise",
        Rc::strong_count(&brom),
        Rc::weak_count(&brom)
    );

    println!("\nBrom wird vergessen:");
    drop(brom);
    println!("Vorfahren von Aria: {:?}", aria.ahnen());
    println!("Aria hat noch {} Kinder", aria.kinder.borrow().len());
    println!("Ende von main");
}
