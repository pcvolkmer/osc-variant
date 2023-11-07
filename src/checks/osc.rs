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
use std::path::Path;
use std::str::FromStr;

pub fn check_file(file: &Path) -> Vec<CheckNotice> {
    match fs::read_to_string(file) {
        Ok(content) => check(content),
        _ => vec![CheckNotice::Error {
            description: "Kann Datei nicht lesen".to_string(),
            line: None,
        }],
    }
}

pub fn check(content: String) -> Vec<CheckNotice> {
    let mut result = content
        .lines()
        .enumerate()
        .flat_map(|(line, content)| check_line(line, content.to_string()))
        .collect::<Vec<_>>();

    let inner_checks = &mut match OnkostarEditor::from_str(content.as_str()) {
        Ok(data) => data.check(),
        Err(err) => vec![CheckNotice::Error {
            description: format!("Interner Fehler: {}", err),
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
