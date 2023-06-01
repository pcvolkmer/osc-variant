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

use serde::{Deserialize, Serialize};

use crate::model::onkostar_editor::Ordner;

#[derive(Serialize, Deserialize, Debug)]
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
    ordner: Ordner
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entries {
    #[serde(rename = "Entry")]
    entry: Vec<Entry>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    #[serde(rename = "PropertyCatalogue")]
    #[serde(skip_serializing_if="Option::is_none")]
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
    #[serde(skip_serializing_if="Option::is_none")]
    range_from: Option<String>,
    #[serde(rename = "RangeUntil")]
    #[serde(skip_serializing_if="Option::is_none")]
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
    #[serde(skip_serializing_if="Option::is_none")]
    use_: Option<Use>,
    #[serde(rename = "FesteNachkommastellen")]
    #[serde(skip_serializing_if="Option::is_none")]
    feste_nachkommastellen: Option<u16>,
    #[serde(rename = "SID")]
    sid: String,
    #[serde(rename = "GUID")]
    guid: String,
    #[serde(rename = "Revision")]
    revision: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Use {
    #[serde(rename = "ProgramModule", default)]
    program_module: Vec<ProgramModule>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProgramModule {
    #[serde(rename="@program")]
    program: String,
    #[serde(rename="@name")]
    name: String
}