/*
 * This file is part of osc-variant
 *
 * Copyright (C) 2025 the original author or authors.
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

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::marker::PhantomData;
use std::path::Path;
use std::str::FromStr;

use crate::model::onkostar_editor::OnkostarEditor;
use crate::profile::Profile;

pub enum FileError {
    Reading(String, String),
    Writing(String, String),
    Parsing(String, String),
}

impl Error for FileError {}

impl Debug for FileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for FileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                FileError::Reading(filename, err) => format!("Kann Datei '{filename}' nicht lesen: {err}"),
                FileError::Writing(filename, err) => format!("Kann Datei '{filename}' nicht schreiben: {err}"),
                FileError::Parsing(filename, err) => format!(
                    "Die Datei '{filename}' wird nicht unterstützt, ist fehlerhaft oder enthält zusätzliche Inhalte\n{err}"
                )
            }
        )
    }
}

#[allow(dead_code)]
pub enum InputFile {
    Osc {
        filename: String,
        content: String,
    },
    Osb {
        filename: String,
        content: Vec<InputFile>,
    },
    Yaml {
        filename: String,
        content: String,
    },
    Other {
        filename: String,
        content: Vec<u8>,
    },
}

impl InputFile {
    pub fn filename(&self) -> String {
        match self {
            InputFile::Osb { filename, .. }
            | InputFile::Osc { filename, .. }
            | InputFile::Yaml { filename, .. }
            | InputFile::Other { filename, .. } => filename.clone(),
        }
    }

    #[allow(unused_variables)]
    pub fn read(filename: String, password: Option<String>) -> Result<Self, FileError> {
        if let Some(extension) = Path::new(filename.as_str()).extension() {
            return match extension.to_str() {
                Some("osc") => match fs::read_to_string(filename.clone()) {
                    Ok(content) => Ok(InputFile::Osc { filename, content }),
                    Err(err) => Err(FileError::Reading(filename, err.to_string())),
                },
                #[cfg(feature = "unzip-osb")]
                Some("osb") => {
                    use bytes::BytesMut;
                    use deob::deobfuscate;
                    use std::io::Read;

                    let file = match fs::File::open(filename.clone()) {
                        Ok(file) => file,
                        Err(err) => return Err(FileError::Reading(filename, err.to_string())),
                    };

                    let mut archive = match zip::ZipArchive::new(file) {
                        Ok(file) => file,
                        Err(err) => return Err(FileError::Reading(filename, err.to_string())),
                    };

                    let mut result = vec![];

                    let password = password.unwrap_or_else(|| deobfuscate(env!("OSB_KEY").trim()));

                    for i in 0..archive.len() {
                        if let Ok(mut zip_file) = archive.by_index_decrypt(i, password.as_bytes()) {
                            if zip_file.is_file()
                                && zip_file.name().to_lowercase().ends_with(".osc")
                            {
                                let mut buf = String::new();
                                let _ = zip_file.read_to_string(&mut buf);
                                result.push(InputFile::Osc {
                                    filename: zip_file.name().to_string(),
                                    content: buf,
                                });
                            } else {
                                let mut buf = BytesMut::new();
                                let _ = zip_file.read(&mut buf);
                                result.push(InputFile::Other {
                                    filename: zip_file.name().to_string(),
                                    content: buf.to_vec(),
                                });
                            }
                        } else {
                            return Err(FileError::Parsing(
                                filename,
                                "Kann OSB-Datei nicht lesen".to_string(),
                            ));
                        }
                    }

                    Ok(InputFile::Osb {
                        filename,
                        content: result,
                    })
                }
                Some("yml" | "yaml") => match fs::read_to_string(filename.clone()) {
                    Ok(content) => Ok(InputFile::Yaml { filename, content }),
                    Err(err) => Err(FileError::Reading(filename, err.to_string())),
                },
                _ => Err(FileError::Parsing(
                    filename,
                    "Kein unterstütztes Dateiformat".to_string(),
                )),
            };
        }

        Err(FileError::Reading(filename, String::new()))
    }
}

impl TryFrom<InputFile> for OnkostarEditor {
    type Error = FileError;

    fn try_from(value: InputFile) -> Result<Self, Self::Error> {
        match value {
            InputFile::Osc {
                filename, content, ..
            } => match OnkostarEditor::from_str(content.as_str()) {
                Ok(data) => Ok(data),
                Err(err) => Err(FileError::Parsing(filename, err)),
            },
            InputFile::Osb { filename, .. }
            | InputFile::Yaml { filename, .. }
            | InputFile::Other { filename, .. } => {
                Err(FileError::Parsing(filename, "Keine OSC-Datei".to_string()))
            }
        }
    }
}

impl TryFrom<InputFile> for Profile {
    type Error = FileError;

    fn try_from(value: InputFile) -> Result<Self, Self::Error> {
        match value {
            InputFile::Yaml { filename, content } => match Profile::from_str(&content) {
                Ok(profile) => Ok(profile),
                Err(err) => Err(FileError::Parsing(filename, err)),
            },
            InputFile::Osc { filename, .. }
            | InputFile::Osb { filename, .. }
            | InputFile::Other { filename, .. } => Err(FileError::Parsing(
                filename,
                "Keine Profildatei".to_string(),
            )),
        }
    }
}

/// Shortcut methods for OSC and Profile files
pub struct FileReader<FileType> {
    file_type: PhantomData<FileType>,
}

impl FileReader<OnkostarEditor> {
    pub fn read(filename: &str) -> Result<OnkostarEditor, FileError> {
        TryInto::<OnkostarEditor>::try_into(InputFile::read(filename.to_string(), None)?)
    }
}

impl FileReader<Profile> {
    pub fn read(filename: &str) -> Result<Profile, FileError> {
        TryInto::<Profile>::try_into(InputFile::read(filename.to_string(), None)?)
    }
}
