/*
 * This file is part of osc-variant
 *
 * Copyright (C) 2024-2026 the original author or authors.
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

use std::fmt::Debug;
use std::str::FromStr;

use crate::osc::data_catalogue::DataCatalogue;
use crate::osc::form::{DataFormType, Form, UnterformularType};
use crate::osc::other::{Ablaufschema, Akte, RecordLinkage, Rskript, SidGuid};
use crate::osc::property_catalogue::PropertyCatalogue;
use crate::osc::{Comparable, FolderContained, Named, Sortable};
use crate::profile::{Profile, ProfileApplicable};

use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct OnkostarEditor {
    #[serde(rename = "InfoXML")]
    pub info_xml: InfoXML,
    #[serde(rename = "Editor")]
    pub editor: Editor,
}

impl OnkostarEditor {
    pub fn find_property_catalogue(&self, name: &str) -> Option<&PropertyCatalogue> {
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

    pub fn find_data_catalogue(&self, name: &str) -> Option<&DataCatalogue> {
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

    pub fn find_data_form(&self, name: &str) -> Option<&Form<DataFormType>> {
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

    pub fn find_unterformular(&self, name: &str) -> Option<&Form<UnterformularType>> {
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

    pub fn filter_by_name_contains(&mut self, name: &str) {
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

    pub fn sorted(&mut self) {
        self.editor
            .property_catalogue
            .sort_unstable_by_key(Sortable::sorting_key);

        self.editor.property_catalogue.iter_mut().for_each(|item| {
            item.sorted();
        });

        self.editor
            .data_catalogue
            .sort_unstable_by_key(Sortable::sorting_key);

        self.editor.data_catalogue.iter_mut().for_each(|item| {
            item.sorted();
        });

        /*self.editor
        .data_form
        .sort_unstable_by_key(|e| e.sorting_key());*/

        self.editor
            .data_form
            .sort_unstable_by(Form::compare_by_requirement);

        self.editor.data_form.iter_mut().for_each(|item| {
            item.sorted();
        });

        /*self.editor
        .unterformular
        .sort_unstable_by_key(|e| e.sorting_key());*/

        self.editor
            .unterformular
            .sort_unstable_by(Form::compare_by_requirement);

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

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct InfoXML {
    #[serde(rename = "DatumXML")]
    pub datum_xml: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Version")]
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Editor {
    #[serde(rename = "PropertyCatalogue", default)]
    pub property_catalogue: Vec<PropertyCatalogue>,
    #[serde(rename = "DataCatalogue", default)]
    pub data_catalogue: Vec<DataCatalogue>,
    #[serde(rename = "Unterformular", default)]
    pub unterformular: Vec<Form<UnterformularType>>,
    #[serde(rename = "DataForm", default)]
    pub data_form: Vec<Form<DataFormType>>,

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
