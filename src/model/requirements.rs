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

use crate::model::data_catalogue::DataCatalogue;
use crate::model::form::{DataFormType, Form, UnterformularType};
use crate::model::onkostar_editor::OnkostarEditor;
use crate::model::property_catalogue::PropertyCatalogue;
use crate::model::{Comparable, Listable, Sortable};
use std::fmt::Display;

#[allow(clippy::enum_variant_names)]
pub enum Requirement<'a> {
    PropertyCatalogue(&'a PropertyCatalogue),
    DataCatalogue(&'a DataCatalogue),
    ExternalPropertyCatalogue(String),
    ExternalDataCatalogue(String),
    DataFormReference(&'a Form<DataFormType>),
    UnterformularReference(&'a Form<UnterformularType>),
    #[allow(dead_code)]
    ExternalDataFormReference(String),
    ExternalUnterformularReference(String),

    DataFormSubform(&'a Form<DataFormType>),
    UnterformularSubform(&'a Form<UnterformularType>),
    #[allow(dead_code)]
    ExternalDataFormSubform(String),
    ExternalUnterformularSubform(String),
}

impl Sortable for Requirement<'_> {
    fn sorting_key(&self) -> String {
        match self {
            Requirement::PropertyCatalogue(item) => item.get_name(),
            Requirement::DataCatalogue(item) => item.get_name(),
            Requirement::DataFormReference(item) => item.get_name(),
            Requirement::UnterformularReference(item) => item.get_name(),
            Requirement::DataFormSubform(item) => item.get_name(),
            Requirement::UnterformularSubform(item) => item.get_name(),
            Requirement::ExternalPropertyCatalogue(name)
            | Requirement::ExternalDataCatalogue(name)
            | Requirement::ExternalDataFormReference(name)
            | Requirement::ExternalDataFormSubform(name)
            | Requirement::ExternalUnterformularReference(name)
            | Requirement::ExternalUnterformularSubform(name) => name.to_string(),
        }
    }
}

impl Display for Requirement<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Requirement::PropertyCatalogue(item) => item.to_listed_string(),
            Requirement::DataCatalogue(item) => item.to_listed_string(),
            Requirement::DataFormReference(item) => item.to_listed_string(),
            Requirement::UnterformularReference(item) => item.to_listed_string(),
            Requirement::DataFormSubform(item) => item.to_listed_string(),
            Requirement::UnterformularSubform(item) => item.to_listed_string(),
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
        };
        write!(f, "{str}")
    }
}

pub trait Requires
where
    Self: Listable,
{
    fn requires_form_reference(&self, _: &str) -> bool {
        false
    }

    fn requires_subform(&self, _: &str) -> bool {
        false
    }

    fn get_required_entries<'a>(&'a self, all: &'a OnkostarEditor) -> Vec<Requirement<'a>>;

    fn to_requirement_string<'a>(&'a self, all: &'a OnkostarEditor) -> String {
        format!(
            "{}\n{}",
            self.to_listed_string(),
            self.get_required_entries(all)
                .iter()
                .map(|entry| match entry {
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
                        Some(format!("  + {entry}\n"))
                    }
                    Requirement::DataFormReference(_)
                    | Requirement::ExternalDataFormReference(_)
                    | Requirement::UnterformularReference(_)
                    | Requirement::ExternalUnterformularReference(_) => {
                        Some(format!("  > {entry}\n"))
                    }
                    Requirement::DataFormSubform(_)
                    | Requirement::ExternalDataFormSubform(_)
                    | Requirement::UnterformularSubform(_)
                    | Requirement::ExternalUnterformularSubform(_) => {
                        Some(format!("  * {entry}\n"))
                    }
                    _ => None,
                })
                .filter(Option::is_some)
                .flatten()
                .collect::<String>()
        )
    }
}
