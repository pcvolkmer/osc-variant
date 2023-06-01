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

pub mod onkostar_editor;
pub mod property_catalogue;
pub mod data_catalogue;
pub mod data_form;
pub mod unterformular;

#[derive(Serialize, Deserialize, Debug)]
pub struct Script {
    #[serde(rename = "Code")]
    code: String,
    #[serde(rename = "Valid")]
    valid: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlausibilityRule<T> {
    #[serde(rename = "Type")]
    type_: String,
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "Bezeichnung")]
    bezeichnung: String,
    #[serde(rename = "Formula")]
    formula: String,
    #[serde(rename = "Active")]
    active: bool,
    #[serde(rename = "Editable")]
    editable: bool,
    #[serde(rename = "Valid")]
    valid: bool,
    #[serde(rename = "Condition")]
    #[serde(skip_serializing_if="Option::is_none")]
    condition: Option<String>,
    #[serde(rename = "ConditionValid")]
    #[serde(skip_serializing_if="Option::is_none")]
    condition_valid: Option<bool>,
    #[serde(rename = "Statusauswirkung")]
    statusauswirkung: String,
    #[serde(rename = "DataFormEntries")]
    data_form_entries: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entries<T> {
    #[serde(rename = "Entry")]
    entry: Vec<T>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ansicht {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Beschreibung")]
    beschreibung: String,
    #[serde(rename = "Konfiguration")]
    konfiguration: String,
    #[serde(rename = "DataForm")]
    data_form: String,
    #[serde(rename = "DataCatalogue")]
    data_catalogue: String,
    #[serde(rename = "TypAuswahl")]
    typ_auswahl: String,
    #[serde(rename = "Suche")]
    suche: bool,
    #[serde(rename = "SID")]
    sid: String,
    #[serde(rename = "GUID")]
    guid: String,
    #[serde(rename = "Revision")]
    revision: u16,
    #[serde(rename = "InBibliothekAusliefern")]
    in_bibliothek_ausliefern: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ansichten {
    #[serde(rename = "Ansicht", default)]
    program_module: Vec<Ansicht>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MenuCategory {
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "position")]
    position: String,
    #[serde(rename = "column")]
    column: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Filter {
    #[serde(rename = "Condition")]
    condition: String,
    #[serde(rename = "Valid")]
    valid: bool,
    #[serde(rename = "RefEntries")]
    #[serde(skip_serializing_if="Option::is_none")]
    ref_entries: Option<RefEntries>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RefEntries {
    #[serde(rename = "RefEntry")]
    #[serde(skip_serializing_if="Option::is_none")]
    ref_entry: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlausibilityRules<T> {
    #[serde(rename = "PlausibilityRule")]
    #[serde(skip_serializing_if="Option::is_none")]
    plausibility_rule: Option<Vec<PlausibilityRule<T>>>
}