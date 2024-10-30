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

use std::fs;
use std::path::Path;
use std::str::FromStr;

use crate::checks::{CheckNotice, Checkable};
use crate::model::onkostar_editor::OnkostarEditor;

pub fn check_file(file: &Path) -> Result<Vec<CheckNotice>, CheckNotice> {
    match fs::read_to_string(file) {
        Ok(content) => check(content),
        _ => Err(CheckNotice::Error {
            description: "Kann Datei nicht lesen".to_string(),
            line: None,
        }),
    }
}

pub fn check(content: String) -> Result<Vec<CheckNotice>, CheckNotice> {
    let mut result = content
        .lines()
        .enumerate()
        .flat_map(|(line, content)| check_line(line, content.to_string()))
        .collect::<Vec<_>>();

    let inner_checks = &mut match OnkostarEditor::from_str(content.as_str()) {
        Ok(data) => data.check(),
        Err(err) => {
            return Err(CheckNotice::Error {
                description: format!("Interner Fehler: {}", err),
                line: None,
            })
        }
    };
    result.append(inner_checks);

    Ok(result)
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
