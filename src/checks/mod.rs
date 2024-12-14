/*
 * This file is part of osc-variant
 *
 * Copyright (C) 2023-2024 the original author or authors.
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

use std::fmt::{Display, Formatter};
use std::path::Path;

use console::style;

#[cfg(feature = "unzip-osb")]
pub mod osb;
pub mod osc;

#[allow(dead_code)]
pub enum CheckNotice {
    /// This will result in Error if importing file and has a support code
    ErrorWithCode {
        code: String,
        description: String,
        line: Option<usize>,
        example: Option<String>,
    },
    /// This will result in Error if importing file
    Error {
        description: String,
        line: Option<usize>,
    },
    /// Other known issues
    Warning {
        description: String,
        line: Option<usize>,
    },
    /// Other known issues
    Info {
        description: String,
        line: Option<usize>,
    },
    /// Ok
    Ok(String),
}

impl Display for CheckNotice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckNotice::ErrorWithCode {
                code,
                description,
                line,
                example,
            } => match line {
                Some(line) => write!(
                    f,
                    "{: <7} ({}) at Line {}: {}{}",
                    style("ERROR").red().bold(),
                    code,
                    line,
                    description,
                    match example {
                        Some(example) => format!("\n        🔥 '{}'", style(example).dim()),
                        _ => String::new(),
                    }
                ),
                None => write!(
                    f,
                    "{: <7} ({}): {}{}",
                    style("ERROR").red().bold(),
                    code,
                    description,
                    match example {
                        Some(example) => format!("\n        🔥 '{}'", style(example).dim()),
                        _ => String::new(),
                    }
                ),
            },
            CheckNotice::Error { description, line } => match line {
                Some(line) => write!(
                    f,
                    "{: <7} at Line {}: {}",
                    style("ERROR").red().bold(),
                    line,
                    description
                ),
                None => write!(f, "{: <7} {}", style("ERROR").red().bold(), description),
            },
            CheckNotice::Warning { description, line } => match line {
                Some(line) => write!(
                    f,
                    "{: <7} at Line {}: {}",
                    style("WARNING").yellow().bold(),
                    line,
                    description
                ),
                None => write!(
                    f,
                    "{: <7} {}",
                    style("WARNING").yellow().bold(),
                    description
                ),
            },
            CheckNotice::Info { description, line } => match line {
                Some(line) => write!(
                    f,
                    "{: <7} at Line {}: {}",
                    style("INFO").blue().bold(),
                    line,
                    description
                ),
                None => write!(f, "{: <7} {}", style("INFO").blue().bold(), description),
            },
            CheckNotice::Ok(msg) => write!(f, "{: <7} {}", style("OK").green(), msg),
        }
    }
}

pub trait Checkable {
    fn check(&self) -> Vec<CheckNotice>;
}

#[allow(unused)]
pub trait Fixable {
    fn fix(&mut self) -> bool;
}

#[allow(unused_variables)]
pub fn check_file(file: &Path, password: &Option<String>) -> Result<Vec<CheckNotice>, CheckNotice> {
    match file.extension() {
        Some(ex) => match ex.to_str() {
            #[cfg(feature = "unzip-osb")]
            Some("osb") => match password {
                Some(password) => osb::check_file(file, password.as_str()),
                None => {
                    use deob::deobfuscate;
                    osb::check_file(file, deobfuscate(env!("OSB_KEY").trim()).as_str())
                }
            },
            Some("osc") => osc::check_file(file),
            _ => Err(CheckNotice::Error {
                description: "Keine prüfbare Datei".to_string(),
                line: None,
            }),
        },
        _ => Err(CheckNotice::Error {
            description: "Keine prüfbare Datei".to_string(),
            line: None,
        }),
    }
}

pub fn print() {
    struct Problem<'a> {
        code: &'a str,
        name: &'a str,
        description: &'a str,
        fixable: bool,
    }

    impl Display for Problem<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{} {} {}\n\n{}",
                style(self.code).bold(),
                style(self.name).underlined(),
                if self.fixable {
                    style("(Behebbar)").green()
                } else {
                    style("(Nicht behebbar)").red()
                },
                self.description
            )
        }
    }

    println!(
        "{}",
        style("Die folgenden Probleme sind bekannt\n")
            .yellow()
            .bold()
    );

    for problem in vec![
        Problem {
            code: "2023-0001",
            name: "Unterformular mit Markierung 'hat Unterformulare'",
            description: "  Aktuell gibt es keine Unterformulare in Unterformularen, daher\n  \
            sollte dies nicht vorkommen.\n\n  \
            Eine mögliche Ursache ist die Speicherung eines Unterformulars als Formular.",
            fixable: false,
        },
        Problem {
            code: "2023-0002",
            name: "Formular hat keine Angabe zum Prozedurdatum",
            description: "  Formulare benötigen die Angabe des Prozedurdatums, anderenfalls\n  \
            führt dies zu Problemen in Onkostar.\n\n  \
            Unterformulare können ein Prozedurdatum haben, müssen es aber nicht.\n\n  \
            Eine mögliche Ursache ist die Speicherung eines Formulars als Unterformular.",
            fixable: false,
        },
        Problem {
            code: "2023-0003",
            name: "Leerzeichen am Ende der Plausibilitätsregel-Bezeichnung (OSTARSUPP-13334)",
            description:
                "  Treten Leerzeichen am Ende der Plausibilitätsregel-Bezeichnung auf,\n  \
            führt dies zu Fehlern beim Import der OSC-Datei.\n\n  \
            Das Problem wird beim Verwenden des Unterbefehls 'modify' automatisch\n  \
            behoben und Leerzeichen entfernt. 
            ",
            fixable: true,
        },
        Problem {
            code: "2023-0004",
            name: "Verweis auf noch nicht definiertes Formular (OSTARSUPP-13212)",
            description: "  Wenn ein Formular einen Verweis auf ein anderes Formular enthält,\n  \
            das nicht vor diesem Formular in der OSC-Datei definiert ist, wird der\n  \
            Formularverweis beim Import der OSC-Datei nicht übernommen.\n\n  \
            Dies kann bei wechselseitiger Abhängigkeit zwischen zwei (Unter-)Formularen\n  \
            auftreten.\n\n  \
            In diesem Fall kann ein erneuter/zweiter Import helfen, da das Onkostar in\n  \
            diesem Fall alle Formulare importiert hat und der Formularverweis dann \n  \
            gespeichert werden kann. 
            ",
            fixable: false,
        },
        Problem {
            code: "2024-0005",
            name: "Formular hat Formularverweise ohne Angabe des Formulars in den Formularfeldern",
            description: "  Formularverweise ohne Angabe des Formulars führen zu Problemen\n  \
            bei der Verwendung und Darstellung des entsprechenden Formularverweises.\n\n  \
            Dieses Problem muss manuell behoben werden.
            ",
            fixable: false,
        },
    ] {
        println!("{problem}\n");
    }
}
