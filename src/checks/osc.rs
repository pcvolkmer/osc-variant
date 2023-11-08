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

use crate::checks::{CheckNotice, Checkable};
use crate::model::onkostar_editor::OnkostarEditor;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

pub fn check(file: &Path) -> Vec<CheckNotice> {
    let mut result = match File::open(file) {
        Ok(file) => BufReader::new(file)
            .lines()
            .enumerate()
            .flat_map(|(line, content)| match content {
                Ok(content) => check_line(line, content),
                _ => {
                    return vec![CheckNotice::Error {
                        description: "Cannot read line".to_string(),
                        line: Some(line),
                    }]
                }
            })
            .collect::<Vec<_>>(),
        _ => {
            return vec![CheckNotice::Error {
                description: "Kann Datei nicht lesen".to_string(),
                line: None,
            }]
        }
    };

    let inner_checks = &mut match fs::read_to_string(file) {
        Ok(content) => match OnkostarEditor::from_str(content.as_str()) {
            Ok(data) => data.check(),
            Err(err) => vec![CheckNotice::Error {
                description: format!("Interner Fehler: {}", err),
                line: None,
            }],
        },
        _ => vec![CheckNotice::Error {
            description: "Kann Datei nicht lesen".to_string(),
            line: None,
        }],
    };
    result.append(inner_checks);

    result
}

fn check_line(line: usize, content: String) -> Vec<CheckNotice> {
    let mut result = vec![];

    if content.contains(" </Bezeichnung>") {
        result.append(&mut vec![CheckNotice::ErrorWithCode {
            code: "2023-0003".to_string(),
            description:
                "Leerzeichen am Ende der Plausibilitätsregel-Bezeichnung (OSTARSUPP-13334)"
                    .to_string(),
            line: Some(line),
            example: Some(content.trim().to_string()),
        }])
    }

    result
}
