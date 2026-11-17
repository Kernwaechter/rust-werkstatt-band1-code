use std::fs;
use std::path::PathBuf;

use chrono::{TimeZone, Utc};
use projekt_putzer::{Fundort, ProjektPutzer, PutzBericht};

fn arbeitsordner(name: &str) -> PathBuf {
    let ordner = std::env::temp_dir().join(format!("projekt_putzer_test_{name}"));
    let _ = fs::remove_dir_all(&ordner);
    fs::create_dir_all(&ordner).unwrap();
    ordner
}

#[test]
fn historie_speichern_und_laden() {
    let ordner = arbeitsordner("historie");
    let pfad = ordner.join("historie.json");

    let historie = vec![PutzBericht {
        zeitpunkt: Utc.with_ymd_and_hms(2026, 10, 23, 10, 0, 0).unwrap(),
        freigegeben_bytes: 1024 * 1024 * 50,
        anzahl_ordner: 3,
    }];

    ProjektPutzer::historie_speichern(&historie, &pfad).unwrap();
    assert!(!pfad.with_extension("tmp").exists());

    let geladen = ProjektPutzer::historie_laden(&pfad).unwrap();
    assert_eq!(geladen, historie);
}

#[test]
fn scannen_findet_target_ordner() {
    let ordner = arbeitsordner("scan");
    let projekt1 = ordner.join("mein_projekt");
    let target = projekt1.join("target");
    fs::create_dir_all(&target).unwrap();
    fs::write(projekt1.join("Cargo.toml"), "[package]\nname = \"dummy\"\nversion = \"0.1.0\"\n").unwrap();
    fs::write(target.join("dummy.o"), "testdaten").unwrap();

    let funde = ProjektPutzer::scannen(&ordner, 0).unwrap();
    assert_eq!(funde.len(), 1);
    assert_eq!(funde[0].projekt_pfad, projekt1);
    assert_eq!(funde[0].target_pfad, target);
    assert!(funde[0].groesse_bytes > 0);
}

#[test]
fn bereinigen_loescht_target_ordner() {
    let ordner = arbeitsordner("loeschen");
    let target = ordner.join("target");
    fs::create_dir_all(&target).unwrap();
    fs::write(target.join("artefakt.bin"), "12345").unwrap();

    let fund = Fundort {
        projekt_pfad: ordner.clone(),
        target_pfad: target.clone(),
        groesse_bytes: 5,
        zuletzt_geaendert: Utc::now(),
    };

    let bericht = ProjektPutzer::bereinigen(&[fund], false).unwrap();
    assert_eq!(bericht.anzahl_ordner, 1);
    assert_eq!(bericht.freigegeben_bytes, 5);
    assert!(!target.exists());
}

#[test]
fn dry_run_loescht_nicht() {
    let ordner = arbeitsordner("dry_run");
    let target = ordner.join("target");
    fs::create_dir_all(&target).unwrap();
    fs::write(target.join("artefakt.bin"), "12345").unwrap();

    let fund = Fundort {
        projekt_pfad: ordner.clone(),
        target_pfad: target.clone(),
        groesse_bytes: 5,
        zuletzt_geaendert: Utc::now(),
    };

    let bericht = ProjektPutzer::bereinigen(&[fund], true).unwrap();
    assert_eq!(bericht.anzahl_ordner, 1);
    assert!(target.exists());
}
