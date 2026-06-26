/*
 * This file is part of osc-variant
 *
 * Copyright (C) 2026 the original author or authors.
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
use crate::model::form::{Form, Notice};
use crate::model::{Named, TypedEntry};

pub(crate) trait WithNotice {
    fn get_notices(&self) -> Vec<Notice>;
}

impl<Type> WithNotice for Form<Type> {
    fn get_notices(&self) -> Vec<Notice> {
        if let Some(entries) = &self.entries {
            entries
                .entry
                .iter()
                .filter(|entry| {
                    !entry.is_subform()
                        && !entry.is_section()
                        && !entry.is_label()
                        && if let Some(filter) = &entry.filter {
                            filter.condition.trim() != "false"
                        } else {
                            true
                        }
                })
                .flat_map(|entry| {
                    Some(Notice {
                        form: self.get_name(),
                        form_field: entry.name.clone(),
                        form_field_description: entry.description.clone(),
                        guid: entry.guid.clone(),
                        html: entry.hinweis.clone().unwrap_or_default(),
                        position: entry.position.clone(),
                    })
                })
                .collect()
        } else {
            vec![]
        }
    }
}
