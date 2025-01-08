/*
 * This file is part of osc-variant
 *
 * Copyright (C) 2024 the original author or authors.
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

use std::fs;
use std::io::Read;
use std::path::Path;

use indicatif::{ProgressBar, ProgressStyle};

use crate::checks::{osc, CheckNotice};

#[cfg(feature = "unzip-osb")]
pub fn check_file(file: &Path, password: &str) -> Result<Vec<CheckNotice>, CheckNotice> {
    let file = match fs::File::open(file) {
        Ok(file) => file,
        Err(err) => {
            return Err(CheckNotice::Error {
                description: format!("Kann Datei nicht lesen: {err}"),
                line: None,
            });
        }
    };

    let mut archive = match zip::ZipArchive::new(file) {
        Ok(file) => file,
        Err(err) => {
            return Err(CheckNotice::Error {
                description: format!("Kann Datei nicht lesen: {err}"),
                line: None,
            });
        }
    };

    let mut result = vec![];

    #[allow(clippy::unwrap_used)]
    let progress_bar = ProgressBar::new(archive.len() as u64).with_style(
        ProgressStyle::default_bar()
            .template("{wide_bar} {msg:32} {pos}/{len}")
            .unwrap(),
    );

    for i in 0..archive.len() {
        progress_bar.inc(1);
        if let Ok(mut zip_file) = archive.by_index_decrypt(i, password.as_bytes()) {
            progress_bar.set_message(zip_file.name().to_string());
            if zip_file.is_file() && zip_file.name().ends_with(".osc") {
                let mut buf = String::new();
                let _ = zip_file.read_to_string(&mut buf);
                match osc::check(&buf) {
                    Ok(ref mut check_result) => {
                        result.push(CheckNotice::Info {
                            description: format!("Prüfe Eintrag '{}'", zip_file.name()),
                            line: None,
                        });
                        if check_result.is_empty() {
                            result.push(CheckNotice::Ok(format!(
                                "Keine Probleme in '{}' erkannt",
                                zip_file.name()
                            )));
                        }
                        result.append(check_result);
                    }
                    Err(_) => result.push(CheckNotice::Warning {
                        description: format!(
                            "Überspringe Eintrag '{}': Inhalt kann nicht geprüft werden",
                            zip_file.name(),
                        ),
                        line: None,
                    }),
                };
                continue;
            }
            if zip_file.is_file() {
                result.push(CheckNotice::Warning {
                    description: format!(
                        "Überspringe Eintrag '{}': Keine OSC-Datei",
                        zip_file.name()
                    ),
                    line: None,
                });
            }
        } else {
            return Err(CheckNotice::Error {
                description: "Kann Datei nicht lesen".to_string(),
                line: None,
            });
        }
    }
    progress_bar.finish_and_clear();

    Ok(result)
}
