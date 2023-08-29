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

use std::collections::HashSet;

use console::style;
use serde::{Deserialize, Serialize};

use crate::model::onkostar_editor::OnkostarEditor;
use crate::model::requirements::{Requirement, Requires};
use crate::model::{Comparable, Listable, Ordner, Sortable};

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
}

impl Listable for DataCatalogue {
    fn to_listed_string(&self) -> String {
        format!(
            "Datenkatalog '{}' in Revision '{}'",
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
    fn get_required_entries<'a>(&'a self, all: &'a OnkostarEditor) -> Vec<Requirement> {
        self.entries
            .entry
            .iter()
            .filter(|&entry| entry.property_catalogue.is_some())
            .map(|entry| match &entry.property_catalogue {
                Some(entry) => entry.to_string(),
                _ => String::new(),
            })
            .collect::<HashSet<_>>()
            .into_iter()
            .map(|entry| all.find_property_catalogue(entry.as_str()))
            .filter(Option::is_some)
            .map(|entry| Requirement::PropertyCatalogue(entry.unwrap()))
            .collect::<Vec<_>>()
    }

    fn to_requirement_string<'a>(&'a self, all: &'a OnkostarEditor) -> String {
        format!(
            "Datenkatalog '{}' in Revision '{}'\n{}",
            style(&self.name).yellow(),
            style(&self.revision).yellow(),
            self.get_required_entries(all)
                .iter()
                .map(|entry| match entry {
                    Requirement::PropertyCatalogue(x) => {
                        Some(format!("  + {}\n", x.to_listed_string()))
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
