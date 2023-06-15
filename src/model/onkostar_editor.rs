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

use console::style;
use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::model::data_catalogue::DataCatalogue;
use crate::model::data_form::DataForm;
use crate::model::property_catalogue::PropertyCatalogue;
use crate::model::unterformular::Unterformular;
use crate::model::{FormEntryContainer, Listable};
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
    pub fn apply_profile(&mut self, profile: &Profile) {
        self.editor.data_form.iter_mut().for_each(|data_form| {
            data_form.apply_profile(profile);
        });
        self.editor.unterformular.iter_mut().for_each(|data_form| {
            data_form.apply_profile(profile);
        })
    }

    pub fn print_list(&self) {
        println!(
            "Die Datei wurde am {} mit {} in Version {} erstellt.\n\nFolgende Inhalte sind gespeichert\n",
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
        println!("\n{} {}", list.len(), style(title).underlined());
        list.iter()
            .for_each(|entry| println!("{}", entry.to_listed_string()));
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
