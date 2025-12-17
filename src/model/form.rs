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
use crate::checks::CheckNotice::{ErrorWithCode, Warning};
use crate::checks::{CheckNotice, Checkable};
use crate::model::onkostar_editor::OnkostarEditor;
use crate::model::other::Entry;
use crate::model::requirements::{Requirement, Requires};
use crate::model::{
    Ansichten, Comparable, Entries, FolderContent, FormEntry, FormEntryContainer, Kennzahlen,
    Listable, MenuCategory, PlausibilityRules, PunkteKategorien, Script, Sortable,
    apply_profile_to_form_entry, apply_profile_to_form_field,
};
use crate::model::{Haeufigkeiten, Ordner};
use crate::profile::Profile;
use console::style;
use serde::{Deserialize, Serialize};
use std::any::TypeId;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct DataFormType;

#[derive(Debug)]
pub struct UnterformularType;

#[derive(Debug)]
pub struct DataFormReferenceType;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Form<Type> {
    #[serde(skip)]
    _type: PhantomData<Type>,

    #[serde(rename = "DataCatalogues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    data_catalogues: Option<DataCatalogues>,
    #[serde(rename = "Category")]
    category: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
    #[serde(rename = "MenuEntry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    menu_entry: Option<String>,
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(rename = "Note")]
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<String>,
    #[serde(rename = "Readonly")]
    readonly: bool,
    #[serde(rename = "Active")]
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(rename = "TudokPosition")]
    tudok_position: String,
    #[serde(rename = "Aktenbereich")]
    #[serde(skip_serializing_if = "Option::is_none")]
    aktenbereich: Option<String>,
    #[serde(rename = "BefragungRelevant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    befragung_relevant: Option<bool>,
    #[serde(rename = "Hotkey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    hotkey: Option<String>,
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<String>,
    #[serde(rename = "BigSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    big_summary: Option<String>,
    #[serde(rename = "KalenderSchnipsel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    kalender_schnipsel: Option<String>,
    #[serde(rename = "EmailTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    mail_template: Option<String>,
    #[serde(rename = "ErkrankungText", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    erkrankung_text: Option<String>,
    #[serde(rename = "ErkrankungTextLong")]
    #[serde(skip_serializing_if = "Option::is_none")]
    erkrankung_text_long: Option<String>,
    #[serde(rename = "ErkrankungProzedurText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    erkrankung_prozedur_text: Option<String>,
    #[serde(rename = "ErkrankungSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    erkrankung_summary: Option<String>,
    #[serde(rename = "ErkrankungBigSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    erkrankung_big_summary: Option<String>,
    #[serde(rename = "Kontext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    kontext: Option<i32>,
    #[serde(rename = "Datenart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    datenart: Option<String>,
    #[serde(rename = "ShowHistoryButton", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    show_history_button: Option<bool>,
    #[serde(rename = "TudokReadonly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    tudok_readonly: Option<bool>,
    #[serde(rename = "VitalstatusRelevant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    vitalstatus_relevant: Option<bool>,
    #[serde(rename = "AutoNummerierung")]
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_nummerierung: Option<bool>,
    #[serde(rename = "Zwischenspeichern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    zwischenspeichern: Option<bool>,
    #[serde(rename = "Zurueckblaettern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    zurueckblaettern: Option<bool>,
    #[serde(rename = "Datenbankexport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    datenbankexport: Option<bool>,
    #[serde(rename = "DatenschutzRelevant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    datenschutz_relevant: Option<bool>,
    #[serde(rename = "KonferenzRelevant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    konferenz_relevant: Option<bool>,
    #[serde(rename = "Drucken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    drucken: Option<String>,
    #[serde(rename = "hatUnterformulare")]
    hat_unterformulare: bool,
    #[serde(rename = "ScriptBeimSchliessen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    script_beim_schliessen: Option<Script>,
    #[serde(rename = "ScriptBeimSpeichern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    script_beim_speichern: Option<Script>,
    #[serde(rename = "ScriptBeimNeuanlegen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    script_beim_neuanlegen: Option<Script>,
    #[serde(rename = "ScriptBeimBearbeiten")]
    #[serde(skip_serializing_if = "Option::is_none")]
    script_beim_bearbeiten: Option<Script>,
    #[serde(rename = "ScriptBeimKopieren")]
    #[serde(skip_serializing_if = "Option::is_none")]
    script_beim_kopieren: Option<Script>,
    #[serde(rename = "ScriptBeimImport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    script_beim_import: Option<Script>,
    #[serde(rename = "ScriptBeimAnonymisieren")]
    #[serde(skip_serializing_if = "Option::is_none")]
    script_beim_anonymisieren: Option<Script>,
    #[serde(rename = "SID")]
    sid: String,
    #[serde(rename = "GUID")]
    guid: String,
    #[serde(rename = "Revision")]
    revision: u16,
    #[serde(rename = "maxAnzahl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    max_anzahl: Option<u16>,
    #[serde(rename = "VerknuepftGUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    verknuepft_guid: Option<String>,
    #[serde(rename = "SeitenzahlSichtbar")]
    #[serde(skip_serializing_if = "Option::is_none")]
    seitenanzahl_sichtbar: Option<bool>,
    #[serde(rename = "Entries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    entries: Option<Entries<Entry>>,
    #[serde(rename = "PlausibilityRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    plausibility_rules: Option<PlausibilityRules<DataFormEntries>>,
    #[serde(rename = "Haeufigkeiten")]
    #[serde(skip_serializing_if = "Option::is_none")]
    haeufigkeiten: Option<Haeufigkeiten>,
    #[serde(rename = "Kennzahlen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    kennzahlen: Option<Kennzahlen>,
    #[serde(rename = "Ordner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    ordner: Option<Ordner>,
    #[serde(rename = "MenuCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    menu_category: Option<MenuCategory>,
    #[serde(rename = "PunkteKategorien")]
    #[serde(skip_serializing_if = "Option::is_none")]
    punkte_kategorien: Option<PunkteKategorien>,
    #[serde(rename = "Ansichten")]
    #[serde(skip_serializing_if = "Option::is_none")]
    ansichten: Option<Ansichten>,
}

impl<Type: 'static> FormEntryContainer for Form<Type> {
    fn apply_profile(&mut self, profile: &Profile) {
        profile.forms.iter().for_each(|profile_form| {
            if self.name == profile_form.name
                && let Some(ref mut entries) = self.entries
            {
                entries.entry.iter_mut().for_each(|entry| {
                    profile_form
                        .form_references
                        .iter()
                        .for_each(|form_reference| {
                            apply_profile_to_form_entry(entry, form_reference);
                        });

                    // Hide form field using filter set to "false" if requested and change default value
                    profile_form
                        .form_fields
                        .iter()
                        .for_each(|form_field| apply_profile_to_form_field(entry, form_field));

                    if let Some(menu_category) = &profile_form.menu_category {
                        self.menu_category = Some(MenuCategory {
                            name: menu_category.name.clone(),
                            position: menu_category.position.clone(),
                            column: menu_category.column.clone(),
                        });
                    }
                });
            }
        });
    }
}

impl<Type: 'static> Listable for Form<Type>
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
            style(&self.name).yellow(),
            style(&self.revision).yellow()
        )
    }
}

impl<Type: 'static> Sortable for Form<Type> {
    fn sorting_key(&self) -> String {
        self.name.clone()
    }

    fn sorted(&mut self) -> &Self {
        if let Some(ref mut data_catalogues) = self.data_catalogues {
            data_catalogues.data_catalogue.sort_unstable();
        }

        if let Some(ref mut entries) = self.entries {
            entries.entry.sort_unstable_by_key(Entry::sorting_key);

            entries.entry.iter_mut().for_each(|item| {
                item.sorted();
            });
        }

        if let Some(ref mut plausibility_rules) = self.plausibility_rules
            && let Some(ref mut plausibility_rule) = plausibility_rules.plausibility_rule
        {
            plausibility_rule.sort_unstable_by_key(|item| item.bezeichnung.clone());

            for item in plausibility_rule {
                if let Some(ref mut data_form_entry_names) = item.data_form_entries.entry_name {
                    data_form_entry_names.sort_unstable();
                }
            }
        }

        self
    }
}

impl<Type> Comparable for Form<Type>
where
    Type: Debug + 'static,
{
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_revision(&self) -> u16 {
        self.revision
    }

    fn compare_by_requirement(a: &Self, b: &Self) -> Ordering {
        if a.get_name() == b.get_name()
            || a.is_system_library_content()
            || b.is_system_library_content()
        {
            return Ordering::Equal;
        }

        if a.requires_form_reference(&b.get_name()) || a.requires_subform(&b.get_name()) {
            return Ordering::Greater;
        }

        Ordering::Less
    }
}

impl<Type> Requires for Form<Type>
where
    Self: Listable + 'static,
{
    fn requires_form_reference(&self, name: &str) -> bool {
        if let Some(ref entries) = self.entries {
            entries
                .entry
                .iter()
                .map(|item| {
                    item.type_ == "formReference"
                        && match item.referenced_data_form.as_ref() {
                            Some(refname) => refname == name,
                            _ => false,
                        }
                })
                .filter(|&it| it)
                .next_back()
                .unwrap_or_default()
        } else {
            false
        }
    }

    fn requires_subform(&self, name: &str) -> bool {
        if let Some(ref entries) = self.entries {
            entries
                .entry
                .iter()
                .map(|item| {
                    item.type_ == "subform"
                        && match item.referenced_data_form.as_ref() {
                            Some(refname) => refname == name,
                            _ => false,
                        }
                })
                .filter(|&it| it)
                .next_back()
                .unwrap_or_default()
        } else {
            false
        }
    }

    fn get_required_entries<'a>(&'a self, all: &'a OnkostarEditor) -> Vec<Requirement<'a>> {
        let mut result = match self.data_catalogues {
            Some(ref data_catalogues) => data_catalogues
                .data_catalogue
                .iter()
                .collect::<HashSet<_>>()
                .into_iter()
                .map(|entry| match all.find_data_catalogue(entry.as_str()) {
                    Some(contained) => Requirement::DataCatalogue(contained),
                    None => Requirement::ExternalDataCatalogue(entry.clone()),
                })
                .collect::<Vec<_>>(),
            None => vec![],
        };

        result.sort_unstable_by_key(Requirement::sorting_key);

        if let Some(ref entries) = self.entries {
            let referenced_forms = &mut entries
                .entry
                .iter()
                .filter(|&entry| entry.get_type() == "formReference")
                .filter_map(|entry| match &entry.referenced_data_form {
                    Some(name) => Some(name),
                    None => None,
                })
                .collect::<HashSet<_>>()
                .into_iter()
                .map(|entry| match all.find_data_form(entry.as_str()) {
                    Some(contained) => Requirement::DataFormReference(contained),
                    None => match all.find_unterformular(entry.as_str()) {
                        Some(contained) => Requirement::UnterformularReference(contained),
                        None => Requirement::ExternalDataFormReference(entry.clone()),
                    },
                })
                .collect::<Vec<_>>();

            //
            let new_referenced_forms = &mut entries
                .entry
                .iter()
                .flat_map(|entry| &entry.data_form_references)
                .flat_map(|rdf| {
                    rdf.referenced_data_form
                        .iter()
                        .map(|x| x.name.clone())
                        .collect::<Vec<_>>()
                })
                .map(|entry| match all.find_data_form(entry.as_str()) {
                    Some(contained) => Requirement::DataFormReference(contained),
                    None => match all.find_unterformular(entry.as_str()) {
                        Some(contained) => Requirement::UnterformularReference(contained),
                        None => Requirement::ExternalDataFormReference(entry.clone()),
                    },
                })
                .collect::<Vec<_>>();
            referenced_forms.append(new_referenced_forms);
            referenced_forms.sort_unstable_by_key(Requirement::sorting_key);

            referenced_forms.dedup_by_key(|requirement| requirement.sorting_key());
            result.append(referenced_forms);
            //

            let sub_forms = &mut entries
                .entry
                .iter()
                .filter(|&entry| entry.get_type() == "subform")
                .filter_map(|entry| match &entry.referenced_data_form {
                    Some(name) => Some(name),
                    None => None,
                })
                .collect::<HashSet<_>>()
                .into_iter()
                .map(|entry| match all.find_data_form(entry.as_str()) {
                    Some(contained) => Requirement::DataFormSubform(contained),
                    None => match all.find_unterformular(entry.as_str()) {
                        Some(contained) => Requirement::UnterformularSubform(contained),
                        None => Requirement::ExternalUnterformularSubform(entry.clone()),
                    },
                })
                .collect::<Vec<_>>();
            sub_forms.sort_unstable_by_key(Requirement::sorting_key);
            result.append(sub_forms);
        }

        result
    }
}

impl<Type: 'static> FolderContent for Form<Type> {
    fn get_library_folder(&self) -> String {
        match &self.ordner {
            Some(ordner) => ordner.bibliothek.name.clone(),
            None => String::new(),
        }
    }
}

impl<Type> Form<Type> {
    fn common_check(&self) -> Vec<CheckNotice> {
        let missing_forms_in_refs = match self.entries {
            Some(ref entries) => entries
                .entry
                .iter()
                .filter(|entry| {
                    entry.type_ == "formReference"
                        && entry.referenced_data_form.is_none()
                        && entry.data_form_references.is_none()
                })
                .map(|entry| format!("'{}'", entry.get_name()))
                .collect::<Vec<_>>(),
            None => vec![],
        };

        let missing_forms_in_refs_legacy = match self.entries {
            Some(ref entries) => entries
                .entry
                .iter()
                .filter(|entry| {
                    entry.type_ == "formReference" && entry.referenced_data_form.is_none()
                })
                .map(|entry| format!("'{}'", entry.get_name()))
                .collect::<Vec<_>>(),
            None => vec![],
        };

        let mut result = vec![];

        if !missing_forms_in_refs.is_empty() && !missing_forms_in_refs_legacy.is_empty() {
            result.push(ErrorWithCode {
                code: "2024-0005".to_string(),
                description: format!(
                    "Formular '{}' hat Formularverweise ohne Angabe des Formulars in: {}",
                    self.name,
                    missing_forms_in_refs.join(", ")
                ),
                line: None,
                example: None,
            });
        }

        if missing_forms_in_refs.is_empty() && !missing_forms_in_refs_legacy.is_empty() {
            result.push(Warning {
                description: format!(
                    "Formular '{}' hat Formularverweise, die erst in neueren Onkostar-Versionen ab 2.14.0 funktionieren",
                    self.name
                ),
                line: None,
            });
        }

        result
    }
}

impl Checkable for Form<DataFormType> {
    fn check(&self) -> Vec<CheckNotice> {
        let mut result = match self.entries {
            Some(ref entries) => {
                if entries
                    .entry
                    .iter()
                    .filter(|entry| entry.procedure_date_status != "none")
                    .count()
                    == 0
                {
                    vec![ErrorWithCode {
                        code: "2023-0002".to_string(),
                        description: format!(
                            "Formular '{}' hat keine Angabe zum Prozedurdatum",
                            self.name
                        ),
                        line: None,
                        example: None,
                    }]
                } else {
                    vec![]
                }
            }
            None => vec![],
        };

        result.append(&mut self.common_check());

        result
    }
}

impl Checkable for Form<UnterformularType> {
    fn check(&self) -> Vec<CheckNotice> {
        let mut result = if self.hat_unterformulare {
            vec![ErrorWithCode {
                code: "2023-0001".to_string(),
                description: format!(
                    "Unterformular '{}' mit Markierung 'hat Unterformulare'",
                    self.name
                ),
                line: None,
                example: None,
            }]
        } else {
            vec![]
        };

        result.append(&mut self.common_check());

        result
    }
}

impl Form<DataFormReferenceType> {
    /// Create a new minimal form reference with given form name
    pub fn new_form_reference(reference_name: &str) -> Self {
        Self {
            _type: PhantomData::<DataFormReferenceType>,
            data_catalogues: None,
            category: "0".to_string(),
            name: reference_name.to_string(),
            version: None,
            menu_entry: None,
            title: None,
            description: None,
            note: None,
            readonly: false,
            active: None,
            tudok_position: "0".to_string(),
            aktenbereich: None,
            befragung_relevant: None,
            hotkey: None,
            summary: None,
            big_summary: None,
            kalender_schnipsel: None,
            mail_template: None,
            erkrankung_text: None,
            erkrankung_text_long: None,
            erkrankung_prozedur_text: None,
            erkrankung_summary: None,
            erkrankung_big_summary: None,
            kontext: None,
            datenart: None,
            show_history_button: None,
            tudok_readonly: None,
            vitalstatus_relevant: None,
            auto_nummerierung: None,
            zwischenspeichern: None,
            zurueckblaettern: None,
            datenbankexport: None,
            datenschutz_relevant: None,
            konferenz_relevant: None,
            drucken: None,
            hat_unterformulare: false,
            script_beim_schliessen: None,
            script_beim_speichern: None,
            script_beim_neuanlegen: None,
            script_beim_bearbeiten: None,
            script_beim_kopieren: None,
            script_beim_import: None,
            script_beim_anonymisieren: None,
            sid: "2001".to_string(),
            guid: "00000000-0000-0000-0000-000000000000".to_string(),
            revision: 1,
            max_anzahl: None,
            verknuepft_guid: None,
            seitenanzahl_sichtbar: None,
            entries: None,
            plausibility_rules: None,
            haeufigkeiten: None,
            kennzahlen: None,
            ordner: None,
            menu_category: None,
            punkte_kategorien: None,
            ansichten: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct DataCatalogues {
    #[serde(rename = "DataCatalogue")]
    data_catalogue: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct DataFormEntries {
    #[serde(rename = "EntryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    entry_name: Option<Vec<String>>,
}

#[allow(clippy::unwrap_used)]
#[allow(clippy::panic)]
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::model::Script;
    use crate::model::onkostar_editor::OnkostarEditor;
    use crate::profile::Profile;

    #[test]
    fn should_change_dataform_entry_default_value() {
        let onkostar_editor = OnkostarEditor::from_str(include_str!("../../tests/test.osc"));

        assert!(onkostar_editor.is_ok());
        let mut onkostar_editor = onkostar_editor.unwrap();

        let profile = "forms:
               - name: 'Hauptformular'
                 form_fields:
                   - name: Auswahl
                     default_value: 'B'
            ";

        let profile = Profile::from_str(profile);
        assert!(profile.is_ok());
        let profile = profile.unwrap();

        onkostar_editor.apply_profile(&profile);

        let Some(actual) = &onkostar_editor.editor.data_form[0].entries else {
            panic!()
        };

        assert_eq!(actual.entry[2].name, "Auswahl");
        assert_eq!(actual.entry[2].default_value, "B");
    }

    #[test]
    fn should_not_change_dataform_entry_default_value() {
        let onkostar_editor = OnkostarEditor::from_str(include_str!("../../tests/test.osc"));

        assert!(onkostar_editor.is_ok());
        let mut onkostar_editor = onkostar_editor.unwrap();

        let profile = "forms:
               - name: 'Hauptformular'
            ";

        let profile = Profile::from_str(profile);
        assert!(profile.is_ok());
        let profile = profile.unwrap();

        onkostar_editor.apply_profile(&profile);

        let Some(actual) = &onkostar_editor.editor.data_form[0].entries else {
            panic!()
        };

        assert_eq!(actual.entry[2].name, "Auswahl");
        assert_eq!(actual.entry[2].default_value, "");
    }

    #[test]
    fn should_change_menu_category() {
        let onkostar_editor = OnkostarEditor::from_str(include_str!("../../tests/test.osc"));

        assert!(onkostar_editor.is_ok());
        let mut onkostar_editor = onkostar_editor.unwrap();

        let profile = "forms:
               - name: 'Hauptformular'
                 menu_category:
                   name: Testformulare
                   position: 1.0
                   column: 1
            ";

        let profile = Profile::from_str(profile);
        assert!(profile.is_ok());
        let profile = profile.unwrap();

        onkostar_editor.apply_profile(&profile);

        match &onkostar_editor.editor.data_form[0].menu_category {
            Some(menu_category) => assert_eq!(menu_category.name, "Testformulare"),
            _ => panic!("Test failed: MenuCategory not found!"),
        }
    }

    #[test]
    fn should_keep_menu_category() {
        let onkostar_editor = OnkostarEditor::from_str(include_str!("../../tests/test.osc"));

        assert!(onkostar_editor.is_ok());
        let mut onkostar_editor = onkostar_editor.unwrap();

        let profile = "forms:
               - name: 'Hauptformular'
            ";

        let profile = Profile::from_str(profile);
        assert!(profile.is_ok());
        let profile = profile.unwrap();

        onkostar_editor.apply_profile(&profile);

        match &onkostar_editor.editor.data_form[0].menu_category {
            Some(menu_category) => assert_eq!(menu_category.name, "Test"),
            _ => panic!("Test failed: MenuCategory not found!"),
        }
    }

    #[test]
    fn should_change_dataform_entry_scripts_code_with_form_fields() {
        let onkostar_editor = OnkostarEditor::from_str(include_str!("../../tests/test.osc"));

        assert!(onkostar_editor.is_ok());
        let mut onkostar_editor = onkostar_editor.unwrap();

        let profile = "forms:
               - name: 'Hauptformular'
                 form_fields:
                   - name: Auswahl
                     scripts_code: |-
                       // Example code
                       console.log(42);
            ";

        let profile = Profile::from_str(profile);
        assert!(profile.is_ok());
        let profile = profile.unwrap();

        onkostar_editor.apply_profile(&profile);

        let Some(actual) = &onkostar_editor.editor.data_form[0].entries else {
            panic!()
        };

        assert_eq!(
            actual.entry[2].scripts,
            Some(Script {
                code: "// Example code&#10;console.log(42);".into(),
                valid: true
            })
        );
    }

    #[test]
    fn should_change_dataform_entry_scripts_code_with_form_references() {
        let onkostar_editor = OnkostarEditor::from_str(include_str!("../../tests/test.osc"));

        assert!(onkostar_editor.is_ok());
        let mut onkostar_editor = onkostar_editor.unwrap();

        let profile = "forms:
               - name: 'Hauptformular'
                 form_fields:
                   - name: Auswahl
                     scripts_code: |-
                       // Example code
                       console.log(42);
            ";

        let profile = Profile::from_str(profile);
        assert!(profile.is_ok());
        let profile = profile.unwrap();

        onkostar_editor.apply_profile(&profile);

        let Some(actual) = &onkostar_editor.editor.data_form[0].entries else {
            panic!()
        };

        assert_eq!(
            actual.entry[2].scripts,
            Some(Script {
                code: "// Example code&#10;console.log(42);".into(),
                valid: true
            })
        );
    }

    #[test]
    fn should_remove_dataform_entry_filter_with_form_fields() {
        let onkostar_editor = OnkostarEditor::from_str(include_str!("../../tests/test.osc"));

        assert!(onkostar_editor.is_ok());
        let mut onkostar_editor = onkostar_editor.unwrap();

        let profile = "forms:
               - name: 'Hauptformular'
                 form_fields:
                   - name: Auswahl
                     remove_filter: true
                     scripts_code: |-
                       // Example code
                       console.log(42);
            ";

        let profile = Profile::from_str(profile);
        assert!(profile.is_ok());
        let profile = profile.unwrap();

        onkostar_editor.apply_profile(&profile);

        let Some(actual) = &onkostar_editor.editor.data_form[0].entries else {
            panic!()
        };

        assert_eq!(actual.entry[0].filter, None);

        assert_eq!(actual.entry[1].filter, None);

        assert_eq!(actual.entry[2].filter, None);

        assert_eq!(actual.entry[3].filter, None);
    }

    #[test]
    fn should_remove_dataform_entry_filter_with_form_references() {
        let onkostar_editor = OnkostarEditor::from_str(include_str!("../../tests/test.osc"));

        assert!(onkostar_editor.is_ok());
        let mut onkostar_editor = onkostar_editor.unwrap();

        let profile = "forms:
               - name: 'Hauptformular'
                 form_fields:
                   - name: Auswahl
                     remove_filter: true
                     scripts_code: |-
                       // Example code
                       console.log(42);
            ";

        let profile = Profile::from_str(profile);
        assert!(profile.is_ok());
        let profile = profile.unwrap();

        onkostar_editor.apply_profile(&profile);

        let Some(actual) = &onkostar_editor.editor.data_form[0].entries else {
            panic!()
        };

        assert_eq!(actual.entry[0].filter, None);

        assert_eq!(actual.entry[1].filter, None);

        assert_eq!(actual.entry[2].filter, None);

        assert_eq!(actual.entry[3].filter, None);
    }

    #[test]
    fn should_change_unterformular_entry_default_value() {
        let onkostar_editor = OnkostarEditor::from_str(include_str!("../../tests/test.osc"));

        assert!(onkostar_editor.is_ok());
        let mut onkostar_editor = onkostar_editor.unwrap();

        let profile = "forms:
               - name: 'Unterformular'
                 form_fields:
                   - name: Termin
                     default_value: '2024-03-18'
            ";

        let profile = Profile::from_str(profile);
        assert!(profile.is_ok());
        let profile = profile.unwrap();

        onkostar_editor.apply_profile(&profile);

        let Some(actual) = &onkostar_editor.editor.unterformular[0].entries else {
            panic!()
        };

        assert_eq!(actual.entry[1].name, "Termin");
        assert_eq!(actual.entry[1].default_value, "2024-03-18");
    }

    #[test]
    fn should_not_change_unterformular_entry_default_value() {
        let onkostar_editor = OnkostarEditor::from_str(include_str!("../../tests/test.osc"));

        assert!(onkostar_editor.is_ok());
        let mut onkostar_editor = onkostar_editor.unwrap();

        let profile = "forms:
               - name: 'Unterformular'
            ";

        let profile = Profile::from_str(profile);
        assert!(profile.is_ok());
        let profile = profile.unwrap();

        onkostar_editor.apply_profile(&profile);

        let Some(actual) = &onkostar_editor.editor.unterformular[0].entries else {
            panic!()
        };

        assert_eq!(actual.entry[1].name, "Termin");
        assert_eq!(actual.entry[1].default_value, "");
    }

    #[test]
    fn should_ignore_menu_category_for_subform() {
        let onkostar_editor = OnkostarEditor::from_str(include_str!("../../tests/test.osc"));

        assert!(onkostar_editor.is_ok());
        let mut onkostar_editor = onkostar_editor.unwrap();

        let profile = "forms:
               - name: 'Unterformular'
            ";

        let profile = Profile::from_str(profile);
        assert!(profile.is_ok());
        let profile = profile.unwrap();

        onkostar_editor.apply_profile(&profile);

        assert!(
            &onkostar_editor.editor.unterformular[0]
                .menu_category
                .is_none()
        );
    }

    #[test]
    fn should_change_unterformular_entry_scripts_code_with_form_fields() {
        let onkostar_editor = OnkostarEditor::from_str(include_str!("../../tests/test.osc"));

        assert!(onkostar_editor.is_ok());
        let mut onkostar_editor = onkostar_editor.unwrap();

        let profile = "forms:
               - name: 'Unterformular'
                 form_fields:
                   - name: Termin
                     scripts_code: |-
                       // Example code
                       console.log(42);
            ";

        let profile = Profile::from_str(profile);
        assert!(profile.is_ok());
        let profile = profile.unwrap();

        onkostar_editor.apply_profile(&profile);

        let Some(actual) = &onkostar_editor.editor.unterformular[0].entries else {
            panic!()
        };

        assert_eq!(
            actual.entry[1].scripts,
            Some(Script {
                code: "// Example code&#10;console.log(42);".into(),
                valid: true
            })
        );
    }

    #[test]
    fn should_change_unterformular_entry_scripts_code_with_form_references() {
        let onkostar_editor = OnkostarEditor::from_str(include_str!("../../tests/test.osc"));

        assert!(onkostar_editor.is_ok());
        let mut onkostar_editor = onkostar_editor.unwrap();

        let profile = "forms:
               - name: 'Unterformular'
                 form_fields:
                   - name: Termin
                     scripts_code: |-
                       // Example code
                       console.log(42);
            ";

        let profile = Profile::from_str(profile);
        assert!(profile.is_ok());
        let profile = profile.unwrap();

        onkostar_editor.apply_profile(&profile);

        let Some(actual) = &onkostar_editor.editor.unterformular[0].entries else {
            panic!()
        };

        assert_eq!(
            actual.entry[1].scripts,
            Some(Script {
                code: "// Example code&#10;console.log(42);".into(),
                valid: true
            })
        );
    }

    #[test]
    fn should_remove_unterformular_entry_filter_with_form_fields() {
        let onkostar_editor = OnkostarEditor::from_str(include_str!("../../tests/test.osc"));

        assert!(onkostar_editor.is_ok());
        let mut onkostar_editor = onkostar_editor.unwrap();

        let profile = "forms:
               - name: 'Unterformular'
                 form_fields:
                   - name: Termin
                     remove_filter: true
                     scripts_code: |-
                       // Example code
                       console.log(42);
            ";

        let profile = Profile::from_str(profile);
        assert!(profile.is_ok());
        let profile = profile.unwrap();

        onkostar_editor.apply_profile(&profile);

        let Some(actual) = &onkostar_editor.editor.unterformular[0].entries else {
            panic!()
        };

        assert_eq!(actual.entry[0].filter, None);
        assert_eq!(actual.entry[1].filter, None);
    }

    #[test]
    fn should_remove_unterformular_entry_filter_with_form_references() {
        let onkostar_editor = OnkostarEditor::from_str(include_str!("../../tests/test.osc"));

        assert!(onkostar_editor.is_ok());
        let mut onkostar_editor = onkostar_editor.unwrap();

        let profile = "forms:
               - name: 'Unterformular'
                 form_fields:
                   - name: Termin
                     remove_filter: true
                     scripts_code: |-
                       // Example code
                       console.log(42);
            ";

        let profile = Profile::from_str(profile);
        assert!(profile.is_ok());
        let profile = profile.unwrap();

        onkostar_editor.apply_profile(&profile);

        let Some(actual) = &onkostar_editor.editor.unterformular[0].entries else {
            panic!()
        };

        assert_eq!(actual.entry[0].filter, None);
        assert_eq!(actual.entry[1].filter, None);
    }
}
