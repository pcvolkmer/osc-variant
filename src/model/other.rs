/*
 * MIT License
 *
 * Copyright (c) 2024 Comprehensive Cancer Center Mainfranken
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

use crate::model::{Ansicht, Ordner};
use serde::{Deserialize, Serialize};

// Ablaufschema ...
#[derive(Debug, Deserialize, Serialize)]
pub struct Ablaufschema {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Beschreibung")]
    pub beschreibung: String,
    #[serde(rename = "AblaufschemaFormulars")]
    pub ablaufschema_formulars: Vec<AblaufschemaFormular>,
    #[serde(rename = "AblaufschemaElements")]
    pub ablaufschema_elements: Vec<AblaufschemaElement>,
    #[serde(rename = "Personenstamms")]
    pub personenstamms: Vec<String>,
    #[serde(rename = "Ordner")]
    pub ordner: Ordner,
    #[serde(rename = "Aktiv")]
    pub aktiv: bool,
    #[serde(rename = "ReadOnly")]
    pub read_only: bool,
    #[serde(rename = "AusfuehrungIntervall")]
    pub ausfuehrung_intervall: i32,
    #[serde(rename = "Uhrzeit")]
    pub uhrzeit: String,
    #[serde(rename = "SID")]
    pub sid: i32,
    #[serde(rename = "GUID")]
    pub guid: String,
    #[serde(rename = "Revision")]
    pub revision: i32,
    #[serde(rename = "MeldungID")]
    pub meldung_id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AblaufschemaFormular {
    #[serde(rename = "DataFormName")]
    pub data_form_name: String,
    #[serde(rename = "DataFormSID")]
    pub data_form_sid: i32,
    #[serde(rename = "DataFormGUID")]
    pub data_form_guid: String,
    #[serde(rename = "Vorbedingung")]
    pub vorbedingung: String,
    #[serde(rename = "VorbedingungGueltig")]
    pub vorbedingung_gueltig: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AblaufschemaElement {
    #[serde(rename = "Typ")]
    pub typ: i32,
    #[serde(rename = "Beschreibung")]
    pub beschreibung: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Nummer")]
    pub nummer: i32,
    #[serde(rename = "Vorbedingung")]
    pub vorbedingung: String,
    #[serde(rename = "VorbedingungGueltig")]
    pub vorbedingung_gueltig: bool,
    #[serde(rename = "Uebergangsart")]
    pub uebergangsart: i32,
    #[serde(rename = "SID")]
    pub sid: i32,
    #[serde(rename = "GUID")]
    pub guid: String,
    #[serde(rename = "Revision")]
    pub revision: i32,
    #[serde(rename = "AktionTyp")]
    pub aktion_typ: i32,
    #[serde(rename = "Parameter")]
    pub parameter: String,
    #[serde(rename = "Sichtbar")]
    pub sichtbar: bool,
    #[serde(rename = "StartElementSID")]
    pub start_element_sid: i32,
    #[serde(rename = "StartElementGUID")]
    pub start_element_guid: String,
    #[serde(rename = "EndElementSID")]
    pub end_element_sid: i32,
    #[serde(rename = "EndElementGUID")]
    pub end_element_guid: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Akte {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Beschreibung")]
    pub beschreibung: Option<String>,
    #[serde(rename = "Ordner")]
    pub ordner: Ordner,
    #[serde(rename = "Aktiv")]
    pub aktiv: bool,
    #[serde(rename = "Kontext")]
    pub kontext: Option<String>,
    #[serde(rename = "SID")]
    pub sid: Option<i32>,
    #[serde(rename = "GUID")]
    pub guid: Option<String>,
    #[serde(rename = "Revision")]
    pub revision: Option<i32>,
    #[serde(rename = "Modul")]
    pub modul: Vec<Modul>,
    #[serde(rename = "BerechtigungenAktiv")]
    pub berechtigungen_aktiv: Option<bool>,
    #[serde(rename = "AkteRolle")]
    pub akte_rolle: Vec<AkteRolle>,
    #[serde(rename = "OffeneProzedurenReiterAktiv")]
    pub offene_prozeduren_reiter_aktiv: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AkteRolle {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Beschreibung")]
    pub beschreibung: Option<String>,
    #[serde(rename = "ModulBerechtigung")]
    pub modul_berechtigung: Vec<ModulBerechtigung>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecordLinkage {
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "UntereGrenze")]
    pub untere_grenze: i32,
    #[serde(rename = "ObereGrenze")]
    pub obere_grenze: i32,
    #[serde(rename = "Verwenden")]
    pub verwenden: bool,
    #[serde(rename = "RecordLinkageAbgleichvariablen")]
    pub record_linkage_abgleichvariablen: Vec<RecordLinkageAbgleichvariablen>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecordLinkageAbgleichvariablen {
    #[serde(rename = "Abgleichvariable")]
    pub abgleichvariable: i32,
    #[serde(rename = "Gewichtung")]
    pub gewichtung: f64,
    #[serde(rename = "Verwenden")]
    pub verwenden: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Rskript {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Titel")]
    pub titel: Option<String>,
    #[serde(rename = "Skript")]
    pub skript: Option<String>,
    #[serde(rename = "Bemerkung")]
    pub bemerkung: String,
    #[serde(rename = "Felder")]
    pub felder: Option<String>,
    #[serde(rename = "Ordner")]
    pub ordner: Ordner,
    #[serde(rename = "SID")]
    pub sid: Option<i32>,
    #[serde(rename = "GUID")]
    pub guid: Option<String>,
    #[serde(rename = "Revision")]
    pub revision: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SidGuid {
    #[serde(rename = "Sid")]
    pub sid: i32,
    #[serde(rename = "Guid")]
    pub guid: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Modul {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Beschreibung")]
    pub beschreibung: Option<String>,
    #[serde(rename = "MenuEntry")]
    pub menu_entry: Option<String>,
    #[serde(rename = "Typ")]
    pub typ: i32,
    #[serde(rename = "Position")]
    pub position: f64,
    #[serde(rename = "Konfiguration")]
    pub konfiguration: Option<String>,
    #[serde(rename = "SID")]
    pub sid: Option<i32>,
    #[serde(rename = "GUID")]
    pub guid: Option<String>,
    #[serde(rename = "Revision")]
    pub revision: Option<i32>,
    #[serde(rename = "Formular")]
    pub formular: Vec<ModulFormular>,
    #[serde(rename = "Ansicht")]
    pub ansicht: Option<Ansicht>,
    #[serde(rename = "ElementParentGUID")]
    pub element_parent_guid: Option<String>,
    #[serde(rename = "GeoeffnetAnzeigen")]
    pub geoeffnet_anzeigen: Option<bool>,
    #[serde(rename = "AbAufschliessenAktiv")]
    pub ab_aufschliessen_aktiv: Option<bool>,
    #[serde(rename = "AnmerkungenAktiv")]
    pub anmerkungen_aktiv: Option<bool>,
    #[serde(rename = "AufgabenlisteAktiv")]
    pub aufgabenliste_aktiv: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModulBerechtigung {
    #[serde(rename = "ModulName")]
    pub modul_name: String,
    #[serde(rename = "anzeigen")]
    pub anzeigen: bool,
    #[serde(rename = "anlegen")]
    pub anlegen: bool,
    #[serde(rename = "bearbeiten")]
    pub bearbeiten: bool,
    #[serde(rename = "loeschen")]
    pub loeschen: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModulFormular {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "SID")]
    pub sid: i32,
    #[serde(rename = "GUID")]
    pub guid: String,
}
