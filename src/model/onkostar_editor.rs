/*
 * MIT License
 *
 * 2023 Comprehensive Cancer Center Mainfranken
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

use crate::model::data_catalogue::DataCatalogue;
use crate::model::data_form::DataForm;
use crate::model::property_catalogue::PropertyCatalogue;
use crate::model::unterformular::Unterformular;

#[derive(Serialize, Deserialize, Debug)]
pub struct OnkostarEditor {
    #[serde(rename = "InfoXML")]
    info_xml: InfoXML,
    #[serde(rename = "Editor")]
    editor: Editor
}

impl OnkostarEditor {
    pub fn apply_variant(&mut self) {
        self.editor.data_form.iter_mut().for_each(|data_form| {
            data_form.apply_variant();
        });
        self.editor.unterformular.iter_mut().for_each(|data_form| {
            data_form.apply_variant();
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InfoXML {
    #[serde(rename = "DatumXML")]
    datum_xml: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Version")]
    version: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Editor {
    #[serde(rename = "PropertyCatalogue")]
    property_catalogue: Vec<PropertyCatalogue>,
    #[serde(rename = "DataCatalogue")]
    data_catalogue: Vec<DataCatalogue>,
    #[serde(rename = "Unterformular")]
    unterformular: Vec<Unterformular>,
    #[serde(rename = "DataForm")]
    data_form: Vec<DataForm>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ordner {
    #[serde(rename = "Bibliothek")]
    bibliothek: Bibliothek,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Typ")]
    typ: String,
    #[serde(rename = "ParentOrdner", default)]
    #[serde(skip_serializing_if="Option::is_none")]
    parent_order: Option<Box<Ordner>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bibliothek {
    #[serde(rename = "Name")]
    name: String
}

