use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use projekt_putzer::{format_groesse, ProjektPutzer, PutzerFehler};

/// Der Projekt-Putzer: Befreit die Festplatte von alten target/-Ordnern.
#[derive(Parser, Debug)]
#[command(name = "projekt-putzer", version, about = "Sucht und bereinigt alte Rust-Build-Ordner")]
struct Cli {
    /// Pfad zur Historie-Datei
    #[arg(long, default_value = "putzer_historie.json")]
    historie: PathBuf,

    #[command(subcommand)]
    befehl: Befehl,
}

#[derive(Subcommand, Debug)]
enum Befehl {
    /// Sucht nach veralteten target/-Ordnern ohne zu löschen
    Scannen {
        /// Startverzeichnis für die Suche
        #[arg(short, long, default_value = ".")]
        pfad: PathBuf,

        /// Mindestalter in Tagen seit dem letzten Build
        #[arg(short, long, default_value_t = 14)]
        tage: u32,
    },

    /// Bereinigt veraltete target/-Ordner
    Putzen {
        /// Startverzeichnis für die Bereinigung
        #[arg(short, long, default_value = ".")]
        pfad: PathBuf,

        /// Mindestalter in Tagen
        #[arg(short, long, default_value_t = 14)]
        tage: u32,

        /// Nur simulieren, keine Ordner tatsächlich löschen
        #[arg(long)]
        dry_run: bool,
    },

    /// Zeigt bisherige Reinigungsläufe aus der Historie
    Historie,
}

fn main() -> ExitCode {
    env_logger::init();
    let cli = Cli::parse();

    match ausfuehren(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(fehler) => {
            eprintln!("Fehler: {fehler}");
            ExitCode::FAILURE
        }
    }
}

fn ausfuehren(cli: Cli) -> Result<(), PutzerFehler> {
    match cli.befehl {
        Befehl::Scannen { pfad, tage } => {
            println!("🔍 Scanne „{}“ nach Projekten älter als {} Tagen...", pfad.display(), tage);
            let funde = ProjektPutzer::scannen(&pfad, tage)?;

            if funde.is_empty() {
                println!("✨ Keine veralteten target/-Ordner gefunden. Alles sauber!");
                return Ok(());
            }

            println!("\nGefundene Build-Ordner:");
            let mut summe: u64 = 0;
            for fund in &funde {
                summe += fund.groesse_bytes;
                println!(
                    "  • {:<10} | {:>3} Tage alt | {}",
                    fund.lesbare_groesse(),
                    fund.alter_in_tagen(chrono::Utc::now()),
                    fund.target_pfad.display()
                );
            }
            println!("─────────────────────────────────────────");
            println!("Gesamt verschwendet: {}", format_groesse(summe));
            println!("Tipp: Mit `projekt-putzer putzen --tage {tage}` freigeben.");
            Ok(())
        }

        Befehl::Putzen { pfad, tage, dry_run } => {
            let funde = ProjektPutzer::scannen(&pfad, tage)?;
            if funde.is_empty() {
                println!("✨ Nichts zu bereinigen.");
                return Ok(());
            }

            let modus = if dry_run { " (SIMULATION)" } else { "" };
            println!("🧹 Bereinige {} gefundene Ordner{}...", funde.len(), modus);

            let bericht = ProjektPutzer::bereinigen(&funde, dry_run)?;
            println!("Erfolg: {} Ordner bereinigt, {} freigegeben.", bericht.anzahl_ordner, format_groesse(bericht.freigegeben_bytes));

            if !dry_run {
                let mut historie = ProjektPutzer::historie_laden(&cli.historie).unwrap_or_default();
                historie.push(bericht);
                ProjektPutzer::historie_speichern(&historie, &cli.historie)?;
            }
            Ok(())
        }

        Befehl::Historie => {
            let historie = ProjektPutzer::historie_laden(&cli.historie)?;
            if historie.is_empty() {
                println!("Noch keine Reinigungsläufe verzeichnet.");
                return Ok(());
            }

            println!("Vergangene Reinigungsläufe:");
            for (idx, e) in historie.iter().enumerate() {
                println!(
                    "  {}. {} | {} Ordner | {} freigegeben",
                    idx + 1,
                    e.zeitpunkt.format("%d.%m.%Y %H:%M"),
                    e.anzahl_ordner,
                    format_groesse(e.freigegeben_bytes)
                );
            }
            Ok(())
        }
    }
}
