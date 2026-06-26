/*
 * This file is part of osc-variant
 *
 * Copyright (C) 2023-2026 the original author or authors.
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

use crate::osc::data_catalogue::DataCatalogue;
use crate::osc::form::{DataFormType, Form, UnterformularType};
use crate::osc::onkostar_editor::OnkostarEditor;
use crate::osc::property_catalogue::PropertyCatalogue;
use crate::osc::{Named, Sortable};

#[allow(clippy::enum_variant_names)]
pub enum Requirement<'a> {
    PropertyCatalogue(&'a PropertyCatalogue),
    DataCatalogue(&'a DataCatalogue),
    ExternalPropertyCatalogue(String),
    ExternalDataCatalogue(String),

    DataFormReference(&'a Form<DataFormType>),
    UnterformularReference(&'a Form<UnterformularType>),
    ExternalDataFormReference(String),
    #[allow(dead_code)]
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
            Requirement::DataFormReference(item) | Requirement::DataFormSubform(item) => {
                item.get_name()
            }
            Requirement::UnterformularReference(item) | Requirement::UnterformularSubform(item) => {
                item.get_name()
            }
            Requirement::ExternalPropertyCatalogue(name)
            | Requirement::ExternalDataCatalogue(name)
            | Requirement::ExternalDataFormReference(name)
            | Requirement::ExternalDataFormSubform(name)
            | Requirement::ExternalUnterformularReference(name)
            | Requirement::ExternalUnterformularSubform(name) => name.clone(),
        }
    }
}

pub trait Requires {
    fn requires_form_reference(&self, _: &str) -> bool {
        false
    }

    fn requires_subform(&self, _: &str) -> bool {
        false
    }

    fn get_required_entries<'a>(&'a self, all: &'a OnkostarEditor) -> Vec<Requirement<'a>>;
}
