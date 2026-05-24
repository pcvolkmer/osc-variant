/*
 * This file is part of osc-variant
 *
 * Copyright (C) 2023-2026 the original author or authors.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 */
use clap::{Parser, Subcommand};
use clap_complete::Shell;
use regex::Regex;
use std::str::FromStr;

#[derive(Parser)]
#[command(author, version, about)]
#[command(arg_required_else_help(true))]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: SubCommand,

    #[arg(short = 'v', global = true, help = "Zeige umfangreichere Ausgaben")]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum SubCommand {
    #[command(
        name = "completion",
        about = "Erzeuge und gebe Command-Completion aus",
        hide = true
    )]
    Completion { shell: Shell },
    #[command(
        name = "sha256sum",
        about = "Berechne SHA256 Prüfsumme für die angegebene Datei"
    )]
    Sha256Sum { inputfile: String },
    #[command(about = "Zeigt alle enthaltenen Kataloge und Formulare mit Revision an.")]
    List {
        inputfile: String,
        #[arg(
            long = "sorted",
            help = "Sortiere Kataloge und Formulare nach Name (Optional)"
        )]
        sorted: bool,
        #[arg(long = "filter", help = "Filtere Ausgabe nach Name (Optional)")]
        filter: Option<String>,
    },
    #[command(about = "Zeigt Kataloge und Formulare mit Revision und Abhängigkeiten an.")]
    Tree {
        inputfile: String,
        #[arg(
            long = "sorted",
            help = "Sortiere Kataloge und Formulare nach Name (Optional)"
        )]
        sorted: bool,
        #[arg(long = "filter", help = "Filtere Ausgabe nach Name (Optional)")]
        filter: Option<String>,
    },
    #[command(about = "Modifiziert die angegebene Datei anhand der Profildatei")]
    Modify {
        inputfile: String,
        #[arg(long = "profile", help = "Profildatei (optional)")]
        profile: Option<String>,
        #[arg(long = "notices", help = "CSV-Datei mit Ausfüllhinweisen (optional)")]
        noticefile: Option<String>,
        #[arg(long = "output", help = "Ausgabedatei (optional)")]
        outputfile: Option<String>,
        #[arg(long = "compact", help = "Kompakte Ausgabe, ohne Einrücken (Optional)")]
        compact: bool,
        #[arg(
            long = "sorted",
            alias = "x-sorted",
            help = "Sortiere Kataloge und Formulare nach Name und Abhängigkeiten (Optional)."
        )]
        sorted: bool,
        #[arg(
            long = "strip",
            alias = "x-strip",
            help = "Entferne Einträge aus der Systembibliothek die nicht importiert werden (Optional)."
        )]
        strip: bool,
        #[arg(
            long = "fix",
            help = "Erweiterte Problembehandlung und Reparatur der OSC-Datei"
        )]
        fix: bool,
    },
    #[command(about = "Vergleiche zwei Dateien anhand der Revision der enthaltenen Inhalte")]
    Diff {
        inputfile_a: String,
        inputfile_b: String,
        #[arg(long = "strict", help = "Strikter Vergleich des Inhalts")]
        strict: bool,
    },
    #[command(about = if cfg!(feature = "unzip-osb") { "Prüfe eine OSB- oder OSC-Datei auf bekannte Problemen" } else { "Prüfe eine OSC-Datei auf bekannte Problemen" }
    )]
    Check {
        #[arg(help = "Die zu prüfende Datei", group = "check-file", required = true)]
        file: Option<String>,
        #[arg(
            short = 'p',
            long = "password",
            help = "Passwort der OSB-Datei (Optional - für OSB-Dateien)",
            requires = "check-file",
            hide = !cfg!(feature = "unzip-osb")
        )]
        password: Option<String>,
        #[arg(
            long = "list",
            help = "Prüfe nicht und zeige Liste mit Checks auf bekannte Problemen",
            conflicts_with = "check-file"
        )]
        list: bool,
    },
    #[command(about = "Exportiere CSV-Datei mit Ausfüllhinweisen")]
    ExportNoticeCsv { inputfile: String },
    #[command(subcommand, about = "Befehle zur Nutzung von Bundles")]
    Bundle(BundleSubCommand),
    #[cfg(feature = "unzip-osb")]
    #[command(about = "Entpackt eine OSB-Datei")]
    UnzipOsb {
        file: String,
        #[arg(
            short = 'p',
            long = "password",
            help = "Passwort der OSB-Datei (Optional)"
        )]
        password: Option<String>,
        #[arg(short = 'd', long = "dir", help = "Zielverzeichnis (Optional)")]
        dir: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum BundleSubCommand {
    #[cfg(feature = "bundle-edit")]
    #[command(about = "Erstelle ein Bundle")]
    Create {
        #[arg(help = "Name des Bundles)")]
        bundle_name: String,
        #[arg(short = 'm', long = "message", help = "Beschreibung des Bundles")]
        description: String,
        #[arg(long = "license", help = "Lizenz des Bundles")]
        license: Option<String>,
        #[arg(long = "repository", help = "Quellcode-Repository des Bundles")]
        repository: Option<String>,
    },
    #[cfg(feature = "bundle-edit")]
    #[command(about = "Füge OSC-Datei als Bundle-Version hinzu")]
    AddVersion {
        #[arg(help = "Name des Bundles)")]
        bundle_name: String,
        file: String,
        #[arg(long = "tag", help = "Versions-Tag der Bundle-Version (Optional)")]
        tag: Option<String>,
        #[arg(
            short = 'm',
            long = "message",
            requires = "tag",
            help = "Beschreibung der Bundle-Version (Optional)"
        )]
        message: Option<String>,
    },
    #[command(about = "Liste alle Bundles auf")]
    List {
        #[arg(help = "Maximale Anzahl", default_value_t = 10)]
        limit: usize,
    },
    #[command(about = "Suche nach einem Bundle")]
    Search {
        #[arg(help = "Name des Bundles)")]
        bundle_name: String,
        #[arg(help = "Maximale Anzahl", default_value_t = 10)]
        limit: usize,
    },
    #[command(about = "Infos zu einem Bundle")]
    Info {
        #[arg(help = "Bundle-Version-Spezifikation ('Bundle-Name'[@'Versions-Tag'])")]
        spec: BundleVersionSpec,
    },
    #[command(about = "Exportiere ein Bundle als OSC-Datei")]
    Export {
        #[arg(help = "Bundle-Version-Spezifikation ('Bundle-Name'[@'Versions-Tag'])")]
        spec: BundleVersionSpec,
        #[arg(long = "compact", help = "Kompakte Ausgabe, ohne Einrücken (Optional)")]
        compact: bool,
    },
    #[cfg(feature = "bundle-edit")]
    #[command(about = "Räume das Repository auf")]
    Cleanup,
}

#[derive(Clone)]
pub struct BundleVersionSpec {
    pub bundle_name: String,
    pub version_tag: Option<String>,
}

impl FromStr for BundleVersionSpec {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('@');
        let bundle_name = parts.next().ok_or("Bundle-Name fehlt")?;
        let version_tag = parts.next().map(ToString::to_string);
        Ok(BundleVersionSpec {
            bundle_name: bundle_name.to_string(),
            version_tag: if let Some(version_tag) = version_tag {
                // Ensure strict semver as default
                let numbers = Regex::new(r"^\d").expect("Regex fehlerhaft");
                if numbers.is_match(&version_tag) {
                    Some(format!("={version_tag}"))
                } else {
                    Some(version_tag)
                }
            } else {
                None
            },
        })
    }
}
