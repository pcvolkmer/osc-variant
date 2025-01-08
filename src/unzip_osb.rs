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

use console::style;
use deob::deobfuscate;
use std::path::Path;
use std::{fs, io};

macro_rules! started {
    ( $o:expr ) => {
        println!("{: <6}{}", style("[..]").cyan(), $o);
    };
}

macro_rules! ok {
    ( $o:expr ) => {
        use console::Term;
        let _ = Term::stdout().move_cursor_up(1);
        println!("{: <6}{}", style("[OK]").green(), $o);
    };
}

macro_rules! error {
    ( $o:expr, $e:expr ) => {
        use console::Term;
        let _ = Term::stdout().move_cursor_up(1);
        println!("{: <6}{} - Error: {}", style("[ERR]").red(), $o, $e);
    };
}

pub fn unzip_osb(path: &str, dir: &str, password: Option<String>) {
    let password = password.unwrap_or_else(|| deobfuscate(env!("OSB_KEY").trim()));

    println!("Entpacke OSB-Datei {}\n", style(path).yellow());

    let file = match fs::File::open(path) {
        Ok(file) => file,
        Err(err) => {
            println!(
                "{: <6}Abbruch! - Kann Datei nicht entpacken: {}",
                style("[ERR]").red(),
                err
            );
            return;
        }
    };

    let mut archive = match zip::ZipArchive::new(file) {
        Ok(file) => file,
        Err(err) => {
            println!(
                "{: <6}Abbruch! - Kann Datei nicht entpacken: {}",
                style("[ERR]").red(),
                err
            );
            return;
        }
    };

    for i in 0..archive.len() {
        let Ok(mut file) = archive.by_index_decrypt(i, password.as_bytes()) else {
            println!(
                "{: <6}Abbruch! - Kann Datei nicht entpacken",
                style("[ERR]").red()
            );
            return;
        };

        let outpath = match file.enclosed_name() {
            Some(path) => Path::new(dir).join(&path),
            None => continue,
        };

        started!(outpath.display());

        if file.is_dir() {
            if !outpath.exists() {
                match fs::create_dir_all(&outpath) {
                    Ok(()) => {}
                    Err(err) => {
                        error!(outpath.display(), err);
                        continue;
                    }
                }
            }
            ok!(outpath.display());
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    match fs::create_dir_all(p) {
                        Ok(()) => {}
                        Err(err) => {
                            error!(outpath.display(), err);
                            continue;
                        }
                    }
                }
            }
            let mut outfile = match fs::File::create(&outpath) {
                Ok(file) => file,
                Err(err) => {
                    error!(outpath.display(), err);
                    continue;
                }
            };
            match io::copy(&mut file, &mut outfile) {
                Ok(_) => {}
                Err(err) => {
                    error!(outpath.display(), err);
                    continue;
                }
            }
            ok!(outpath.display());
        }
    }
}
