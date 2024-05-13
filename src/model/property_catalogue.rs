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

use console::style;
use serde::{Deserialize, Serialize};

use crate::model::{Comparable, FolderContent, Listable, Ordner, Sortable};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PropertyCatalogue {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(rename = "Standard")]
    standard: String,
    #[serde(rename = "Readonly")]
    readonly: bool,
    #[serde(rename = "Anmerkung")]
    #[serde(skip_serializing_if = "Option::is_none")]
    anmerkung: Option<String>,
    #[serde(rename = "SID")]
    sid: String,
    #[serde(rename = "GUID")]
    guid: String,
    #[serde(rename = "Revision")]
    revision: u16,
    #[serde(rename = "Versions")]
    versions: Versions,
    #[serde(rename = "Ordner")]
    ordner: Ordner,
}

impl Listable for PropertyCatalogue {
    fn to_listed_string(&self) -> String {
        format!(
            "Merkmalskatalog ({}) '{}' in Revision '{}'",
            match self.is_system_library_content() {
                true => style("S").yellow(),
                _ => style("u"),
            },
            style(&self.name).yellow(),
            style(&self.revision).yellow()
        )
    }
}

impl Sortable for PropertyCatalogue {
    fn sorting_key(&self) -> String {
        self.name.clone()
    }

    fn sorted(&mut self) -> &Self {
        if let Some(ref mut versions) = self.versions.entry {
            versions.sort_unstable_by_key(|item| item.version_number);
            versions.iter_mut().for_each(|version| {
                version.sorted();
            });
        }
        self
    }
}

impl Comparable for PropertyCatalogue {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_revision(&self) -> u16 {
        self.revision
    }
}

impl FolderContent for PropertyCatalogue {
    fn get_library_folder(&self) -> String {
        self.ordner.bibliothek.name.to_string()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Versions {
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    entry: Option<Vec<Version>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Version {
    #[serde(rename = "VersionNumber")]
    version_number: u16,
    #[serde(rename = "ValidFrom")]
    valid_from: String,
    #[serde(rename = "OID")]
    oid: String,
    #[serde(rename = "Active")]
    active: bool,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "SID")]
    sid: String,
    #[serde(rename = "GUID")]
    guid: String,
    #[serde(rename = "Revision")]
    revision: u16,
    #[serde(rename = "Entries")]
    entries: Option<VersionEntries>,
    #[serde(rename = "Abbildung")]
    #[serde(skip_serializing_if = "Option::is_none")]
    abbildung: Option<Vec<Abbildung>>,
    #[serde(rename = "Categories")]
    categories: Option<Categories>,
}

impl Sortable for Version {
    fn sorting_key(&self) -> String {
        self.oid.clone()
    }

    fn sorted(&mut self) -> &Self
    where
        Self: Sized,
    {
        if let Some(ref mut abbildung) = self.abbildung {
            abbildung.sort_unstable_by_key(|item| item.sorting_key());
            abbildung.iter_mut().for_each(|item| {
                item.sorted();
            });
        }

        if let Some(ref mut entries) = self.entries {
            entries
                .content
                .sort_unstable_by_key(|item| item.sorting_key());
            entries.content.iter_mut().for_each(|item| {
                item.sorted();
            });
        }

        if let Some(ref mut categories) = self.categories {
            categories
                .content
                .sort_unstable_by_key(|item| item.sorting_key());
            categories.content.iter_mut().for_each(|item| {
                item.sorted();
            });
        }

        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct VersionEntries {
    #[serde(rename = "Entry", default)]
    content: Vec<VersionEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct VersionEntry {
    #[serde(rename = "Code")]
    code: String,
    #[serde(rename = "ShortDescription")]
    short_description: String,
    #[serde(rename = "Description")]
    description: Option<String>,
    #[serde(rename = "Synonyms", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    synonyms: Option<String>,
    #[serde(rename = "Note", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<String>,
    #[serde(rename = "Type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<String>,
    #[serde(rename = "Position")]
    position: String,
}

impl Sortable for VersionEntry {
    fn sorting_key(&self) -> String {
        self.code.clone()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Categories {
    #[serde(rename = "Category", default)]
    content: Vec<Category>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Category {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "Auswahldialog")]
    auswahldialog: bool,
    #[serde(rename = "EntryCount")]
    entry_count: u16,
    #[serde(rename = "Beschreibung")]
    beschreibung: String,
    #[serde(rename = "CategoryEntries")]
    category_entries: CategoryEntries,
}

impl Sortable for Category {
    fn sorting_key(&self) -> String {
        self.name.clone()
    }

    fn sorted(&mut self) -> &Self
    where
        Self: Sized,
    {
        self.category_entries
            .content
            .sort_unstable_by_key(|item| item.sorting_key());
        self.category_entries.content.iter_mut().for_each(|item| {
            item.sorted();
        });

        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CategoryEntries {
    #[serde(rename = "CategoryEntry", default)]
    content: Vec<CategoryEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CategoryEntry {
    #[serde(rename = "Code")]
    code: String,
    #[serde(rename = "shortdesc", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    shortdesc: Option<String>,
    #[serde(rename = "description", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(rename = "Synonyms", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    synonyms: Option<String>,
    #[serde(rename = "note", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<String>,
}

impl Sortable for CategoryEntry {
    fn sorting_key(&self) -> String {
        self.code.clone()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Abbildung {
    #[serde(rename = "ZielMKVersionOid")]
    ziel_mk_version_oid: String,
    #[serde(rename = "Eintrag", default)]
    content: Vec<AbbildungEintrag>,
}

impl Sortable for Abbildung {
    fn sorting_key(&self) -> String {
        self.ziel_mk_version_oid.clone()
    }

    fn sorted(&mut self) -> &Self
    where
        Self: Sized,
    {
        self.content.sort_unstable_by_key(|item| item.sorting_key());
        self.content.iter_mut().for_each(|item| {
            item.sorted();
        });

        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct AbbildungEintrag {
    #[serde(rename = "Entry-from")]
    entry_from: AbbildungEntry,
    #[serde(rename = "Entry-to")]
    entry_to: AbbildungEntry,
}

impl Sortable for AbbildungEintrag {
    fn sorting_key(&self) -> String {
        format!("{}-{}", self.entry_from.code, self.entry_to.code)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct AbbildungEntry {
    #[serde(rename = "Code")]
    code: String,
    #[serde(rename = "ShortDescription")]
    short_description: String,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "Synonyms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    synonyms: Option<String>,
    #[serde(rename = "Note", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<String>,
    #[serde(rename = "Position")]
    position: String,
}
