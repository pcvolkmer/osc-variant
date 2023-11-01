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

use console::style;
use deob::deobfuscate;
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

pub fn unzip_osb_using_password(path: &str, password: &str) {
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
        let mut file = if let Ok(Ok(file)) = archive.by_index_decrypt(i, password.as_bytes()) {
            file
        } else {
            println!(
                "{: <6}Abbruch! - Kann Datei nicht entpacken",
                style("[ERR]").red()
            );
            return;
        };

        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        started!(outpath.display());

        if !file.is_dir() {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
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
        } else {
            if !outpath.exists() {
                match fs::create_dir_all(&outpath) {
                    Ok(_) => {}
                    Err(err) => {
                        error!(outpath.display(), err);
                        continue;
                    }
                }
            }
            ok!(outpath.display());
        }
    }
}

pub fn unzip_osb(path: &str) {
    unzip_osb_using_password(path, deobfuscate(env!("OSB_KEY").trim()).as_str());
}
