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

use std::ffi::{c_char, CStr, CString};

#[link(name = "deob")]
extern "C" {
    fn deob(key: *const c_char);
}

pub fn deobfuscate(s: &str) -> String {
    let key = CString::new(s).unwrap_or_default().into_raw();
    unsafe {
        deob(key);
        String::from_utf8_lossy(CStr::from_ptr(key).to_bytes()).to_string()
    }
}
