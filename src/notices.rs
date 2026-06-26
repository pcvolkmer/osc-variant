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
use model::osc::form::Form;
use model::osc::{Named, TypedEntry};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Notice {
    #[serde(rename = "Formular")]
    pub(crate) form: String,
    #[serde(rename = "Formularfeldbeschreibung")]
    pub(crate) form_field_description: String,
    #[serde(rename = "Formularfeld")]
    pub(crate) form_field: String,
    #[serde(rename = "GUID")]
    pub(crate) guid: String,
    #[serde(rename = "Hinweis als HTML")]
    pub(crate) html: String,
    #[serde(skip)]
    pub(crate) position: String,
}

pub(crate) trait WithNotice {
    fn get_notices(&self) -> Vec<Notice>;
    fn apply_notices(&mut self, notices: Vec<Notice>);
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
                        form_field: entry.get_name(),
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

    fn apply_notices(&mut self, notices: Vec<Notice>) {
        let mut has_updates = false;

        if let Some(ref mut entries) = self.entries {
            entries.entry.iter_mut().for_each(|entry| {
                for notice in &notices {
                    if entry.guid == notice.guid && !notice.html.trim().is_empty() {
                        match entry.hinweis {
                            Some(ref mut hinweis) => {
                                if hinweis.trim() != notice.html.trim() {
                                    has_updates = true;
                                    entry.revision += 1;
                                    *hinweis = notice.html.trim().to_string();
                                }
                            }
                            None => entry.hinweis = Some(notice.html.trim().to_string()),
                        }

                        entry.hinweis = Some(notice.html.trim().to_string());
                    }
                }
            });
        }

        if has_updates {
            self.revision += 1;
        }
    }
}
