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

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::path::Path;
use std::str::FromStr;

use crate::model::onkostar_editor::OnkostarEditor;

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
                FileError::Reading(filename, err) => format!("Kann Datei '{}' nicht lesen: {}", filename, err),
                FileError::Writing(filename, err) => format!("Kann Datei '{}' nicht schreiben: {}", filename, err),
                #[cfg(feature = "unzip-osb")]
                FileError::Parsing(filename, err) => format!(
                    "Die Datei '{}' ist entweder keine OSB- oder OSC-Datei, fehlerhaft oder enthält zusätzliche Inhalte\n{}",
                    filename,
                    err
                ),
                #[cfg(not(feature = "unzip-osb"))]
                FileError::Parsing(filename, err) => format!(
                    "Die Datei '{}' ist keine OSC-Datei, fehlerhaft oder enthält zusätzliche Inhalte\n{}",
                    filename,
                    err
                ),
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
    Profile {
        filename: String,
        content: String,
    },
    Osb {
        filename: String,
        content: Vec<InputFile>,
    },
    Other {
        filename: String,
        content: Vec<u8>,
    },
}

impl InputFile {
    pub fn filename(&self) -> String {
        match self {
            InputFile::Osc { filename, .. } => filename,
            InputFile::Profile { filename, .. } => filename,
            InputFile::Osb { filename, .. } => filename,
            InputFile::Other { filename, .. } => filename,
        }
        .to_string()
    }

    pub fn read(filename: String, _password: Option<String>) -> Result<Self, FileError> {
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

                    let password = _password.unwrap_or_else(|| deobfuscate(env!("OSB_KEY").trim()));

                    for i in 0..archive.len() {
                        if let Ok(Ok(mut zip_file)) =
                            archive.by_index_decrypt(i, password.as_bytes())
                        {
                            if zip_file.is_file() && zip_file.name().ends_with(".osc") {
                                let mut buf = String::new();
                                let _ = zip_file.read_to_string(&mut buf);
                                result.push(InputFile::Osc {
                                    filename: zip_file.name().to_string(),
                                    content: buf,
                                })
                            } else {
                                let mut buf = BytesMut::new();
                                let _ = zip_file.read(&mut buf);
                                result.push(InputFile::Other {
                                    filename: zip_file.name().to_string(),
                                    content: buf.to_vec(),
                                })
                            }
                        } else {
                            return Err(FileError::Parsing(
                                filename.into(),
                                "Kann OSB-Datei nicht lesen".to_string(),
                            ));
                        }
                    }

                    Ok(InputFile::Osb {
                        filename,
                        content: result,
                    })
                }
                #[cfg(feature = "unzip-osb")]
                _ => Err(FileError::Parsing(
                    filename,
                    "Nur OSB- oder OSC-Dateien werden unterstützt".to_string(),
                )),
                #[cfg(not(feature = "unzip-osb"))]
                _ => Err(FileError::Parsing(
                    filename,
                    "Nur OSC-Dateien werden unterstützt".to_string(),
                )),
            };
        }

        Err(FileError::Reading(filename, String::new()))
    }
}

impl TryFrom<InputFile> for OnkostarEditor {
    type Error = FileError;

    fn try_from(value: InputFile) -> Result<Self, Self::Error> {
        return match value {
            InputFile::Osc {
                filename, content, ..
            } => match OnkostarEditor::from_str(content.as_str()) {
                Ok(data) => Ok(data),
                Err(err) => Err(FileError::Parsing(filename, err)),
            },
            InputFile::Osb { filename, .. }
            | InputFile::Profile { filename, .. }
            | InputFile::Other { filename, .. } => {
                Err(FileError::Parsing(filename, "Keine OSC-Datei".to_string()))
            }
        };
    }
}
