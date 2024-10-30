/*
 * This file is part of osc-variant
 *
 * Copyright (C) 2024 the original author or authors.
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

use std::cmp::Ordering;
use std::fmt::Debug;
use std::str::FromStr;

use console::style;
use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};

use crate::checks::{CheckNotice, Checkable};
use crate::model::data_catalogue::DataCatalogue;
use crate::model::data_form::DataForm;
use crate::model::other::{Ablaufschema, Akte, RecordLinkage, Rskript, SidGuid};
use crate::model::property_catalogue::PropertyCatalogue;
use crate::model::requirements::{Requirement, Requires};
use crate::model::unterformular::Unterformular;
use crate::model::{Comparable, FolderContent, FormEntryContainer, Listable, Sortable};
use crate::profile::Profile;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct OnkostarEditor {
    #[serde(rename = "InfoXML")]
    info_xml: InfoXML,
    #[serde(rename = "Editor")]
    pub editor: Editor,
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

    pub fn find_data_form<'a>(&'a self, name: &str) -> Option<&'a DataForm> {
        match self
            .editor
            .data_form
            .iter()
            .filter(|&item| item.get_name().eq_ignore_ascii_case(name))
            .nth(0)
        {
            Some(x) => Some(x),
            _ => None,
        }
    }

    pub fn find_unterformular<'a>(&'a self, name: &str) -> Option<&'a Unterformular> {
        match self
            .editor
            .unterformular
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

    fn filter_by_name_contains(&mut self, name: &str) {
        self.editor
            .property_catalogue
            .retain(|e| e.get_name().contains(name));
        self.editor
            .data_catalogue
            .retain(|e| e.get_name().contains(name));
        self.editor
            .data_form
            .retain(|e| e.get_name().contains(name));
        self.editor
            .unterformular
            .retain(|e| e.get_name().contains(name));
    }
    pub fn print_list_filtered(&mut self, name: &str) {
        println!(
            "Die Datei wurde am {} mit {} in Version {} erstellt.\n\nFolgende Inhalte für '{}' sind gespeichert",
            style(&self.info_xml.datum_xml).yellow(),
            style(&self.info_xml.name).yellow(),
            style(&self.info_xml.version).yellow(),
            name
        );

        self.filter_by_name_contains(name);

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

    pub fn print_tree_filtered(&mut self, name: &str) {
        println!(
            "Die Datei wurde am {} mit {} in Version {} erstellt.\n\nFolgende Inhalte für '{}' sind gespeichert",
            style(&self.info_xml.datum_xml).yellow(),
            style(&self.info_xml.name).yellow(),
            style(&self.info_xml.version).yellow(),
            name
        );

        self.filter_by_name_contains(name);

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

        /*self.editor
        .data_form
        .sort_unstable_by_key(|e| e.sorting_key());*/

        self.editor
            .data_form
            .sort_unstable_by(DataForm::compare_by_requirement);

        self.editor.data_form.iter_mut().for_each(|item| {
            item.sorted();
        });

        /*self.editor
        .unterformular
        .sort_unstable_by_key(|e| e.sorting_key());*/

        self.editor
            .unterformular
            .sort_unstable_by(Unterformular::compare_by_requirement);

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
        if !s
            .matches("<AnalyseBereichEditor>")
            .collect::<String>()
            .is_empty()
        {
            return Err(
                "Datei mit Inhalt 'AnalyseBereichEditor' kann nicht verwendet werden".into(),
            );
        }

        match from_str::<OnkostarEditor>(s) {
            Ok(profile) => Ok(profile),
            Err(err) => Err(err.to_string()),
        }
    }
}

impl Checkable for OnkostarEditor {
    fn check(&self) -> Vec<CheckNotice> {
        // Inner form checks

        let mut result = self
            .editor
            .data_form
            .iter()
            .flat_map(|entity| entity.check())
            .collect::<Vec<_>>();

        let other = &mut self
            .editor
            .unterformular
            .iter()
            .flat_map(|entity| entity.check())
            .collect::<Vec<_>>();

        result.append(other);

        // Check requirements

        let mut requirement_checked_forms = vec![];

        fn requirement_error(
            form: &impl Comparable,
            item: &impl Comparable,
            t: &str,
        ) -> CheckNotice {
            CheckNotice::ErrorWithCode {
                code: "2023-0004".to_string(),
                description: format!(
                    "'{}' hat einen Verweis auf zuvor nicht definiertes {t} '{}' (OSTARSUPP-13212)",
                    form.get_name(),
                    item.get_name()
                ),
                line: None,
                example: None,
            }
        }

        self.editor.unterformular.iter().for_each(|form| {
            requirement_checked_forms.push(form.get_name());
            form.get_required_entries(self)
                .iter()
                .for_each(|entry| match entry {
                    Requirement::DataFormReference(item) => {
                        if !requirement_checked_forms.contains(&item.get_name()) {
                            result.push(requirement_error(form, *item, "Formular"))
                        }
                    }
                    Requirement::UnterformularReference(item) => {
                        if !requirement_checked_forms.contains(&item.get_name()) {
                            result.push(requirement_error(form, *item, "Unterformular"))
                        }
                    }
                    _ => {}
                });
        });

        self.editor.data_form.iter().for_each(|form| {
            requirement_checked_forms.push(form.get_name());
            form.get_required_entries(self)
                .iter()
                .for_each(|entry| match entry {
                    Requirement::DataFormReference(item) => {
                        if !requirement_checked_forms.contains(&item.get_name()) {
                            result.push(requirement_error(form, *item, "Formular"))
                        }
                    }
                    Requirement::UnterformularReference(item) => {
                        if !requirement_checked_forms.contains(&item.get_name()) {
                            result.push(requirement_error(form, *item, "Unterformular"))
                        }
                    }
                    _ => {}
                });
        });

        result
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
pub struct Editor {
    #[serde(rename = "PropertyCatalogue", default)]
    property_catalogue: Vec<PropertyCatalogue>,
    #[serde(rename = "DataCatalogue", default)]
    data_catalogue: Vec<DataCatalogue>,
    #[serde(rename = "Unterformular", default)]
    pub unterformular: Vec<Unterformular>,
    #[serde(rename = "DataForm", default)]
    pub data_form: Vec<DataForm>,

    #[serde(rename = "Ablaufschema", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ablaufschema: Option<Vec<Ablaufschema>>,
    #[serde(rename = "Akte", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub akte: Option<Vec<Akte>>,
    #[serde(rename = "RecordLinkage", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_linkage: Option<Vec<RecordLinkage>>,
    #[serde(rename = "Rskript", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rskript: Option<Vec<Rskript>>,
    #[serde(rename = "FormulareLoeschen", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formulare_loeschen: Option<Vec<SidGuid>>,
    #[serde(rename = "FormulareDeaktivieren", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formulare_deaktivieren: Option<Vec<SidGuid>>,
}
