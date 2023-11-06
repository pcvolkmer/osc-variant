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

pub mod osc;

use console::style;
use std::fmt::{Display, Formatter};

pub enum CheckNotice {
    /// This will result in Error if importing file and has a support code
    ErrorWithCode {
        code: String,
        description: String,
        line: Option<usize>,
        example: Option<String>,
    },
    /// This will result in Error if importing file
    Error {
        description: String,
        line: Option<usize>,
    },
    #[allow(dead_code)]
    /// Other known issues
    Warning {
        description: String,
        line: Option<usize>,
    },
}

impl Display for CheckNotice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckNotice::ErrorWithCode {
                code,
                description,
                line,
                example,
            } => match line {
                Some(line) => write!(
                    f,
                    "{} ({}) at Line {}: {}{}",
                    style("ERROR").red().bold(),
                    code,
                    line,
                    description,
                    match example {
                        Some(example) => format!(" -> '{}'", style(example).dim()),
                        _ => String::new(),
                    }
                ),
                None => write!(
                    f,
                    "{} ({}): {}{}",
                    style("ERROR").red().bold(),
                    code,
                    description,
                    match example {
                        Some(example) => format!(" -> '{}'", style(example).dim()),
                        _ => String::new(),
                    }
                ),
            },
            CheckNotice::Error { description, line } => match line {
                Some(line) => write!(
                    f,
                    "{} at Line {}: {}",
                    style("ERROR").red().bold(),
                    line,
                    description
                ),
                None => write!(f, "{}: {}", style("ERROR").red().bold(), description),
            },
            CheckNotice::Warning { description, line } => match line {
                Some(line) => write!(
                    f,
                    "{} at Line {}: {}",
                    style("WARNING").yellow().bold(),
                    line,
                    description
                ),
                None => write!(f, "{}: {}", style("WARNING").yellow().bold(), description),
            },
        }
    }
}

pub trait Checkable {
    fn check(&self) -> Vec<CheckNotice>;
}

pub trait Fixable {
    fn fix(&mut self) -> bool;
}

pub fn print_checks() {
    println!(
        "{}",
        style("Die folgenden Probleme sind bekannt\n")
            .yellow()
            .bold()
    );

    struct Problem<'a> {
        code: &'a str,
        name: &'a str,
        description: &'a str,
        fixable: bool,
    }

    impl<'a> Display for Problem<'a> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{} {} {}\n\n{}",
                style(self.code).bold(),
                style(self.name).underlined(),
                match self.fixable {
                    true => style("(Behebbar)").green(),
                    false => style("(Nicht behebbar)").red(),
                },
                self.description
            )
        }
    }

    vec![Problem {
        code: "2023-0001",
        name: "Leerzeichen am Ende der Plausibilitätsregel-Bezeichnung (OSTARSUPP-13334)",
        description: "Treten Leerzeichen am Ende der Plausibilitätsregel-Bezeichnung auf,\n\
        führt dies zu Fehlern beim Import der OSC-Datei.\n\
        \n\
        Das Problem wird beim Verwenden des Unterbefehls 'modify' automatisch\n\
        behoben und Leerzeichen entfernt. 
        ",
        fixable: true,
    }]
    .iter()
    .for_each(|problem| println!("{}\n", problem))
}
