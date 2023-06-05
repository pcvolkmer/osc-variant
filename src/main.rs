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

use clap::Parser;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::ops::Add;
use std::str::FromStr;

use crate::cli::{Cli, Command};
use quick_xml::de::from_str;
use quick_xml::se::Serializer;
use serde::Serialize;

use crate::model::onkostar_editor::OnkostarEditor;
use crate::profile::Profile;

mod cli;
mod model;
mod profile;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::List { inputfile } => {
            let contents =
                fs::read_to_string(inputfile).expect("Should have been able to read the file");

            match OnkostarEditor::from_str(contents.as_str()) {
                Ok(data) => data.list_forms(),
                Err(err) => {
                    eprintln!("Kann Eingabedatei nicht lesen!");
                    eprintln!(
                        "Die Datei ist entweder keine OSC-Datei, fehlerhaft oder enthält zusätzliche Inhalte:\n{}",
                        err
                    );
                }
            }
        }
        Command::Modify {
            inputfile,
            profile,
            outputfile,
        } => {
            let contents =
                fs::read_to_string(inputfile).expect("Should have been able to read the file");

            if let Ok(mut data) = from_str::<OnkostarEditor>(contents.as_str()) {
                if let Some(profile) = profile {
                    let profile =
                        fs::read_to_string(profile).expect("Should have been able to read profile");
                    if let Ok(profile) = Profile::from_str(profile.as_str()) {
                        data.apply_profile(&profile);
                    } else {
                        eprintln!("Kann Profildatei nicht lesen!");
                        return;
                    }
                }

                let mut buf = String::new();

                let mut serializer = Serializer::new(&mut buf);
                serializer.indent(' ', 2);

                data.serialize(serializer).expect("Generated XML");

                let output = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n"
                    .to_string()
                    .add(
                        buf
                            // Replace &apos; and &quot; as used in original file
                            .replace("&apos;", "'")
                            .replace("&quot;", "\"")
                            .as_str(),
                    );

                match outputfile {
                    Some(filename) => {
                        let mut file = OpenOptions::new()
                            .read(false)
                            .write(true)
                            .create(true)
                            .truncate(true)
                            .open(filename)
                            .unwrap();
                        file.write_all(output.as_bytes())
                            .expect("Should have written output file");
                    }
                    None => {
                        println!("{}", output)
                    }
                }
            } else {
                eprintln!("Kann Eingabedatei nicht lesen!");
                eprintln!(
                    "Die Datei ist entweder keine OSC-Datei, fehlerhaft oder enthält zusätzliche Inhalte."
                );
            }
        }
    }
}
