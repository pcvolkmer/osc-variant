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
use std::fs::OpenOptions;
use std::io::Write;
use std::ops::Add;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::checks::{check_file, print_checks, CheckNotice};
use clap::Parser;
use console::style;
use dialoguer::Confirm;
use quick_xml::se::Serializer;
use serde::Serialize;
use sha256::digest;

use crate::cli::{Cli, SubCommand};
use crate::model::onkostar_editor::OnkostarEditor;
use crate::profile::Profile;

mod checks;
mod cli;
mod model;
mod profile;
#[cfg(feature = "unzip-osb")]
mod unzip_osb;

enum FileError {
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
                FileError::Parsing(filename, err) => format!(
                    "Die Datei '{}' ist entweder keine OSC-Datei, fehlerhaft oder enthält zusätzliche Inhalte\n{}",
                    filename,
                    err
                ),
            }
        )
    }
}

fn read_inputfile(inputfile: String) -> Result<OnkostarEditor, FileError> {
    return match fs::read_to_string(inputfile.clone()) {
        Ok(content) => match OnkostarEditor::from_str(content.as_str()) {
            Ok(data) => Ok(data),
            Err(err) => Err(FileError::Parsing(inputfile, err)),
        },
        Err(err) => Err(FileError::Reading(inputfile, err.to_string())),
    };
}

fn write_outputfile(filename: String, content: &String) -> Result<(), FileError> {
    OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename.clone())
        .map_err(|err| FileError::Writing(filename.clone(), err.to_string()))?
        .write_all(content.as_bytes())
        .map_err(|err| FileError::Writing(filename, err.to_string()))?;
    Ok(())
}

fn read_profile(filename: String) -> Result<Profile, FileError> {
    let profile = fs::read_to_string(filename.clone())
        .map_err(|err| FileError::Reading(filename.clone(), err.to_string()))?;
    let profile =
        Profile::from_str(profile.as_str()).map_err(|err| FileError::Reading(filename, err))?;
    Ok(profile)
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.cmd {
        SubCommand::List {
            inputfile,
            sorted,
            filter,
        } => {
            let mut data = read_inputfile(inputfile)?;
            if sorted {
                data.sorted()
            }
            if let Some(name) = filter {
                OnkostarEditor::print_list_filtered(&mut data, name.as_str());
                return Ok(());
            }
            data.print_list();
        }
        SubCommand::Tree {
            inputfile,
            sorted,
            filter,
        } => {
            let mut data = read_inputfile(inputfile)?;
            if sorted {
                data.sorted()
            }
            if let Some(name) = filter {
                OnkostarEditor::print_tree_filtered(&mut data, name.as_str());
                return Ok(());
            }
            OnkostarEditor::print_tree(&data);
        }
        SubCommand::Modify {
            inputfile,
            profile,
            outputfile,
            compact,
            sorted,
            strip,
            interactive,
            fix,
        } => {
            let data = &mut read_inputfile(inputfile)?;

            if let Some(profile) = profile {
                let profile = if profile.contains(".") {
                    read_profile(profile.clone()).map_err(|_| {
                        FileError::Reading(profile, "Kann Profildatei nicht lesen!".into())
                    })?
                } else {
                    Profile::embedded_profile(profile.as_str())?
                };

                data.apply_profile(&profile);
            }

            let mut compact = compact;
            let mut sorted = sorted;
            let mut strip = strip;

            if interactive {
                compact = Confirm::new()
                    .with_prompt("Kompakte Ausgabe, ohne Einrücken?")
                    .default(compact)
                    .interact()
                    .unwrap();

                sorted = Confirm::new()
                    .with_prompt("Sortiere Kataloge und Formulare nach Name und Abhängigkeiten?")
                    .default(sorted)
                    .interact()
                    .unwrap();

                strip = Confirm::new()
                    .with_prompt(
                        "Entferne Einträge aus der Systembibliothek die nicht importiert werden?",
                    )
                    .default(strip)
                    .interact()
                    .unwrap();
            }

            if fix {
                // No operation as of now
            }

            if sorted {
                data.sorted();
            }

            if strip {
                data.strip_system_library_content();
            }

            let mut buf = String::new();

            let mut serializer = Serializer::new(&mut buf);
            if !compact {
                serializer.indent(' ', 2);
            }

            data.serialize(serializer).expect("Generated XML");

            let output = &"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n"
                .to_string()
                .add(
                    buf
                        // Replace &apos; and &quot; as used in original file
                        .replace("&apos;", "'")
                        .replace("&quot;", "\"")
                        .as_str(),
                );

            match outputfile {
                Some(filename) => write_outputfile(filename, output)?,
                None => {
                    println!("{}", output)
                }
            }
        }
        SubCommand::Diff {
            inputfile_a,
            inputfile_b,
            strict,
        } => {
            println!(
                "Vergleiche Datei A ({}) mit Datei B ({})",
                style(&inputfile_a).yellow(),
                style(&inputfile_b).yellow()
            );

            let data_a = &mut read_inputfile(inputfile_a)?;
            let data_b = &mut read_inputfile(inputfile_b)?;

            data_a.print_diff(data_b, strict);
        }
        SubCommand::Sha256Sum { inputfile } => {
            match fs::read_to_string(inputfile.clone()) {
                Ok(content) => {
                    println!(
                        "{}  {}",
                        digest(content).as_str(),
                        PathBuf::from(inputfile.clone())
                            .canonicalize()
                            .unwrap_or_default()
                            .to_str()
                            .unwrap_or_default()
                    )
                }
                Err(err) => {
                    eprintln!("{}", FileError::Reading(inputfile, err.to_string()));
                }
            };
        }
        SubCommand::Check {
            file,
            list,
            password,
        } => {
            if list {
                print_checks();
            } else {
                match check_file(Path::new(file.unwrap_or_default().as_str()), password) {
                    Ok(notices) => {
                        println!(
                            "Es wurden {} Probleme gefunden\n",
                            notices
                                .iter()
                                .filter(|notice| match notice {
                                    CheckNotice::ErrorWithCode { .. }
                                    | CheckNotice::Error { .. } => true,
                                    _ => false,
                                })
                                .count()
                        );
                        notices
                            .iter()
                            .for_each(|check_notice| println!("{}", check_notice));
                    }
                    Err(err) => {
                        println!("{}", err)
                    }
                }
            }
        }
        #[cfg(feature = "unzip-osb")]
        SubCommand::UnzipOsb {
            file,
            password,
            dir,
        } => {
            use crate::unzip_osb::{unzip_osb, unzip_osb_using_password};
            match password {
                Some(password) => unzip_osb_using_password(
                    file.as_str(),
                    dir.unwrap_or_default().as_str(),
                    password.as_str(),
                ),
                None => unzip_osb(file.as_str(), dir.unwrap_or_default().as_str()),
            }
        }
    };

    Ok(())
}
