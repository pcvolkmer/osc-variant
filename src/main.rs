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

use crate::cli::Cli;
use crate::commands::handle;
use clap::Parser;
use std::error::Error;

mod checks;
mod cli;
mod commands;
mod file_io;
mod model;
mod profile;
#[cfg(feature = "unzip-osb")]
mod unzip_osb;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    handle(cli.cmd)?;
    Ok(())
}
