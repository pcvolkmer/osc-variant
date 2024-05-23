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

use std::cmp::Ordering;
use std::collections::HashSet;

use console::style;
use serde::{Deserialize, Serialize};

use crate::checks::CheckNotice::ErrorWithCode;
use crate::checks::{CheckNotice, Checkable};
use crate::model::onkostar_editor::OnkostarEditor;
use crate::model::requirements::{Requirement, Requires};
use crate::model::{
    apply_profile_to_form_entry, apply_profile_to_form_field, Ansichten, Comparable, Entries,
    Filter, FolderContent, FormEntry, FormEntryContainer, Kennzahlen, Listable, MenuCategory,
    PlausibilityRules, PunkteKategorien, RefEntries, Script, Sortable,
};
use crate::model::{Haeufigkeiten, Ordner};
use crate::profile::Profile;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Unterformular {
    #[serde(rename = "DataCatalogues")]
    data_catalogues: DataCatalogues,
    #[serde(rename = "Category")]
    category: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Version")]
    version: String,
    #[serde(rename = "MenuEntry")]
    menu_entry: String,
    #[serde(rename = "Title")]
    title: String,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "Note")]
    note: String,
    #[serde(rename = "Readonly")]
    readonly: bool,
    #[serde(rename = "Active")]
    active: bool,
    #[serde(rename = "TudokPosition")]
    tudok_position: String,
    #[serde(rename = "Aktenbereich")]
    aktenbereich: String,
    #[serde(rename = "BefragungRelevant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    befragung_relevant: Option<bool>,
    #[serde(rename = "Hotkey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    hotkey: Option<String>,
    #[serde(rename = "Summary")]
    summary: String,
    #[serde(rename = "BigSummary")]
    big_summary: String,
    #[serde(rename = "KalenderSchnipsel")]
    kalender_schnipsel: String,
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
    kontext: String,
    #[serde(rename = "Datenart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    datenart: Option<String>,
    #[serde(rename = "ShowHistoryButton", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    show_history_button: Option<bool>,
    #[serde(rename = "TudokReadonly")]
    tudok_readonly: bool,
    #[serde(rename = "VitalstatusRelevant")]
    vitalstatus_relevant: bool,
    #[serde(rename = "AutoNummerierung")]
    auto_nummerierung: bool,
    #[serde(rename = "Zwischenspeichern")]
    zwischenspeichern: bool,
    #[serde(rename = "Zurueckblaettern")]
    zurueckblaettern: bool,
    #[serde(rename = "Datenbankexport")]
    datenbankexport: bool,
    #[serde(rename = "DatenschutzRelevant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    datenschutz_relevant: Option<bool>,
    #[serde(rename = "KonferenzRelevant")]
    konferenz_relevant: bool,
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
    entries: Entries<Entry>,
    #[serde(rename = "PlausibilityRules")]
    plausibility_rules: PlausibilityRules<DataFormEntries>,
    #[serde(rename = "Haeufigkeiten")]
    haeufigkeiten: Haeufigkeiten,
    #[serde(rename = "Kennzahlen")]
    kennzahlen: Kennzahlen,
    #[serde(rename = "Ordner")]
    ordner: Ordner,
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

impl FormEntryContainer for Unterformular {
    fn apply_profile(&mut self, profile: &Profile) {
        profile.forms.iter().for_each(|profile_form| {
            if self.name == profile_form.name {
                self.entries.entry.iter_mut().for_each(|entry| {
                    profile_form
                        .form_references
                        .iter()
                        .for_each(|form_reference| {
                            apply_profile_to_form_entry(entry, form_reference)
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
                })
            }
        });
    }
}

impl Listable for Unterformular {
    fn to_listed_string(&self) -> String {
        format!(
            "Unterformular ({}) '{}' in Revision '{}'",
            match self.is_system_library_content() {
                true => style("S").yellow(),
                _ => style("u"),
            },
            style(&self.name).yellow(),
            style(&self.revision).yellow(),
        )
    }
}

impl Sortable for Unterformular {
    fn sorting_key(&self) -> String {
        self.name.clone()
    }

    fn sorted(&mut self) -> &Self {
        self.data_catalogues.data_catalogue.sort_unstable();

        self.entries
            .entry
            .sort_unstable_by_key(|item| item.sorting_key());

        self.entries.entry.iter_mut().for_each(|item| {
            item.sorted();
        });

        if let Some(ref mut plausibility_rule) = self.plausibility_rules.plausibility_rule {
            plausibility_rule.sort_unstable_by_key(|item| item.bezeichnung.clone());

            plausibility_rule.iter_mut().for_each(|item| {
                if let Some(ref mut data_form_entry_names) = item.data_form_entries.entry_name {
                    data_form_entry_names.sort_unstable();
                }
            });
        }
        self
    }
}

impl Comparable for Unterformular {
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

impl Requires for Unterformular {
    fn requires_form_reference(&self, name: &str) -> bool {
        self.entries
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
            .last()
            .unwrap_or_default()
    }
    fn requires_subform(&self, name: &str) -> bool {
        self.entries
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
            .last()
            .unwrap_or_default()
    }

    fn get_required_entries<'a>(&'a self, all: &'a OnkostarEditor) -> Vec<Requirement> {
        let mut result = self
            .data_catalogues
            .data_catalogue
            .iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .map(|entry| match all.find_data_catalogue(entry.as_str()) {
                Some(contained) => Requirement::DataCatalogue(contained),
                None => Requirement::ExternalDataCatalogue(entry.to_string()),
            })
            .collect::<Vec<_>>();
        result.sort_unstable_by_key(|item| item.sorting_key());

        let referenced_forms = &mut self
            .entries
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
                    None => Requirement::ExternalUnterformularReference(entry.to_string()),
                },
            })
            .collect::<Vec<_>>();
        referenced_forms.sort_unstable_by_key(|item| item.sorting_key());
        result.append(referenced_forms);

        let sub_forms = &mut self
            .entries
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
                    None => Requirement::ExternalUnterformularSubform(entry.to_string()),
                },
            })
            .collect::<Vec<_>>();
        sub_forms.sort_unstable_by_key(|item| item.sorting_key());
        result.append(sub_forms);

        result
    }
}

impl FolderContent for Unterformular {
    fn get_library_folder(&self) -> String {
        self.ordner.bibliothek.name.to_string()
    }
}

impl Checkable for Unterformular {
    fn check(&self) -> Vec<CheckNotice> {
        if self.hat_unterformulare {
            return vec![ErrorWithCode {
                code: "2023-0001".to_string(),
                description: format!(
                    "Unterformular '{}' mit Markierung 'hat Unterformulare'",
                    self.name
                ),
                line: None,
                example: None,
            }];
        }

        vec![]
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
pub struct Entry {
    #[serde(rename = "@parentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_id: Option<u32>,
    #[serde(rename = "@parentRefId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_ref_id: Option<u32>,
    #[serde(rename = "Type")]
    type_: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "Active")]
    active: bool,
    #[serde(rename = "Readonly")]
    read_only: bool,
    #[serde(rename = "Printable")]
    printable: bool,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "Note")]
    note: String,
    #[serde(rename = "Beschriftung1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    beschriftung1: Option<String>,
    #[serde(rename = "Beschriftung2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    beschriftung2: Option<String>,
    #[serde(rename = "WertAnzeigenPatmodul")]
    #[serde(skip_serializing_if = "Option::is_none")]
    wert_anzeigen_patmodul: Option<String>,
    #[serde(rename = "MultipleChoice")]
    multiple_choice: bool,
    #[serde(rename = "DefaultValue")]
    default_value: String,
    #[serde(rename = "Alignment")]
    alignment: String,
    #[serde(rename = "Direction")]
    direction: String,
    #[serde(rename = "DataCatalogueEntry")]
    data_catalogue_entry: String,
    #[serde(rename = "DataCatalogueEntryTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    data_catalogue_entry_table: Option<String>,
    #[serde(rename = "ElementParent")]
    element_parent: String,
    #[serde(rename = "ProcedureDateStatus")]
    procedure_date_status: String,
    #[serde(rename = "ZuordnungErkrankung")]
    zuordnung_erkrankung: String,
    #[serde(rename = "Grafik")]
    #[serde(skip_serializing_if = "Option::is_none")]
    grafik: Option<String>,
    #[serde(rename = "GrafikAusrichtung")]
    #[serde(skip_serializing_if = "Option::is_none")]
    grafik_ausrichtung: Option<String>,
    #[serde(rename = "Mandatory")]
    mandatory: String,
    #[serde(rename = "Datenart", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    datenart: Option<String>,
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Filter>,
    #[serde(rename = "NotSpecified")]
    not_specified: bool,
    #[serde(rename = "Scripts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    scripts: Option<Script>,
    #[serde(rename = "ReferencedDataForm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    referenced_data_form: Option<String>,
    #[serde(rename = "ReferencedDataFormField")]
    #[serde(skip_serializing_if = "Option::is_none")]
    referenced_data_form_field: Option<String>,
    #[serde(rename = "Anzeige")]
    anzeige: String,
    #[serde(rename = "AnzeigeAuswahl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    anzeige_auswahl: Option<String>,
    #[serde(rename = "Druckvorlage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    druckvorlage: Option<String>,
    #[serde(rename = "VersionFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    version_from: Option<String>,
    #[serde(rename = "Speichern")]
    speichern: String,
    #[serde(rename = "LeerAusblenden")]
    leer_ausblenden: bool,
    #[serde(rename = "Inhalt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    inhalt: Option<String>,
    #[serde(rename = "GeschlossenAnzeigen")]
    geschlossen_anzeigen: bool,
    #[serde(rename = "Min")]
    min: u32,
    #[serde(rename = "Max")]
    max: u32,
    #[serde(rename = "InUebersichtAnzeigen")]
    in_uebersicht_anzeigen: bool,
    #[serde(rename = "Hinweis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    hinweis: Option<String>,
    #[serde(rename = "Vorschlagskategorie")]
    #[serde(skip_serializing_if = "Option::is_none")]
    vorschlagskategorie: Option<String>,
    #[serde(rename = "CategoryFilterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    category_filer_name: Option<String>,
    #[serde(rename = "Platzhalter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    platzhalter: Option<String>,
    #[serde(rename = "ShowAuspraegungen")]
    show_auspraegungen: bool,
    #[serde(rename = "ProzedurdatumUebernehmen")]
    prozedurdatum_uebernehmen: bool,
    #[serde(rename = "Vorschlaege")]
    vorschlaege: bool,
    #[serde(rename = "AnzeigeAuswahldialog")]
    anzeige_auswahldialog: String,
    #[serde(rename = "SucheAuswahldialog")]
    suche_auswahldialog: String,
    #[serde(rename = "InfoAuswahldialog")]
    #[serde(skip_serializing_if = "Option::is_none")]
    info_auswahldialog: Option<String>,
    #[serde(rename = "DiseaseCategoryFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    disease_category_filter: Option<String>,
    #[serde(rename = "MindestbreiteLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    mindestbreite_label: Option<u16>,
    #[serde(rename = "MindestbreiteFeld")]
    #[serde(skip_serializing_if = "Option::is_none")]
    mindestbreite_feld: Option<u16>,
    #[serde(rename = "OrganisationunitFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    ou_filter: Option<String>,
    #[serde(rename = "Aktion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    aktion: Option<String>,
    #[serde(rename = "Top10")]
    #[serde(skip_serializing_if = "Option::is_none")]
    top10: Option<String>,
    #[serde(rename = "GroesseTextfeld")]
    groesse_textfeld: u16,
    #[serde(rename = "FilterAufheben")]
    filter_aufheben: bool,
    #[serde(rename = "Resizable")]
    resizable: bool,
    #[serde(rename = "Verschluesselt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    verschluesselt: Option<bool>,
    #[serde(rename = "MemoWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    memo_width: Option<u32>,
    #[serde(rename = "MemoHeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    memo_height: Option<u32>,
    #[serde(rename = "MemoArt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    memo_art: Option<String>,
    #[serde(rename = "DateValidFrom")]
    date_valid_from: String,
    #[serde(rename = "DateValidTo")]
    date_valid_to: String,
    #[serde(rename = "DateValidFuture")]
    date_valid_future: bool,
    #[serde(rename = "Titel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    titel: Option<String>,
    #[serde(rename = "InAuswertung")]
    in_auswertung: bool,
    #[serde(rename = "InAuswertungGraph")]
    in_auswertung_graph: bool,
    #[serde(rename = "FragebogenItemNummer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    fragebogen_item_nummer: Option<u8>,
    #[serde(rename = "Score")]
    #[serde(skip_serializing_if = "Option::is_none")]
    score: Option<String>,
    #[serde(rename = "AlignmentPatModul")]
    alignment_pat_modul: String,
    #[serde(rename = "DirectionPatModul")]
    direction_pat_modul: String,
    #[serde(rename = "SeitenumbruchPatModul")]
    seitenumbruch_pat_modul: bool,
    #[serde(rename = "Kontaktliste")]
    #[serde(skip_serializing_if = "Option::is_none")]
    kontaktliste: Option<String>,
    #[serde(rename = "MarkierungIgnorieren")]
    markierung_ignorieren: bool,
    #[serde(rename = "SucheArt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    suche_art: Option<String>,
    #[serde(rename = "SID")]
    sid: String,
    #[serde(rename = "GUID")]
    guid: String,
    #[serde(rename = "Revision")]
    revision: u16,
    #[serde(rename = "vorherigeWerte")]
    #[serde(skip_serializing_if = "Option::is_none")]
    vorherige_werte: Option<String>,
    #[serde(rename = "EinfuegenVerhindern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    einfuegen_verhindern: Option<String>,
}

impl FormEntry for Entry {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_type(&self) -> String {
        self.type_.clone()
    }

    fn update_referenced_data_form(&mut self, value: String) {
        self.referenced_data_form = Some(value);
    }

    fn update_anzeige(&mut self, value: String) {
        self.anzeige = value;
    }

    fn update_anzeige_auswahl(&mut self, value: String) {
        self.anzeige_auswahl = Some(value);
    }

    fn update_scripts_code(&mut self, value: String) {
        self.scripts = Some(Script {
            code: value,
            valid: true,
        });
    }

    fn update_default_value(&mut self, value: String) {
        self.default_value = value
    }

    fn hide(&mut self) {
        self.filter = Some(Filter {
            condition: "false".into(),
            valid: true,
            ref_entries: Some(RefEntries { ref_entry: None }),
        });
        self.speichern = "0".into()
    }

    fn remove_filter(&mut self) {
        self.filter = None;
    }
}

impl Sortable for Entry {
    fn sorting_key(&self) -> String {
        self.name.clone()
    }

    fn sorted(&mut self) -> &Self
    where
        Self: Sized,
    {
        if let Some(ref mut filter) = self.filter {
            if let Some(ref mut ref_entries) = filter.ref_entries {
                if let Some(ref mut ref_entry) = ref_entries.ref_entry {
                    ref_entry.sort_unstable()
                }
            }
        }
        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct DataFormEntries {
    #[serde(rename = "EntryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    entry_name: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::model::onkostar_editor::OnkostarEditor;
    use crate::model::{Filter, RefEntries, Script};
    use crate::profile::Profile;

    #[test]
    fn should_change_dataform_entry_default_value() {
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

        assert_eq!(
            onkostar_editor.editor.unterformular[0].entries.entry[1].name,
            "Termin"
        );
        assert_eq!(
            onkostar_editor.editor.unterformular[0].entries.entry[1].default_value,
            ""
        );

        onkostar_editor.apply_profile(&profile);

        assert_eq!(
            onkostar_editor.editor.unterformular[0].entries.entry[1].name,
            "Termin"
        );
        assert_eq!(
            onkostar_editor.editor.unterformular[0].entries.entry[1].default_value,
            "2024-03-18"
        )
    }

    #[test]
    fn should_not_change_dataform_entry_default_value() {
        let onkostar_editor = OnkostarEditor::from_str(include_str!("../../tests/test.osc"));

        assert!(onkostar_editor.is_ok());
        let mut onkostar_editor = onkostar_editor.unwrap();

        let profile = "forms:
               - name: 'Unterformular'
            ";

        let profile = Profile::from_str(profile);
        assert!(profile.is_ok());
        let profile = profile.unwrap();

        assert_eq!(
            onkostar_editor.editor.unterformular[0].entries.entry[1].name,
            "Termin"
        );
        assert_eq!(
            onkostar_editor.editor.unterformular[0].entries.entry[1].default_value,
            ""
        );

        onkostar_editor.apply_profile(&profile);

        assert_eq!(
            onkostar_editor.editor.unterformular[0].entries.entry[1].name,
            "Termin"
        );
        assert_eq!(
            onkostar_editor.editor.unterformular[0].entries.entry[1].default_value,
            ""
        )
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

        assert!(&onkostar_editor.editor.unterformular[0]
            .menu_category
            .is_none());
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

        assert_eq!(
            onkostar_editor.editor.unterformular[0].entries.entry[1].scripts,
            None
        );

        onkostar_editor.apply_profile(&profile);

        assert_eq!(
            onkostar_editor.editor.unterformular[0].entries.entry[1].scripts,
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

        assert_eq!(
            onkostar_editor.editor.unterformular[0].entries.entry[1].scripts,
            None
        );

        onkostar_editor.apply_profile(&profile);

        assert_eq!(
            onkostar_editor.editor.unterformular[0].entries.entry[1].scripts,
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

        assert_eq!(
            onkostar_editor.editor.unterformular[0].entries.entry[0].filter,
            None
        );

        assert_eq!(
            onkostar_editor.editor.unterformular[0].entries.entry[1].filter,
            Some(Filter {
                condition: "getGlobalSetting('mehrere_mtb_in_mtbepisode') = 'true'".into(),
                valid: true,
                ref_entries: Some(RefEntries { ref_entry: None })
            })
        );

        onkostar_editor.apply_profile(&profile);

        assert_eq!(
            onkostar_editor.editor.unterformular[0].entries.entry[0].filter,
            None
        );

        assert_eq!(
            onkostar_editor.editor.unterformular[0].entries.entry[1].filter,
            None
        );
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

        assert_eq!(
            onkostar_editor.editor.unterformular[0].entries.entry[0].filter,
            None
        );

        assert_eq!(
            onkostar_editor.editor.unterformular[0].entries.entry[1].filter,
            Some(Filter {
                condition: "getGlobalSetting('mehrere_mtb_in_mtbepisode') = 'true'".into(),
                valid: true,
                ref_entries: Some(RefEntries { ref_entry: None })
            })
        );

        onkostar_editor.apply_profile(&profile);

        assert_eq!(
            onkostar_editor.editor.unterformular[0].entries.entry[0].filter,
            None
        );

        assert_eq!(
            onkostar_editor.editor.unterformular[0].entries.entry[1].filter,
            None
        );
    }
}
