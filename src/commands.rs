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
use crate::checks::{check_file, print, CheckNotice};
use crate::cli::{Cli, SubCommand};
use crate::file_io::{FileError, FileReader, InputFile};
use crate::model::onkostar_editor::OnkostarEditor;
use crate::profile::Profile;
use clap::CommandFactory;
use clap_complete::generate;
use console::style;
use dialoguer::Confirm;
use quick_xml::se::Serializer;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::ops::Add;
use std::path::{Path, PathBuf};

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

pub fn handle(command: SubCommand) -> Result<(), Box<dyn Error>> {
    match command {
        SubCommand::Completion { shell } => {
            let command = &mut Cli::command();
            generate(
                shell,
                command,
                command.get_name().to_string(),
                &mut std::io::stdout(),
            );
        }
        SubCommand::List {
            inputfile,
            sorted,
            filter,
        } => match InputFile::read(inputfile, None)? {
            osc @ InputFile::Osc { .. } => {
                let mut content: OnkostarEditor = osc.try_into()?;
                if sorted {
                    content.sorted();
                }
                if let Some(name) = filter {
                    OnkostarEditor::print_list_filtered(&mut content, name.as_str());
                    return Ok(());
                }
                content.print_list();
            }
            InputFile::Osb { content, .. } => {
                for file in content {
                    match file {
                        InputFile::Osc { .. } => {
                            println!(
                                "{}{}",
                                style("OSB-Paketinhalt: ").bold().yellow(),
                                style(file.filename()).bold()
                            );

                            let mut content: OnkostarEditor = match file.try_into() {
                                Ok(oe) => oe,
                                Err(err) => {
                                    println!("{err}");
                                    continue;
                                }
                            };

                            if sorted {
                                content.sorted();
                            }
                            if let Some(name) = filter {
                                OnkostarEditor::print_list_filtered(&mut content, name.as_str());
                                return Ok(());
                            }
                            content.print_list();
                            println!();
                        }
                        _ => {
                            println!(
                                "{}{}{}",
                                style("OSB-Paketinhalt: ").bold().yellow(),
                                style(file.filename()).bold(),
                                style(" ignoriert").yellow()
                            );
                        }
                    }
                }
            }
            InputFile::Yaml { filename, .. } | InputFile::Other { filename, .. } => {
                return Err(Box::new(FileError::Reading(
                    filename,
                    "Nur OSB- und OSC-Dateien werden unterstützt".to_string(),
                )))
            }
        },
        SubCommand::Tree {
            inputfile,
            sorted,
            filter,
        } => match InputFile::read(inputfile, None)? {
            osc @ InputFile::Osc { .. } => {
                let mut content: OnkostarEditor = osc.try_into()?;
                if sorted {
                    content.sorted();
                }
                if let Some(name) = filter {
                    OnkostarEditor::print_tree_filtered(&mut content, name.as_str());
                    return Ok(());
                }
                OnkostarEditor::print_tree(&content);
            }
            InputFile::Osb { filename, .. } => return Err(Box::new(FileError::Reading(
                filename,
                "Nur OSC-Dateien werden unterstützt. OSB-Dateien erzeugen eine zu lange Ausgabe."
                    .to_string(),
            ))),
            InputFile::Yaml { filename, .. } | InputFile::Other { filename, .. } => {
                return Err(Box::new(FileError::Reading(
                    filename,
                    "Nur OSC-Dateien werden unterstützt".to_string(),
                )))
            }
        },
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
            let mut data: OnkostarEditor = InputFile::read(inputfile, None)?.try_into()?;

            if let Some(profile) = profile {
                let profile = if profile.contains('.') {
                    FileReader::<Profile>::read(&profile)?
                } else {
                    Profile::embedded_profile(&profile)?
                };

                data.apply_profile(&profile);
            }

            let mut compact = compact;
            let mut sorted = sorted;
            let mut strip = strip;

            if interactive {
                compact = matches!(
                    Confirm::new()
                        .with_prompt("Kompakte Ausgabe, ohne Einrücken?")
                        .default(compact)
                        .interact(),
                    Ok(true)
                );

                sorted = matches!(
                    Confirm::new()
                        .with_prompt(
                            "Sortiere Kataloge und Formulare nach Name und Abhängigkeiten?"
                        )
                        .default(sorted)
                        .interact(),
                    Ok(true)
                );

                strip = matches!(
                    Confirm::new()
                        .with_prompt(
                            "Entferne Einträge aus der Systembibliothek die nicht importiert werden?",
                        )
                        .default(strip)
                        .interact(),
                    Ok(true)
                );
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

            data.serialize(serializer).map_err(|_| {
                FileError::Writing("Cannot serialize result".to_string(), String::new())
            })?;

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
                    println!("{output}");
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

            let data_a = &mut FileReader::<OnkostarEditor>::read(&inputfile_a)?;
            let data_b = &mut FileReader::<OnkostarEditor>::read(&inputfile_b)?;

            data_a.print_diff(data_b, strict);
        }
        SubCommand::Sha256Sum { inputfile } => match fs::read(inputfile.clone()) {
            Ok(content) => {
                let mut hasher = Sha256::new();
                hasher.update(content.as_slice());
                let hash = hasher.finalize();
                println!(
                    "{}  {}",
                    base16ct::lower::encode_string(&hash),
                    PathBuf::from(inputfile.clone())
                        .canonicalize()
                        .unwrap_or_default()
                        .to_str()
                        .unwrap_or_default()
                );
            }
            Err(err) => {
                eprintln!("{}", FileError::Reading(inputfile, err.to_string()));
            }
        },
        SubCommand::Check {
            file,
            list,
            password,
        } => {
            if list {
                print();
            } else {
                match check_file(Path::new(file.unwrap_or_default().as_str()), &password) {
                    Ok(notices) => {
                        println!(
                            "Es wurden {} Probleme gefunden\n",
                            notices
                                .iter()
                                .filter(|notice| matches!(
                                    notice,
                                    CheckNotice::ErrorWithCode { .. } | CheckNotice::Error { .. }
                                ))
                                .count()
                        );
                        notices
                            .iter()
                            .for_each(|check_notice| println!("{check_notice}"));
                    }
                    Err(err) => {
                        println!("{err}");
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
    }

    Ok(())
}
