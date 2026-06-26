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
use console::style;
use model::osc::data_catalogue::DataCatalogue;
use model::osc::form::{DataFormType, Form};
use model::osc::onkostar_editor::OnkostarEditor;
use model::osc::property_catalogue::PropertyCatalogue;
use model::osc::requirements::{Requirement, Requires};
use model::osc::{Comparable, FolderContained, Named, Revisioned};
use std::any::TypeId;
use std::cmp::Ordering;

pub trait PrintableItemList
where
    Self: Comparable,
{
    fn to_listed_string(&self) -> String;

    fn to_verbose_listed_string(&self) -> String {
        format!(
            "{} {}",
            self.to_listed_string(),
            style(format!("[{}]", &self.get_hash()[..7])).dim()
        )
    }
}

impl PrintableItemList for DataCatalogue {
    fn to_listed_string(&self) -> String {
        format!(
            "Datenkatalog ({}) '{}' in Revision '{}'",
            if self.is_system_library_content() {
                style("S").yellow()
            } else {
                style("u")
            },
            style(&self.get_name()).yellow(),
            style(&self.get_revision()).yellow()
        )
    }
}

impl<Type: 'static> PrintableItemList for Form<Type>
where
    Form<Type>: Comparable,
{
    fn to_listed_string(&self) -> String {
        format!(
            "{} ({}) '{}' in Revision '{}'",
            if TypeId::of::<Type>() == TypeId::of::<DataFormType>() {
                "Formular"
            } else {
                "Unterformular"
            },
            if self.is_system_library_content() {
                style("S").yellow()
            } else {
                style("u")
            },
            style(&self.get_name()).yellow(),
            style(&self.get_revision()).yellow()
        )
    }
}

impl PrintableItemList for PropertyCatalogue {
    fn to_listed_string(&self) -> String {
        format!(
            "Merkmalskatalog ({}) '{}' in Revision '{}'",
            if self.is_system_library_content() {
                style("S").yellow()
            } else {
                style("u")
            },
            style(&self.get_name()).yellow(),
            style(&self.get_revision()).yellow()
        )
    }
}

pub trait PrintableRequirement {
    fn to_requirement_string<'a>(&'a self, all: &'a OnkostarEditor, verbose: bool) -> String;
}

impl<T> PrintableRequirement for Form<T>
where
    Self: PrintableItemList + Requires,
{
    fn to_requirement_string<'a>(&'a self, all: &'a OnkostarEditor, verbose: bool) -> String {
        format!(
            "{}\n{}",
            if verbose {
                self.to_verbose_listed_string()
            } else {
                self.to_listed_string()
            },
            self.get_required_entries(all)
                .iter()
                .filter_map(|entry| match entry {
                    Requirement::DataCatalogue(x) => {
                        let inner = x
                            .get_required_entries(all)
                            .iter()
                            .map(|inner_entry| match inner_entry {
                                Requirement::PropertyCatalogue(_) => Some(inner_entry.to_string()),
                                Requirement::ExternalPropertyCatalogue(_) => {
                                    Some(inner_entry.to_string())
                                }
                                _ => None,
                            })
                            .filter_map(|item| item.map(|item| format!("    - {item}\n")))
                            .collect::<String>();
                        if inner.is_empty() {
                            Some(format!("  + {}\n", x.to_listed_string()))
                        } else {
                            Some(format!("  + {}\n{}", x.to_listed_string(), inner))
                        }
                    }
                    Requirement::ExternalDataCatalogue(_) => {
                        Some(format!("  + {}\n", entry.to_string()))
                    }
                    Requirement::DataFormReference(_)
                    | Requirement::ExternalDataFormReference(_)
                    | Requirement::UnterformularReference(_)
                    | Requirement::ExternalUnterformularReference(_) => {
                        Some(format!("  > {}\n", entry.to_string()))
                    }
                    Requirement::DataFormSubform(_)
                    | Requirement::ExternalDataFormSubform(_)
                    | Requirement::UnterformularSubform(_)
                    | Requirement::ExternalUnterformularSubform(_) => {
                        Some(format!("  * {}\n", entry.to_string()))
                    }
                    _ => None,
                })
                .collect::<String>()
        )
    }
}

impl PrintableRequirement for DataCatalogue {
    fn to_requirement_string<'a>(&'a self, all: &'a OnkostarEditor, verbose: bool) -> String {
        format!(
            "{}\n{}",
            if verbose {
                self.to_verbose_listed_string()
            } else {
                self.to_listed_string()
            },
            self.get_required_entries(all)
                .iter()
                .filter_map(|entry| match entry {
                    Requirement::PropertyCatalogue(_)
                    | Requirement::ExternalPropertyCatalogue(_) => {
                        Some(format!("  - {}\n", entry.to_string()))
                    }
                    _ => None,
                })
                .collect::<String>()
        )
    }
}

pub trait PrintableList {
    fn print_list(&self, verbose: bool);
    fn print_list_filtered(&mut self, name: &str, verbose: bool);
    fn print_items(title: &str, list: &[impl PrintableItemList], verbose: bool);
}

impl PrintableList for OnkostarEditor {
    fn print_list(&self, verbose: bool) {
        println!(
            "Die Datei wurde am {} mit {} in Version {} erstellt.\n\nFolgende Inhalte sind gespeichert",
            style(&self.info_xml.datum_xml).yellow(),
            style(&self.info_xml.name).yellow(),
            style(&self.info_xml.version).yellow()
        );
        Self::print_items("Merkmalskataloge", &self.editor.property_catalogue, verbose);
        Self::print_items("Datenkataloge", &self.editor.data_catalogue, verbose);
        Self::print_items("Formulare", &self.editor.data_form, verbose);
        Self::print_items("Unterformulare", &self.editor.unterformular, verbose);
    }

    fn print_list_filtered(&mut self, name: &str, verbose: bool) {
        println!(
            "Die Datei wurde am {} mit {} in Version {} erstellt.\n\nFolgende Inhalte für '{}' sind gespeichert",
            style(&self.info_xml.datum_xml).yellow(),
            style(&self.info_xml.name).yellow(),
            style(&self.info_xml.version).yellow(),
            name
        );

        self.filter_by_name_contains(name);

        Self::print_items("Merkmalskataloge", &self.editor.property_catalogue, verbose);
        Self::print_items("Datenkataloge", &self.editor.data_catalogue, verbose);
        Self::print_items("Formulare", &self.editor.data_form, verbose);
        Self::print_items("Unterformulare", &self.editor.unterformular, verbose);
    }

    fn print_items(title: &str, list: &[impl PrintableItemList], verbose: bool) {
        print!("\n{} {}", list.len(), style(title).underlined());
        println!(
            " - Inhalte der Systembibliothek sind mit ({}), der Benutzerbibliothek mit (u) markiert",
            style("S").yellow()
        );
        for entry in list {
            if verbose {
                println!("{}", entry.to_verbose_listed_string());
                continue;
            }
            println!("{}", entry.to_listed_string());
        }
    }
}

pub trait PrintableTree {
    fn print_tree(&self, verbose: bool);
    fn print_tree_filtered(&mut self, name: &str, verbose: bool);
    fn print_items_tree(&self, title: &str, list: &[impl PrintableRequirement], verbose: bool);
}

impl PrintableTree for OnkostarEditor {
    fn print_tree(&self, verbose: bool) {
        println!(
            "Die Datei wurde am {} mit {} in Version {} erstellt.\n\nFolgende Inhalte sind gespeichert",
            style(&self.info_xml.datum_xml).yellow(),
            style(&self.info_xml.name).yellow(),
            style(&self.info_xml.version).yellow()
        );

        Self::print_items("Merkmalskataloge", &self.editor.property_catalogue, verbose);
        self.print_items_tree("Datenkataloge", &self.editor.data_catalogue, verbose);
        self.print_items_tree("Formulare", &self.editor.data_form, verbose);
        self.print_items_tree("Unterformulare", &self.editor.unterformular, verbose);
    }

    fn print_tree_filtered(&mut self, name: &str, verbose: bool) {
        println!(
            "Die Datei wurde am {} mit {} in Version {} erstellt.\n\nFolgende Inhalte für '{}' sind gespeichert",
            style(&self.info_xml.datum_xml).yellow(),
            style(&self.info_xml.name).yellow(),
            style(&self.info_xml.version).yellow(),
            name
        );

        self.filter_by_name_contains(name);

        Self::print_items("Merkmalskataloge", &self.editor.property_catalogue, verbose);
        self.print_items_tree("Datenkataloge", &self.editor.data_catalogue, verbose);
        self.print_items_tree("Formulare", &self.editor.data_form, verbose);
        self.print_items_tree("Unterformulare", &self.editor.unterformular, verbose);
    }

    fn print_items_tree(&self, title: &str, list: &[impl PrintableRequirement], verbose: bool) {
        print!("\n{} {}", list.len(), style(title).underlined());
        println!(
            " - Inhalte der Systembibliothek sind mit ({}), der Benutzerbibliothek mit (u) markiert",
            style("S").yellow()
        );
        for entry in list {
            println!("{}", entry.to_requirement_string(self, verbose));
        }
    }
}

pub trait PrintableDiff {
    fn print_diff(&mut self, other: &mut Self, strict: bool);
    fn print_item_diff(
        title: &str,
        list_a: &[impl Comparable + Named],
        list_b: &[impl Comparable + Named],
        strict: bool,
    );
}

impl PrintableDiff for OnkostarEditor {
    fn print_diff(&mut self, other: &mut Self, strict: bool) {
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
        list_a: &[impl Comparable + Named],
        list_b: &[impl Comparable + Named],
        strict: bool,
    ) {
        println!("\n{}", style(title).underlined());

        let mut has_diff = false;

        let names_a = list_a.iter().map(Named::get_name).collect::<Vec<_>>();
        let names_b = list_b.iter().map(Named::get_name).collect::<Vec<_>>();

        for entry in &names_b {
            if !names_a.contains(entry) {
                println!("{}: {}", entry, style("Nicht in Datei A enthalten!").red());
                has_diff = true;
            }
        }

        for entry in &names_a {
            if !names_b.contains(entry) {
                println!("{}: {}", entry, style("Nicht in Datei B enthalten!").red());
                has_diff = true;
            }
        }

        for entry_a in list_a {
            for entry_b in list_b {
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
                        Ordering::Equal => {
                            if strict && entry_a.get_hash() != entry_b.get_hash() {
                                println!(
                                    "{}: {} (z.B. GUID oder Reihenfolge von Unterelementen)",
                                    entry_a.get_name(),
                                    style("Inhaltlich verschieden").yellow()
                                );
                                has_diff = true;
                            } else if strict {
                                println!("{}: {}", entry_a.get_name(), style("Identisch").green());
                            }
                        }
                    }
                }
            }
        }

        if !has_diff {
            println!("Keine Unterschiede");
        }
    }
}

trait DisplayableRequirement {
    fn to_string(&self) -> String;
}

impl DisplayableRequirement for Requirement<'_> {
    fn to_string(&self) -> String {
        match self {
            Requirement::PropertyCatalogue(item) => item.to_listed_string(),
            Requirement::DataCatalogue(item) => item.to_listed_string(),
            Requirement::DataFormReference(item) | Requirement::DataFormSubform(item) => {
                item.to_listed_string()
            }
            Requirement::UnterformularReference(item) | Requirement::UnterformularSubform(item) => {
                item.to_listed_string()
            }
            Requirement::ExternalPropertyCatalogue(name) => {
                format!("Merkmalskatalog (-) '{name}' - hier nicht enthalten")
            }
            Requirement::ExternalDataCatalogue(name) => {
                format!("Datenkatalog (-) '{name}' - hier nicht enthalten")
            }
            Requirement::ExternalDataFormReference(name)
            | Requirement::ExternalDataFormSubform(name) => {
                format!("Formular (-) '{name}' - hier nicht enthalten")
            }
            Requirement::ExternalUnterformularReference(name)
            | Requirement::ExternalUnterformularSubform(name) => {
                format!("Unterformular (-) '{name}' - hier nicht enthalten")
            }
        }
    }
}
