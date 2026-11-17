//! Der Projekt-Putzer: Ein echtes Werkstatt-Werkzeug für Entwickler.
//! Es durchsucht Verzeichnisse nach veralteten `target/`-Ordnern,
//! berechnet die verschwendete Festplattenkapazität und bereinigt sie sicher.

use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Ein gefundener Build-Ordner mit Größen- und Altersangaben.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Fundort {
    /// Pfad zum Projektordner (nicht nur target/)
    pub projekt_pfad: PathBuf,
    /// Pfad zum konkreten target-Ordner
    pub target_pfad: PathBuf,
    /// Belegter Speicherplatz in Bytes
    pub groesse_bytes: u64,
    /// Zeitpunkt der letzten Änderung eines Build-Artefakts
    pub zuletzt_geaendert: DateTime<Utc>,
}

impl Fundort {
    /// Formatiert die Bytezahl in eine menschenlesbare Größe (z. B. "1.42 GB" oder "350 MB").
    pub fn lesbare_groesse(&self) -> String {
        format_groesse(self.groesse_bytes)
    }

    /// Berechnet das Alter des Ordners in Tagen bezogen auf `jetzt`.
    pub fn alter_in_tagen(&self, jetzt: DateTime<Utc>) -> i64 {
        let dauer = jetzt.signed_duration_since(self.zuletzt_geaendert);
        dauer.num_days()
    }
}

/// Formatiert Bytes in KB, MB oder GB.
pub fn format_groesse(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    let b = bytes as f64;
    if b >= GB {
        format!("{:.2} GB", b / GB)
    } else if b >= MB {
        format!("{:.1} MB", b / MB)
    } else if b >= KB {
        format!("{:.0} KB", b / KB)
    } else {
        format!("{bytes} B")
    }
}

/// Ein Bericht über einen durchgeführten Reinigungslauf.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PutzBericht {
    /// Wann der Putzer lief
    pub zeitpunkt: DateTime<Utc>,
    /// Wie viele Bytes insgesamt freigegeben wurden
    pub freigegeben_bytes: u64,
    /// Wie viele Ordner bereinigt wurden
    pub anzahl_ordner: usize,
}

/// Das Kern-Werkzeug zum Scannen und Bereinigen.
pub struct ProjektPutzer;

impl ProjektPutzer {
    /// Durchsucht `wurzel` rekursiv nach Rust-Projekten mit `target/`-Ordner,
    /// die älter als `mindest_alter_tage` sind.
    pub fn scannen(wurzel: &Path, mindest_alter_tage: u32) -> Result<Vec<Fundort>, PutzerFehler> {
        if !wurzel.exists() {
            return Err(PutzerFehler::PfadExistiertNicht(wurzel.to_path_buf()));
        }
        let mut fundorte = Vec::new();
        let jetzt = Utc::now();
        Self::scan_intern(wurzel, &mut fundorte)?;

        // Nach Alter filtern
        fundorte.retain(|f| f.alter_in_tagen(jetzt) >= mindest_alter_tage as i64);

        // Nach Größe absteigend sortieren (die dicksten Brocken zuerst)
        fundorte.sort_by(|a, b| b.groesse_bytes.cmp(&a.groesse_bytes));

        Ok(fundorte)
    }

    fn scan_intern(pfad: &Path, sammlung: &mut Vec<Fundort>) -> Result<(), PutzerFehler> {
        let eintraege = fs::read_dir(pfad).map_err(|grund| PutzerFehler::Lesen {
            pfad: pfad.to_path_buf(),
            grund,
        })?;

        let mut hat_cargo_toml = false;
        let mut target_pfad: Option<PathBuf> = None;
        let mut unterordner = Vec::new();

        for eintrag in eintraege {
            let eintrag = eintrag.map_err(|grund| PutzerFehler::Lesen {
                pfad: pfad.to_path_buf(),
                grund,
            })?;
            let dateiname = eintrag.file_name();
            let dateityp = eintrag.file_type().map_err(|grund| PutzerFehler::Lesen {
                pfad: eintrag.path(),
                grund,
            })?;

            if dateityp.is_file() && dateiname == "Cargo.toml" {
                hat_cargo_toml = true;
            } else if dateityp.is_dir() {
                if dateiname == "target" {
                    target_pfad = Some(eintrag.path());
                } else if dateiname != ".git" && dateiname != "node_modules" {
                    unterordner.push(eintrag.path());
                }
            }
        }

        if hat_cargo_toml {
            if let Some(target) = target_pfad {
                let (bytes, modified) = berechne_ordner_details(&target)?;
                sammlung.push(Fundort {
                    projekt_pfad: pfad.to_path_buf(),
                    target_pfad: target,
                    groesse_bytes: bytes,
                    zuletzt_geaendert: modified,
                });
            }
        }

        // Rekursiv in Unterordner weitersuchen (falls Monorepo oder Workspace)
        for sub in unterordner {
            let _ = Self::scan_intern(&sub, sammlung);
        }

        Ok(())
    }

    /// Bereinigt die übergebenen Fundorte. Ist `dry_run` aktiv, wird nichts gelöscht.
    pub fn bereinigen(fundorte: &[Fundort], dry_run: bool) -> Result<PutzBericht, PutzerFehler> {
        let mut freigegeben: u64 = 0;
        let mut geloescht: usize = 0;

        for fund in fundorte {
            if !dry_run {
                log::info!("Lösche {}", fund.target_pfad.display());
                fs::remove_dir_all(&fund.target_pfad).map_err(|grund| PutzerFehler::Loeschen {
                    pfad: fund.target_pfad.clone(),
                    grund,
                })?;
            } else {
                log::info!("[Trockenlauf] Würde löschen: {}", fund.target_pfad.display());
            }
            freigegeben += fund.groesse_bytes;
            geloescht += 1;
        }

        Ok(PutzBericht {
            zeitpunkt: Utc::now(),
            freigegeben_bytes: freigegeben,
            anzahl_ordner: geloescht,
        })
    }

    /// Lädt die Reinigungshistorie aus einer JSON-Datei.
    pub fn historie_laden(pfad: &Path) -> Result<Vec<PutzBericht>, PutzerFehler> {
        match fs::read_to_string(pfad) {
            Ok(text) => {
                log::debug!("Lade Historie aus {}", pfad.display());
                serde_json::from_str(&text).map_err(|grund| PutzerFehler::Format {
                    pfad: pfad.to_path_buf(),
                    grund,
                })
            }
            Err(f) if f.kind() == io::ErrorKind::NotFound => {
                log::info!("Keine Historie unter {} gefunden – beginne neu.", pfad.display());
                Ok(Vec::new())
            }
            Err(grund) => Err(PutzerFehler::Lesen {
                pfad: pfad.to_path_buf(),
                grund,
            }),
        }
    }

    /// Speichert die Historie atomar (erst Temp-Datei, dann Rename).
    pub fn historie_speichern(historie: &[PutzBericht], pfad: &Path) -> Result<(), PutzerFehler> {
        let text = serde_json::to_string_pretty(historie).map_err(|grund| PutzerFehler::Format {
            pfad: pfad.to_path_buf(),
            grund,
        })?;

        if let Some(eltern) = pfad.parent() {
            fs::create_dir_all(eltern).map_err(|grund| PutzerFehler::Schreiben {
                pfad: eltern.to_path_buf(),
                grund,
            })?;
        }

        let temp_pfad = pfad.with_extension("tmp");
        fs::write(&temp_pfad, text).map_err(|grund| PutzerFehler::Schreiben {
            pfad: temp_pfad.clone(),
            grund,
        })?;

        fs::rename(&temp_pfad, pfad).map_err(|grund| PutzerFehler::Schreiben {
            pfad: pfad.to_path_buf(),
            grund,
        })?;

        log::info!("Historie erfolgreich nach {} geschrieben.", pfad.display());
        Ok(())
    }
}

/// Berechnet rekursiv die Ordnergröße und ermittelt den jüngsten Zeitstempel.
fn berechne_ordner_details(ordner: &Path) -> Result<(u64, DateTime<Utc>), PutzerFehler> {
    let mut gesamt_bytes = 0;
    let mut neuster_zeitstempel = SystemTime::UNIX_EPOCH;

    let mut stack = vec![ordner.to_path_buf()];

    while let Some(aktueller_pfad) = stack.pop() {
        if let Ok(eintraege) = fs::read_dir(&aktueller_pfad) {
            for eintrag in eintraege.flatten() {
                if let Ok(meta) = eintrag.metadata() {
                    if meta.is_dir() {
                        stack.push(eintrag.path());
                    } else if meta.is_file() {
                        gesamt_bytes += meta.len();
                        if let Ok(mod_time) = meta.modified() {
                            if mod_time > neuster_zeitstempel {
                                neuster_zeitstempel = mod_time;
                            }
                        }
                    }
                }
            }
        }
    }

    let datetime: DateTime<Utc> = neuster_zeitstempel.into();
    Ok((gesamt_bytes, datetime))
}

/// Fehlertypen des Putzers mit klaren Pfad-Angaben.
#[derive(Debug)]
pub enum PutzerFehler {
    PfadExistiertNicht(PathBuf),
    Lesen { pfad: PathBuf, grund: io::Error },
    Schreiben { pfad: PathBuf, grund: io::Error },
    Loeschen { pfad: PathBuf, grund: io::Error },
    Format { pfad: PathBuf, grund: serde_json::Error },
}

impl fmt::Display for PutzerFehler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PfadExistiertNicht(p) => write!(f, "Der Pfad „{}“ existiert nicht.", p.display()),
            Self::Lesen { pfad, grund } => write!(f, "Fehler beim Lesen von „{}“: {grund}", pfad.display()),
            Self::Schreiben { pfad, grund } => write!(f, "Fehler beim Schreiben von „{}“: {grund}", pfad.display()),
            Self::Loeschen { pfad, grund } => write!(f, "Konnte Ordner „{}“ nicht löschen: {grund}", pfad.display()),
            Self::Format { pfad, grund } => write!(f, "Ungültiges JSON-Format in „{}“: {grund}", pfad.display()),
        }
    }
}

impl Error for PutzerFehler {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Lesen { grund, .. } | Self::Schreiben { grund, .. } | Self::Loeschen { grund, .. } => Some(grund),
            Self::Format { grund, .. } => Some(grund),
            Self::PfadExistiertNicht(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formatierung_von_byte_groessen() {
        assert_eq!(format_groesse(500), "500 B");
        assert_eq!(format_groesse(2048), "2 KB");
        assert_eq!(format_groesse(5 * 1024 * 1024), "5.0 MB");
        assert_eq!(format_groesse((1.5 * 1024.0 * 1024.0 * 1024.0) as u64), "1.50 GB");
    }

    #[test]
    fn fehler_enthaelt_dateipfad() {
        let fehler = PutzerFehler::PfadExistiertNicht(PathBuf::from("/ungueltiger/pfad"));
        assert!(fehler.to_string().contains("/ungueltiger/pfad"));
    }
}
