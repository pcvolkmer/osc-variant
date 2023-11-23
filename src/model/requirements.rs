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

use crate::model::data_catalogue::DataCatalogue;
use crate::model::data_form::DataForm;
use crate::model::onkostar_editor::OnkostarEditor;
use crate::model::property_catalogue::PropertyCatalogue;
use crate::model::unterformular::Unterformular;
use crate::model::{Comparable, Listable, Sortable};
use std::fmt::Display;

#[allow(clippy::enum_variant_names)]
pub enum Requirement<'a> {
    PropertyCatalogue(&'a PropertyCatalogue),
    DataCatalogue(&'a DataCatalogue),
    ExternalPropertyCatalogue(String),
    ExternalDataCatalogue(String),
    DataFormReference(&'a DataForm),
    UnterformularReference(&'a Unterformular),
    #[allow(dead_code)]
    ExternalDataFormReference(String),
    ExternalUnterformularReference(String),

    DataFormSubform(&'a DataForm),
    UnterformularSubform(&'a Unterformular),
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
                format!("Merkmalskatalog (-) '{}' - hier nicht enthalten", name)
            }
            Requirement::ExternalDataCatalogue(name) => {
                format!("Datenkatalog (-) '{}' - hier nicht enthalten", name)
            }
            Requirement::ExternalDataFormReference(name)
            | Requirement::ExternalDataFormSubform(name) => {
                format!("Formular (-) '{}' - hier nicht enthalten", name)
            }
            Requirement::ExternalUnterformularReference(name)
            | Requirement::ExternalUnterformularSubform(name) => {
                format!("Unterformular (-) '{}' - hier nicht enthalten", name)
            }
        };
        write!(f, "{}", str)
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

    fn get_required_entries<'a>(&'a self, all: &'a OnkostarEditor) -> Vec<Requirement>;

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
                            .filter(Option::is_some)
                            .map(|item| format!("    - {}\n", item.unwrap()))
                            .collect::<Vec<_>>()
                            .join("");

                        if inner.is_empty() {
                            Some(format!("  + {}\n", x.to_listed_string()))
                        } else {
                            Some(format!("  + {}\n{}", x.to_listed_string(), inner))
                        }
                    }
                    Requirement::ExternalDataCatalogue(_) => {
                        Some(format!("  + {}\n", entry))
                    }
                    Requirement::DataFormReference(_)
                    | Requirement::ExternalDataFormReference(_)
                    | Requirement::UnterformularReference(_)
                    | Requirement::ExternalUnterformularReference(_) => {
                        Some(format!("  > {}\n", entry))
                    }
                    Requirement::DataFormSubform(_)
                    | Requirement::ExternalDataFormSubform(_)
                    | Requirement::UnterformularSubform(_)
                    | Requirement::ExternalUnterformularSubform(_) => {
                        Some(format!("  * {}\n", entry))
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
