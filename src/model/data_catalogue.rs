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

use std::collections::HashSet;

use console::style;
use serde::{Deserialize, Serialize};

use crate::model::onkostar_editor::OnkostarEditor;
use crate::model::requirements::{Requirement, Requires};
use crate::model::{Ansichten, Comparable, FolderContent, Listable, Ordner, Sortable};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct DataCatalogue {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "NameExport")]
    name_export: String,
    #[serde(rename = "Category")]
    category: String,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "Note")]
    note: String,
    #[serde(rename = "Readonly")]
    readonly: bool,
    #[serde(rename = "BestOf")]
    best_of: bool,
    #[serde(rename = "SID")]
    sid: String,
    #[serde(rename = "GUID")]
    guid: String,
    #[serde(rename = "Revision")]
    revision: u16,
    #[serde(rename = "Entries")]
    entries: Entries,
    #[serde(rename = "Ordner")]
    ordner: Ordner,
    #[serde(rename = "Ansichten", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    ansichten: Option<Ansichten>,
}

impl Listable for DataCatalogue {
    fn to_listed_string(&self) -> String {
        format!(
            "Datenkatalog ({}) '{}' in Revision '{}'",
            match self.is_system_library_content() {
                true => style("S").yellow(),
                _ => style("u"),
            },
            style(&self.name).yellow(),
            style(&self.revision).yellow()
        )
    }
}

impl Sortable for DataCatalogue {
    fn sorting_key(&self) -> String {
        self.name.clone()
    }

    fn sorted(&mut self) -> &Self {
        self.entries
            .entry
            .sort_unstable_by_key(|item| item.sorting_key());
        self.entries.entry.iter_mut().for_each(|item| {
            item.sorted();
        });
        self
    }
}

impl Comparable for DataCatalogue {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_revision(&self) -> u16 {
        self.revision
    }
}

impl Requires for DataCatalogue {
    fn get_required_entries<'a>(&'a self, all: &'a OnkostarEditor) -> Vec<Requirement<'a>> {
        let mut result = self
            .entries
            .entry
            .iter()
            .filter(|&entry| entry.property_catalogue.is_some())
            .map(|entry| match &entry.property_catalogue {
                Some(entry) => entry.to_string(),
                _ => String::new(),
            })
            .collect::<HashSet<_>>()
            .into_iter()
            .map(|entry| match all.find_property_catalogue(entry.as_str()) {
                Some(contained) => Requirement::PropertyCatalogue(contained),
                None => Requirement::ExternalPropertyCatalogue(entry),
            })
            .collect::<Vec<_>>();
        result.sort_unstable_by_key(|item| item.sorting_key());

        result
    }

    fn to_requirement_string<'a>(&'a self, all: &'a OnkostarEditor) -> String {
        format!(
            "{}\n{}",
            self.to_listed_string(),
            self.get_required_entries(all)
                .iter()
                .map(|entry| match entry {
                    Requirement::PropertyCatalogue(_) => {
                        Some(format!("  - {}\n", entry))
                    }
                    Requirement::ExternalPropertyCatalogue(_) => {
                        Some(format!("  - {}\n", entry))
                    }
                    _ => None,
                })
                .filter(Option::is_some)
                .flatten()
                .collect::<Vec<_>>()
                .join("")
        )
    }
}

impl FolderContent for DataCatalogue {
    fn get_library_folder(&self) -> String {
        self.ordner.bibliothek.name.to_string()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Entries {
    #[serde(rename = "Entry")]
    entry: Vec<Entry>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Entry {
    #[serde(rename = "PropertyCatalogue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    property_catalogue: Option<String>,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "NameExport")]
    name_export: String,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "Type")]
    entry_type: String,
    #[serde(rename = "SubTable")]
    sub_table: String,
    #[serde(rename = "ScaleUnit")]
    scale_unit: String,
    #[serde(rename = "MaxLength")]
    max_length: u32,
    #[serde(rename = "DefaultValue")]
    default_value: String,
    #[serde(rename = "Active")]
    active: bool,
    #[serde(rename = "Readonly")]
    read_only: bool,
    #[serde(rename = "Filterable")]
    filterable: bool,
    #[serde(rename = "RangeFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    range_from: Option<String>,
    #[serde(rename = "RangeUntil")]
    #[serde(skip_serializing_if = "Option::is_none")]
    range_until: Option<String>,
    #[serde(rename = "MultipleChoice")]
    multiple_choice: bool,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "Note")]
    note: String,
    #[serde(rename = "DateEstimatedAllowed")]
    date_estimated_allowed: bool,
    #[serde(rename = "DateUnknownAllowed")]
    date_unknown_allowed: bool,
    #[serde(rename = "oeChoiceOptions")]
    oe_choice_options: String,
    #[serde(rename = "Fachabteilungsbezug")]
    fachabteilungsbezug: bool,
    #[serde(rename = "Use")]
    #[serde(skip_serializing_if = "Option::is_none")]
    use_: Option<Use>,
    #[serde(rename = "FesteNachkommastellen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    feste_nachkommastellen: Option<u16>,
    #[serde(rename = "SID")]
    sid: String,
    #[serde(rename = "GUID")]
    guid: String,
    #[serde(rename = "Revision")]
    revision: u16,
}

impl Sortable for Entry {
    fn sorting_key(&self) -> String {
        self.name.clone()
    }

    fn sorted(&mut self) -> &Self
    where
        Self: Sized,
    {
        if let Some(ref mut use_) = self.use_ {
            use_.program_module
                .sort_unstable_by_key(|item| item.sorting_key())
        }
        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Use {
    #[serde(rename = "ProgramModule", default)]
    program_module: Vec<ProgramModule>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ProgramModule {
    #[serde(rename = "@program")]
    program: String,
    #[serde(rename = "@name")]
    name: String,
}

impl Sortable for ProgramModule {
    fn sorting_key(&self) -> String {
        format!("{}-{}", self.program, self.name)
    }
}
