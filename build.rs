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

use clap::CommandFactory;
use clap_complete::generate_to;
use clap_complete::Shell::Bash;
use std::fs;
use std::io::Error;

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    let mut cmd = Cli::command();

    let package_name = std::env::var("CARGO_CRATE_NAME").unwrap_or("osc-variant".to_string());

    fs::remove_dir_all("completion").unwrap_or_default();
    fs::create_dir("completion")?;

    generate_to(Bash, &mut cmd, package_name.as_str(), "completion")?;

    Ok(())
}
