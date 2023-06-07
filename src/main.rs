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
use std::str::FromStr;

use clap::Parser;
use quick_xml::se::Serializer;
use serde::Serialize;

use crate::cli::{Cli, Command};
use crate::model::onkostar_editor::OnkostarEditor;
use crate::profile::Profile;

mod cli;
mod model;
mod profile;

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
    let mut file = OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename.clone())
        .map_err(|err| FileError::Writing(filename.clone(), err.to_string()))?;
    file.write_all(content.as_bytes())
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

    match cli.command {
        Command::List { inputfile } => {
            let data = read_inputfile(inputfile)?;
            data.list_forms();
        }
        Command::Modify {
            inputfile,
            profile,
            outputfile,
            compact,
        } => {
            let data = &mut read_inputfile(inputfile)?;

            if let Some(profile) = profile {
                let profile = read_profile(profile.clone()).map_err(|_| {
                    FileError::Reading(profile, "Kann Profildatei nicht lesen!".into())
                })?;
                data.apply_profile(&profile);
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
    };

    Ok(())
}
