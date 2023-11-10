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

pub trait Fixable {
    fn fix(&mut self) -> bool;
}

#[allow(unused_variables)]
pub fn check_file(file: &Path, password: Option<String>) -> Result<Vec<CheckNotice>, CheckNotice> {
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

pub fn print_checks() {
    println!(
        "{}",
        style("Die folgenden Probleme sind bekannt\n")
            .yellow()
            .bold()
    );

    struct Problem<'a> {
        code: &'a str,
        name: &'a str,
        description: &'a str,
        fixable: bool,
    }

    impl<'a> Display for Problem<'a> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{} {} {}\n\n{}",
                style(self.code).bold(),
                style(self.name).underlined(),
                match self.fixable {
                    true => style("(Behebbar)").green(),
                    false => style("(Nicht behebbar)").red(),
                },
                self.description
            )
        }
    }

    vec![
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
    ]
    .iter()
    .for_each(|problem| println!("{}\n", problem))
}
