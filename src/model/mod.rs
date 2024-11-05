/*
 * This file is part of osc-variant
 *
 * Copyright (C) 2024 the original author or authors.
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

use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

use crate::model::requirements::Requires;
use crate::profile::{FormField, FormReference, Profile, WithScriptsCode};

pub mod data_catalogue;
pub mod form;
pub mod onkostar_editor;
pub mod other;
pub mod property_catalogue;
pub mod requirements;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Script {
    #[serde(rename = "Code")]
    code: String,
    #[serde(rename = "Valid")]
    valid: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<String>,
    #[serde(rename = "ConditionValid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    condition_valid: Option<bool>,
    #[serde(rename = "Statusauswirkung")]
    statusauswirkung: String,
    #[serde(rename = "DataFormEntries")]
    data_form_entries: T,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Entries<T> {
    #[serde(rename = "Entry")]
    entry: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Ansicht {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Beschreibung")]
    beschreibung: String,
    #[serde(rename = "Konfiguration")]
    konfiguration: String,
    #[serde(rename = "DataForm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    data_form: Option<String>,
    #[serde(rename = "DataCatalogue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    data_catalogue: Option<String>,
    #[serde(rename = "TypAuswahl")]
    typ_auswahl: String,
    #[serde(rename = "PersonenstammKontext", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    personenstamm_kontext: Option<String>,
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
#[serde(deny_unknown_fields)]
pub struct Ansichten {
    #[serde(rename = "Ansicht", default)]
    program_module: Vec<Ansicht>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct MenuCategory {
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "position")]
    position: String,
    #[serde(rename = "column")]
    column: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PunkteKategorien {
    #[serde(rename = "PunkteKategorie", default)]
    punkte_kategorie: Vec<PunkteKategorie>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PunkteKategorie {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Beschreibung")]
    beschreibung: String,
    #[serde(rename = "MaxLeerwerte")]
    max_leerwerte: u16,
    #[serde(rename = "Berechnung")]
    berechnung: String,
    #[serde(rename = "Felder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    felder: Option<Felder>,
    #[serde(rename = "Vergleichswerttabellen")]
    vergleichswerttabellen: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Felder {
    #[serde(rename = "Feld", default)]
    feld: Vec<Feld>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Feld {
    #[serde(rename = "DataFormEntryName")]
    data_form_entry_name: String,
    #[serde(rename = "ManuellePunkte")]
    manuelle_punkte: bool,
    #[serde(rename = "Werte")]
    #[serde(skip_serializing_if = "Option::is_none")]
    werte: Option<FeldWerte>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct FeldWerte {
    #[serde(rename = "Wert", default)]
    wert: Vec<FeldWert>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct FeldWert {
    #[serde(rename = "Wert")]
    wert: String,
    #[serde(rename = "Punkte")]
    punkte: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Filter {
    #[serde(rename = "Condition")]
    condition: String,
    #[serde(rename = "Valid")]
    valid: bool,
    #[serde(rename = "RefEntries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    ref_entries: Option<RefEntries>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RefEntries {
    #[serde(rename = "RefEntry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    ref_entry: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PlausibilityRules<T> {
    #[serde(rename = "PlausibilityRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    plausibility_rule: Option<Vec<PlausibilityRule<T>>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Haeufigkeiten {
    #[serde(rename = "Haeufigkeit", default)]
    haeufigkeit: Vec<Haeufigkeit>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Haeufigkeit {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Beschreibung")]
    beschreibung: String,
    #[serde(rename = "Notiz")]
    notiz: String,
    #[serde(rename = "Status")]
    status: bool,
    #[serde(rename = "Formel")]
    formel: String,
    #[serde(rename = "Analysezweck")]
    analysezweck: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "Ueberschrift")]
    ueberschrift: String,
    #[serde(rename = "TaeglichAktualisieren")]
    taeglich_aktualisieren: bool,
    #[serde(rename = "Typ")]
    typ: String,
    #[serde(rename = "NichtBerechnen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    nicht_berechnen: Option<String>,
    #[serde(rename = "TabellenName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    tabellen_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Bibliothek {
    #[serde(rename = "Name")]
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Ordner {
    #[serde(rename = "Bibliothek")]
    bibliothek: Bibliothek,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Typ")]
    typ: String,
    #[serde(rename = "ParentOrdner", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_order: Option<Box<Ordner>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Kennzahlen {
    #[serde(rename = "Kennzahl", default)]
    kennzahl: Vec<Kennzahl>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Kennzahl {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Nummer")]
    nummer: String,
    #[serde(rename = "Beschreibung")]
    beschreibung: String,
    #[serde(rename = "Notiz")]
    notiz: String,
    #[serde(rename = "Vorgabe")]
    vorgabe: String,
    #[serde(rename = "Haeufigkeitenzaehler")]
    haeufigkeitenzaehler: String,
    #[serde(rename = "Haeufigkeitennenner")]
    haeufigkeitennenner: String,
}

fn apply_profile_to_form_entry<E>(entry: &mut E, form_reference: &FormReference)
where
    E: FormEntry,
{
    if entry.get_type() == "formReference" && entry.get_name() == form_reference.name {
        if let Some(profile_referenced_data_form) = &form_reference.referenced_data_form {
            entry.update_referenced_data_form(profile_referenced_data_form.clone());
        }
        if let Some(profile_anzeige) = &form_reference.anzeige {
            entry.update_anzeige(profile_anzeige.clone());
        }
        if let Some(profile_anzeige_auswahl) = &form_reference.anzeige_auswahl {
            entry.update_anzeige_auswahl(profile_anzeige_auswahl.clone());
        }
        if let Some(scripts_code) = &form_reference.escaped_scripts_code() {
            entry.update_scripts_code(scripts_code.clone());
        }
        if form_reference.remove_filter {
            entry.remove_filter()
        }
    }
}

fn apply_profile_to_form_field<E>(entry: &mut E, form_field: &FormField)
where
    E: FormEntry,
{
    if entry.get_name() == form_field.name {
        if form_field.hide {
            entry.hide()
        }
        if let Some(new_default_value) = &form_field.default_value {
            entry.update_default_value(new_default_value.to_string())
        }
        if let Some(scripts_code) = &form_field.escaped_scripts_code() {
            entry.update_scripts_code(scripts_code.clone());
        }
        if form_field.remove_filter {
            entry.remove_filter()
        }
    }
}

pub trait FormEntryContainer {
    fn apply_profile(&mut self, profile: &Profile);
}

pub trait Listable {
    fn to_listed_string(&self) -> String;
}

pub trait Sortable {
    fn sorting_key(&self) -> String;

    fn sorted(&mut self) -> &Self
    where
        Self: Sized,
    {
        self
    }
}

pub trait Comparable: Debug {
    fn get_name(&self) -> String;
    fn get_revision(&self) -> u16;
    fn get_hash(&self) -> String {
        let mut h = DefaultHasher::new();
        format!("{:?}", self).hash(&mut h);
        h.finish().to_string()
    }
    fn compare_by_requirement(_: &Self, _: &Self) -> Ordering
    where
        Self: Requires,
    {
        Ordering::Equal
    }
}

pub trait FormEntry {
    fn get_name(&self) -> String;
    fn get_type(&self) -> String;
    fn update_referenced_data_form(&mut self, value: String);
    fn update_anzeige(&mut self, value: String);
    fn update_anzeige_auswahl(&mut self, value: String);
    fn update_scripts_code(&mut self, value: String);
    fn update_default_value(&mut self, value: String);
    fn hide(&mut self);
    fn remove_filter(&mut self);
}

pub trait FolderContent {
    fn get_library_folder(&self) -> String;

    fn is_system_library_content(&self) -> bool {
        "ONKOSTAR Bibliothek" == self.get_library_folder()
    }
}
