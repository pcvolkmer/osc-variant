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

use std::cmp::Ordering;
use std::fmt::Debug;
use std::str::FromStr;

use console::style;
use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};

use crate::model::data_catalogue::DataCatalogue;
use crate::model::data_form::DataForm;
use crate::model::property_catalogue::PropertyCatalogue;
use crate::model::requirements::Requires;
use crate::model::unterformular::Unterformular;
use crate::model::{Comparable, FolderContent, FormEntryContainer, Listable, Sortable};
use crate::profile::Profile;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct OnkostarEditor {
    #[serde(rename = "InfoXML")]
    info_xml: InfoXML,
    #[serde(rename = "Editor")]
    editor: Editor,
}

impl OnkostarEditor {
    pub fn find_property_catalogue<'a>(&'a self, name: &str) -> Option<&'a PropertyCatalogue> {
        match self
            .editor
            .property_catalogue
            .iter()
            .filter(|&item| item.get_name().eq_ignore_ascii_case(name))
            .nth(0)
        {
            Some(x) => Some(x),
            _ => None,
        }
    }

    pub fn find_data_catalogue<'a>(&'a self, name: &str) -> Option<&'a DataCatalogue> {
        match self
            .editor
            .data_catalogue
            .iter()
            .filter(|&item| item.get_name().eq_ignore_ascii_case(name))
            .nth(0)
        {
            Some(x) => Some(x),
            _ => None,
        }
    }

    pub fn apply_profile(&mut self, profile: &Profile) {
        self.editor
            .data_form
            .iter_mut()
            .filter(|data_form| !data_form.is_system_library_content())
            .for_each(|data_form| {
                data_form.apply_profile(profile);
            });
        self.editor
            .unterformular
            .iter_mut()
            .filter(|data_form| !data_form.is_system_library_content())
            .for_each(|data_form| {
                data_form.apply_profile(profile);
            });
    }

    pub fn print_list(&self) {
        println!(
            "Die Datei wurde am {} mit {} in Version {} erstellt.\n\nFolgende Inhalte sind gespeichert",
            style(&self.info_xml.datum_xml).yellow(),
            style(&self.info_xml.name).yellow(),
            style(&self.info_xml.version).yellow()
        );
        Self::print_items("Merkmalskataloge", &self.editor.property_catalogue);
        Self::print_items("Datenkataloge", &self.editor.data_catalogue);
        Self::print_items("Formulare", &self.editor.data_form);
        Self::print_items("Unterformulare", &self.editor.unterformular);
    }

    fn print_items(title: &str, list: &[impl Listable]) {
        print!("\n{} {}", list.len(), style(title).underlined());
        println!(
            " - Inhalte der Systembibliothek sind mit ({}), der Benutzerbibliothek mit (u) markiert",
            style("S").yellow()
        );
        list.iter()
            .for_each(|entry| println!("{}", entry.to_listed_string()));
    }

    pub fn print_tree(&self) {
        println!(
            "Die Datei wurde am {} mit {} in Version {} erstellt.\n\nFolgende Inhalte sind gespeichert",
            style(&self.info_xml.datum_xml).yellow(),
            style(&self.info_xml.name).yellow(),
            style(&self.info_xml.version).yellow()
        );

        Self::print_items("Merkmalskataloge", &self.editor.property_catalogue);
        self.print_items_tree("Datenkataloge", &self.editor.data_catalogue);
        self.print_items_tree("Formulare", &self.editor.data_form);
        self.print_items_tree("Unterformulare", &self.editor.unterformular);
    }

    fn print_items_tree(&self, title: &str, list: &[impl Requires]) {
        print!("\n{} {}", list.len(), style(title).underlined());
        println!(
            " - Inhalte der Systembibliothek sind mit ({}), der Benutzerbibliothek mit (u) markiert",
            style("S").yellow()
        );
        list.iter()
            .for_each(|entry| println!("{}", entry.to_requirement_string(self)));
    }

    pub fn sorted(&mut self) {
        self.editor
            .property_catalogue
            .sort_unstable_by_key(|e| e.sorting_key());

        self.editor.property_catalogue.iter_mut().for_each(|item| {
            item.sorted();
        });

        self.editor
            .data_catalogue
            .sort_unstable_by_key(|e| e.sorting_key());

        self.editor.data_catalogue.iter_mut().for_each(|item| {
            item.sorted();
        });

        self.editor
            .data_form
            .sort_unstable_by_key(|e| e.sorting_key());

        self.editor.data_form.iter_mut().for_each(|item| {
            item.sorted();
        });

        self.editor
            .unterformular
            .sort_unstable_by_key(|e| e.sorting_key());

        self.editor.unterformular.iter_mut().for_each(|item| {
            item.sorted();
        });
    }

    pub fn strip_system_library_content(&mut self) {
        self.editor
            .property_catalogue
            .retain(|e| !e.is_system_library_content());

        self.editor
            .data_catalogue
            .retain(|e| !e.is_system_library_content());

        self.editor
            .data_form
            .retain(|e| !e.is_system_library_content());

        self.editor
            .unterformular
            .retain(|e| !e.is_system_library_content());
    }

    pub fn print_diff(&mut self, other: &mut Self, strict: bool) {
        println!();

        println!(
            "Datei A wurde am {} mit {} in Version {} erstellt.",
            style(&self.info_xml.datum_xml).yellow(),
            style(&self.info_xml.name).yellow(),
            style(&self.info_xml.version).yellow()
        );

        println!(
            "Datei B wurde am {} mit {} in Version {} erstellt.",
            style(&other.info_xml.datum_xml).yellow(),
            style(&other.info_xml.name).yellow(),
            style(&other.info_xml.version).yellow()
        );

        self.sorted();
        other.sorted();

        Self::print_item_diff(
            "Merkmalskataloge",
            &self.editor.property_catalogue,
            &other.editor.property_catalogue,
            strict,
        );
        Self::print_item_diff(
            "Datenkataloge",
            &self.editor.data_catalogue,
            &other.editor.data_catalogue,
            strict,
        );
        Self::print_item_diff(
            "Formulare",
            &self.editor.data_form,
            &other.editor.data_form,
            strict,
        );
        Self::print_item_diff(
            "Unterformulare",
            &self.editor.unterformular,
            &other.editor.unterformular,
            strict,
        );
    }

    fn print_item_diff(
        title: &str,
        list_a: &[impl Comparable],
        list_b: &[impl Comparable],
        strict: bool,
    ) {
        println!("\n{}", style(title).underlined());

        let mut has_diff = false;

        let names_a = list_a
            .iter()
            .map(|entry| entry.get_name())
            .collect::<Vec<_>>();
        let names_b = list_b
            .iter()
            .map(|entry| entry.get_name())
            .collect::<Vec<_>>();

        names_b.iter().for_each(|entry| {
            if !names_a.contains(entry) {
                println!("{}: {}", entry, style("Nicht in Datei A enthalten!").red());
                has_diff = true;
            }
        });

        names_a.iter().for_each(|entry| {
            if !names_b.contains(entry) {
                println!("{}: {}", entry, style("Nicht in Datei B enthalten!").red());
                has_diff = true;
            }
        });

        list_a.iter().for_each(|entry_a| {
            list_b.iter().for_each(|entry_b| {
                if entry_a.get_name() == entry_b.get_name() {
                    match entry_a.get_revision().cmp(&entry_b.get_revision()) {
                        Ordering::Less => {
                            println!(
                                "{}: {} (Revision {} < Revision {})",
                                entry_a.get_name(),
                                style("Neuer in Datei B").yellow(),
                                style(entry_a.get_revision()).blue(),
                                style(entry_b.get_revision()).green()
                            );
                            has_diff = true;
                        }
                        Ordering::Greater => {
                            println!(
                                "{}: {} (Revision {} > Revision {})",
                                entry_a.get_name(),
                                style("Neuer in Datei A").yellow(),
                                style(entry_a.get_revision()).green(),
                                style(entry_b.get_revision()).blue()
                            );
                            has_diff = true;
                        }
                        _ => {
                            if strict && entry_a.get_hash() != entry_b.get_hash() {
                                println!(
                                    "{}: {} (z.B. GUID oder Reihenfolge von Unterelementen)",
                                    entry_a.get_name(),
                                    style("Inhaltlich verschieden").yellow()
                                );
                                has_diff = true;
                            } else if strict {
                                println!("{}: {}", entry_a.get_name(), style("Identisch").green())
                            }
                        }
                    }
                }
            });
        });

        if !has_diff {
            println!("Keine Unterschiede")
        }
    }
}

impl FromStr for OnkostarEditor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match from_str::<OnkostarEditor>(s) {
            Ok(profile) => Ok(profile),
            Err(err) => Err(err.to_string()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct InfoXML {
    #[serde(rename = "DatumXML")]
    datum_xml: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Version")]
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct Editor {
    #[serde(rename = "PropertyCatalogue")]
    property_catalogue: Vec<PropertyCatalogue>,
    #[serde(rename = "DataCatalogue")]
    data_catalogue: Vec<DataCatalogue>,
    #[serde(rename = "Unterformular")]
    unterformular: Vec<Unterformular>,
    #[serde(rename = "DataForm")]
    data_form: Vec<DataForm>,
}
