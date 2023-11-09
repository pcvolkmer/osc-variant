/*
 * MIT License
 *
 * Copyright (c) 2023 Comprehensive Cancer Center Mainfranken
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use clap::{Command, CommandFactory, Parser, Subcommand};

#[allow(dead_code)]
fn build_cli() -> Command {
    Cli::command()
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true, arg_required_else_help(true))]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Subcommand)]
pub enum SubCommand {
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
            short = 'i',
            long = "interactive",
            help = "Starte interaktiven Dialog zum Modifizieren von OSC-Dateien"
        )]
        interactive: bool,
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
    #[command(about = "Überprüfe OSC-Datei auf bekannte Problemen")]
    Check {
        #[arg(help = "Die zu prüfende Datei", group = "check-file", required = true)]
        file: Option<String>,
        #[arg(
            short = 'p',
            long = "password",
            help = "Passwort der OSB-Datei (Optional - für OSB-Dateien)",
            requires = "check-file"
        )]
        password: Option<String>,
        #[arg(
            long = "list",
            help = "Prüfe nicht und zeige Liste mit Checks auf bekannte Problemen",
            conflicts_with = "check-file"
        )]
        list: bool,
    },
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
        #[arg(short = 'd', help = "Zielverzeichnis (Optional)")]
        dir: Option<String>,
    },
}
