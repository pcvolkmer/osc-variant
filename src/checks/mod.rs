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
use std::fmt::{Display, Formatter};
use std::path::Path;

use crate::checks::CheckNotice::{ErrorWithCode, Info};
use console::style;
use model::osc::form::{DataFormType, Form, UnterformularType};
use model::osc::onkostar_editor::OnkostarEditor;
use model::osc::requirements::{Requirement, Requires};
use model::osc::{Comparable, Named, TypedEntry};

#[cfg(feature = "unzip-osb")]
pub mod osb;
pub mod osc;

#[allow(dead_code)]
#[derive(Debug)]
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

impl CheckNotice {
    fn variant_order(&self) -> u8 {
        match self {
            CheckNotice::ErrorWithCode { .. } | CheckNotice::Error { .. } => 1,
            CheckNotice::Warning { .. } => 2,
            CheckNotice::Info { .. } => 3,
            CheckNotice::Ok(_) => 4,
        }
    }
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
#[allow(clippy::needless_pass_by_value)]
pub fn check_file(file: &Path, password: Option<String>) -> Result<Vec<CheckNotice>, CheckNotice> {
    let mut result = match file.extension() {
        Some(ex) => match ex.to_str() {
            #[cfg(feature = "unzip-osb")]
            Some("osb") => {
                use deob::deobfuscate;
                osb::check_file(
                    file,
                    &password.unwrap_or_else(|| deobfuscate(env!("OSB_KEY").trim())),
                )
            }
            Some("osc") => osc::check_file(file),
            _ => Err(CheckNotice::Error {
                description: "Keine prüfbare Datei".to_string(),
                line: None,
            }),
        }?,
        _ => {
            return Err(CheckNotice::Error {
                description: "Keine prüfbare Datei".to_string(),
                line: None,
            });
        }
    };
    result.sort_by_key(CheckNotice::variant_order);
    Ok(result)
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

    for problem in [
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

impl Checkable for OnkostarEditor {
    fn check(&self) -> Vec<CheckNotice> {
        fn requirement_error(
            form: &impl Comparable,
            item: &impl Comparable,
            t: &str,
        ) -> CheckNotice {
            CheckNotice::ErrorWithCode {
                code: "2023-0004".to_string(),
                description: format!(
                    "'{}' hat einen Verweis auf zuvor nicht definiertes {t} '{}' (OSTARSUPP-13212)",
                    form.get_name(),
                    item.get_name()
                ),
                line: None,
                example: None,
            }
        }

        // Inner form checks

        let mut result = self
            .editor
            .data_form
            .iter()
            .flat_map(Form::check)
            .collect::<Vec<_>>();

        let other = &mut self
            .editor
            .unterformular
            .iter()
            .flat_map(Form::check)
            .collect::<Vec<_>>();

        result.append(other);

        // Check requirements

        let mut requirement_checked_forms = vec![];

        self.editor.unterformular.iter().for_each(|form| {
            requirement_checked_forms.push(form.get_name());
            form.get_required_entries(self)
                .iter()
                .for_each(|entry| match entry {
                    Requirement::DataFormReference(item) => {
                        if !requirement_checked_forms.contains(&item.get_name()) {
                            result.push(requirement_error(form, *item, "Formular"));
                        }
                    }
                    Requirement::UnterformularReference(item) => {
                        if !requirement_checked_forms.contains(&item.get_name()) {
                            result.push(requirement_error(form, *item, "Unterformular"));
                        }
                    }
                    _ => {}
                });
        });

        self.editor.data_form.iter().for_each(|form| {
            requirement_checked_forms.push(form.get_name());
            form.get_required_entries(self)
                .iter()
                .for_each(|entry| match entry {
                    Requirement::DataFormReference(item) => {
                        if !requirement_checked_forms.contains(&item.get_name()) {
                            result.push(requirement_error(form, *item, "Formular"));
                        }
                    }
                    Requirement::UnterformularReference(item) => {
                        if !requirement_checked_forms.contains(&item.get_name()) {
                            result.push(requirement_error(form, *item, "Unterformular"));
                        }
                    }
                    _ => {}
                });
        });

        result
    }
}

fn common_check<T>(form: &Form<T>) -> Vec<CheckNotice> {
    let missing_forms_in_refs = match form.entries {
        Some(ref entries) => entries
            .entry
            .iter()
            .filter(|entry| {
                entry.is_form_reference()
                    && entry.referenced_data_form.is_none()
                    && entry.data_form_references.is_none()
            })
            .map(|entry| format!("'{}'", entry.get_name()))
            .collect::<Vec<_>>(),
        None => vec![],
    };

    let missing_forms_in_refs_legacy = match form.entries {
        Some(ref entries) => entries
            .entry
            .iter()
            .filter(|entry| entry.is_form_reference() && entry.referenced_data_form.is_none())
            .map(|entry| format!("'{}'", entry.get_name()))
            .collect::<Vec<_>>(),
        None => vec![],
    };

    let mut result = vec![];

    if !missing_forms_in_refs.is_empty() && !missing_forms_in_refs_legacy.is_empty() {
        result.push(ErrorWithCode {
            code: "2024-0005".to_string(),
            description: format!(
                "Formular '{}' hat Formularverweise ohne Angabe des Formulars in: {}",
                form.get_name(),
                missing_forms_in_refs.join(", ")
            ),
            line: None,
            example: None,
        });
    }

    if missing_forms_in_refs.is_empty() && !missing_forms_in_refs_legacy.is_empty() {
        result.push(Info {
            description: format!(
                "Formular '{}' hat Formularverweise, die erst in neueren Onkostar-Versionen ab 2.14.0 funktionieren",
                form.get_name()
            ),
            line: None,
        });
    }

    result
}

impl Checkable for Form<DataFormType> {
    fn check(&self) -> Vec<CheckNotice> {
        let mut result = match self.entries {
            Some(ref entries) => {
                if entries
                    .entry
                    .iter()
                    .filter(|entry| entry.procedure_date_status != "none")
                    .count()
                    == 0
                {
                    vec![ErrorWithCode {
                        code: "2023-0002".to_string(),
                        description: format!(
                            "Formular '{}' hat keine Angabe zum Prozedurdatum",
                            self.get_name()
                        ),
                        line: None,
                        example: None,
                    }]
                } else {
                    vec![]
                }
            }
            None => vec![],
        };

        result.append(&mut common_check(self));

        result
    }
}

impl Checkable for Form<UnterformularType> {
    fn check(&self) -> Vec<CheckNotice> {
        let mut result = if self.hat_unterformulare {
            vec![ErrorWithCode {
                code: "2023-0001".to_string(),
                description: format!(
                    "Unterformular '{}' mit Markierung 'hat Unterformulare'",
                    self.get_name()
                ),
                line: None,
                example: None,
            }]
        } else {
            vec![]
        };

        result.append(&mut common_check(self));

        result
    }
}
