/*
 * MIT License
 *
 * Copyright (c) 2024 Comprehensive Cancer Center Mainfranken
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
                description: format!("Kann Datei nicht lesen: {}", err),
                line: None,
            });
        }
    };

    let mut archive = match zip::ZipArchive::new(file) {
        Ok(file) => file,
        Err(err) => {
            return Err(CheckNotice::Error {
                description: format!("Kann Datei nicht lesen: {}", err),
                line: None,
            });
        }
    };

    let mut result = vec![];

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
                match osc::check(buf) {
                    Ok(ref mut check_result) => {
                        result.push(CheckNotice::Info {
                            description: format!("Prüfe Eintrag '{}'", zip_file.name()),
                            line: None,
                        });
                        if check_result.is_empty() {
                            result.push(CheckNotice::Ok(format!(
                                "Keine Probleme in '{}' erkannt",
                                zip_file.name()
                            )))
                        }
                        result.append(check_result)
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
                })
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
