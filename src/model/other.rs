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

use crate::model::{Ansicht, Filter, FormEntry, Ordner, RefEntries, Script, Sortable};
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
    pub type_: String,
    #[serde(rename = "Name")]
    pub name: String,
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
    pub default_value: String,
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
    pub procedure_date_status: String,
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
    pub filter: Option<Filter>,
    #[serde(rename = "NotSpecified")]
    not_specified: bool,
    #[serde(rename = "Scripts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scripts: Option<Script>,
    #[serde(rename = "ReferencedDataForm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_data_form: Option<String>,
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
