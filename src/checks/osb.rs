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

use std::fs;
use std::io::Read;
use std::path::Path;

use crate::checks::{osc, CheckNotice};

#[cfg(feature = "unzip-osb")]
pub fn check_file(file: &Path, password: &str) -> Vec<CheckNotice> {
    let file = match fs::File::open(file) {
        Ok(file) => file,
        Err(err) => {
            return vec![CheckNotice::Error {
                description: format!("Kann Datei nicht lesen: {}", err),
                line: None,
            }];
        }
    };

    let mut archive = match zip::ZipArchive::new(file) {
        Ok(file) => file,
        Err(err) => {
            return vec![CheckNotice::Error {
                description: format!("Kann Datei nicht lesen: {}", err),
                line: None,
            }];
        }
    };

    let mut result = vec![];

    for i in 0..archive.len() {
        if let Ok(Ok(mut zip_file)) = archive.by_index_decrypt(i, password.as_bytes()) {
            if zip_file.is_file() && zip_file.name().ends_with(".osc") {
                result.push(CheckNotice::Info {
                    description: format!("Prüfe Eintrag '{}'", zip_file.name()),
                    line: None,
                });
                let mut buf = String::new();
                let _ = zip_file.read_to_string(&mut buf);
                result.append(&mut osc::check(buf));
                continue;
            }
            result.push(CheckNotice::Warning {
                description: format!("Überspringe Eintrag '{}'", zip_file.name()),
                line: None,
            })
        } else {
            return vec![CheckNotice::Error {
                description: format!("Kann Datei nicht lesen"),
                line: None,
            }];
        }
    }

    result
}
