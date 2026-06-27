/*
 * This file is part of osc-variant
 *
 * Copyright (C) 2026 the original author or authors.
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

use git2::build::CheckoutBuilder;
use model::osc::Comparable;
use model::osc::data_catalogue::DataCatalogue;
use model::osc::form::{DataFormType, Form, UnterformularType};
use model::osc::onkostar_editor::{Editor, InfoXML, OnkostarEditor};
use model::osc::property_catalogue::PropertyCatalogue;
use regex::Regex;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clone)]
pub struct BundleVersionSpec {
    pub bundle_name: String,
    pub version_tag: Option<String>,
}

impl FromStr for BundleVersionSpec {
    type Err = String;

    #[allow(clippy::expect_used)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('@');
        let bundle_name = parts.next().ok_or("Bundle-Name fehlt")?;
        let version_tag = parts.next().map(ToString::to_string);
        Ok(BundleVersionSpec {
            bundle_name: bundle_name.to_string(),
            version_tag: if let Some(version_tag) = version_tag {
                // Ensure strict semver as default
                let numbers = Regex::new(r"^\d").expect("Regex fehlerhaft");
                if numbers.is_match(&version_tag) {
                    Some(format!("={version_tag}"))
                } else {
                    Some(version_tag)
                }
            } else {
                None
            },
        })
    }
}

trait BundleableInfoXML {
    fn from_bundle_version(version: &BundleVersion) -> InfoXML;
}

impl BundleableInfoXML for InfoXML {
    fn from_bundle_version(bundle_version: &BundleVersion) -> Self {
        Self {
            datum_xml: bundle_version.info_xml.datum_xml.clone(),
            name: bundle_version.info_xml.name.clone(),
            version: bundle_version.info_xml.version.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Index {
    bundles: Vec<Bundle>,
}

#[derive(Serialize, Deserialize)]
struct Bundle {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    license: Option<String>,
    repository: Option<String>,
    versions: Vec<BundleVersion>,
}

#[derive(Serialize, Deserialize)]
pub struct BundleVersion {
    id: String,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    license: Option<String>,
    pub info_xml: InfoXML,
}

#[derive(Serialize, Deserialize)]
struct BundleVersionContent {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<String>,
    property_catalogues: Vec<Object>,
    data_catalogues: Vec<Object>,
    data_forms: Vec<Object>,
    sub_forms: Vec<Object>,
}

impl BundleVersionContent {
    fn get_hash(&self) -> String {
        let json = serde_json::to_string_pretty(&self)
            .map_err(|err| err.to_string())
            .unwrap_or_default();
        let mut hasher = Sha256::new();
        hasher.update(&json);
        let hash = hasher.finalize();
        base16ct::lower::encode_string(&hash)
    }
}

#[derive(Serialize, Deserialize)]
struct Object {
    guid: String,
    checksum: String,
    name: String,
    revision: u16,
}

pub enum BundleError {
    InitializationError,
    UpdateError,
    Other(String),
}

pub struct BundleInfo {
    pub name: String,
    pub version: String,
    pub latest_version: String,
    pub info_xml: InfoXML,
    pub description: Option<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub versions: Vec<BundleVersionInfo>,
}

pub struct BundleVersionInfo {
    pub name: String,
    pub version: String,
    pub license: String,
}

impl Debug for BundleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for BundleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BundleError::InitializationError => {
                write!(f, "Fehler beim Initialisieren des Bundles-Repositorys")
            }
            BundleError::UpdateError => write!(
                f,
                "Fehler beim Aktualisieren des Bundles-Repositorys. Verbleibe auf altem Stand."
            ),
            BundleError::Other(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for BundleError {}

fn get_repo_path(path: &str) -> PathBuf {
    if let Ok(repo_dir) = std::env::var("OSC_VARIANT_REPO_DIR") {
        PathBuf::from(format!("{repo_dir}/repo/{path}"))
    } else {
        PathBuf::from(format!(
            "{}/.osc-variant/repo/{path}",
            dirs::home_dir().unwrap_or_default().display()
        ))
    }
}

pub fn update_bundle_repo() -> Result<(), BundleError> {
    if let Ok(repo) = git2::Repository::open(get_repo_path("")) {
        let mut remote = repo
            .find_remote("origin")
            .map_err(|_| BundleError::UpdateError)?;
        remote
            .fetch(&["main"], None, None)
            .map_err(|_| BundleError::UpdateError)?;

        let fetch_head = repo
            .find_reference("refs/remotes/origin/main")
            .map_err(|_| BundleError::UpdateError)?;
        let commit = repo
            .reference_to_annotated_commit(&fetch_head)
            .map_err(|_| BundleError::UpdateError)?;
        let mut reference = repo
            .find_reference("refs/heads/main")
            .map_err(|_| BundleError::UpdateError)?;
        reference
            .set_target(commit.id(), "main")
            .map_err(|_| BundleError::UpdateError)?;
        repo.set_head("refs/heads/main")
            .map_err(|_| BundleError::UpdateError)?;
        repo.checkout_head(Some(CheckoutBuilder::default().force()))
            .map_err(|_| BundleError::UpdateError)?;
    } else {
        git2::Repository::clone(
            "https://git.dnpm.dev/public/os-forms.git",
            get_repo_path(""),
        )
        .map_err(|_| BundleError::InitializationError)?;
    }

    Ok(())
}

fn read_index_or_empty() -> Result<Index, BundleError> {
    match fs::read_to_string(get_repo_path("/index.json")) {
        Ok(json) => match serde_json::from_str(&json) {
            Ok(index) => Ok(index),
            Err(err) => Err(BundleError::Other(err.to_string())),
        },
        Err(_) => Ok(Index { bundles: vec![] }),
    }
}

fn add_item<T>(item: &T) -> Result<Object, ()>
where
    T: Comparable + Serialize,
{
    let guid = item.get_guid();
    let name = item.get_name();
    let revision = item.get_revision();
    if guid.is_empty() {
        return Err(());
    }
    let value = serde_json::to_value(item).map_err(|_| ())?;
    let json = serde_json::to_string_pretty(&value).map_err(|_| ())?;

    let mut hasher = Sha256::new();
    hasher.update(&json);
    let hash = hasher.finalize();
    let checksum = base16ct::lower::encode_string(&hash);

    fs::write(get_repo_path(&format!("/objects/{checksum}.json")), json).map_err(|_| ())?;
    Ok(Object {
        guid,
        checksum,
        name,
        revision,
    })
}

#[allow(clippy::expect_used)]
pub fn create_bundle(
    name: &str,
    description: &str,
    license: Option<String>,
    repository: Option<String>,
) -> Result<(), BundleError> {
    let regex = Regex::new(r"^[a-zA-Z0-9_\-]{5,24}$").expect("Invalid regex");
    if !regex.is_match(name) {
        return Err(BundleError::Other("Der Bundle-Name muss zwischen 5 und 24 Zeichen lang sein und darf nur aus Buchstaben, Zahlen und Unter- und Bindestrichen bestehen".to_string()));
    }

    let mut index = read_index_or_empty()?;

    if index.bundles.iter().any(|b| b.name == name) {
        return Err(BundleError::Other(format!(
            "Bundle '{name}' existiert bereits"
        )));
    }

    index.bundles.push(Bundle {
        name: name.to_string(),
        description: if description.is_empty() {
            None
        } else {
            Some(description.to_string())
        },
        license,
        repository,
        versions: vec![],
    });

    let json =
        serde_json::to_string_pretty(&index).map_err(|err| BundleError::Other(err.to_string()))?;
    fs::write(get_repo_path("/index.json"), json)
        .map_err(|err| BundleError::Other(err.to_string()))?;

    Ok(())
}

pub fn add_bundle_version(
    name: &str,
    data: &mut OnkostarEditor,
    tag: Option<String>,
    message: Option<String>,
    license: Option<String>,
) -> Result<(), BundleError> {
    if let Some(tag) = &tag
        && Version::parse(tag).is_err()
    {
        return Err(BundleError::Other(
            "Versions-Tag muss eine SemVer-Version sein".to_string(),
        ));
    }

    let mut index = read_index_or_empty()?;

    let Some(bundle) = index.bundles.iter_mut().find(|b| b.name == name) else {
        return Err(BundleError::Other(format!(
            "Bundle '{name}' existiert noch nicht"
        )));
    };

    if bundle
        .versions
        .iter()
        .any(|bundle_tag| bundle_tag.tag.is_some() && bundle_tag.tag == tag)
    {
        return Err(BundleError::Other(format!(
            "Versions-Tag '{}' des Bundles '{}' existiert bereits",
            tag.unwrap_or_default(),
            name
        )));
    }

    let mut bundle_version_content = BundleVersionContent {
        name: name.to_string(),
        tag,
        property_catalogues: vec![],
        data_catalogues: vec![],
        data_forms: vec![],
        sub_forms: vec![],
    };

    for item in &data.editor.property_catalogue {
        if let Ok(index_entry) = add_item(item) {
            bundle_version_content.property_catalogues.push(index_entry);
        }
    }

    for item in &data.editor.data_catalogue {
        if let Ok(index_entry) = add_item(item) {
            bundle_version_content.data_catalogues.push(index_entry);
        }
    }

    for item in &data.editor.data_form {
        if let Ok(index_entry) = add_item(item) {
            bundle_version_content.data_forms.push(index_entry);
        }
    }

    for item in &data.editor.unterformular {
        if let Ok(index_entry) = add_item(item) {
            bundle_version_content.sub_forms.push(index_entry);
        }
    }

    bundle
        .versions
        .retain(|version| version.id != bundle_version_content.get_hash());
    bundle.versions.push(BundleVersion {
        id: bundle_version_content.get_hash(),
        name: name.to_string(),
        tag: bundle_version_content.tag.clone(),
        message,
        license: match license.clone() {
            Some(license) => Some(license),
            None => bundle.license.clone(),
        },
        info_xml: InfoXML {
            datum_xml: data.info_xml.datum_xml.clone(),
            name: data.info_xml.name.clone(),
            version: data.info_xml.version.clone(),
        },
    });

    let json = serde_json::to_string_pretty(&bundle_version_content)
        .map_err(|err| BundleError::Other(err.to_string()))?;
    fs::write(
        get_repo_path(&format!(
            "versions/{}.json",
            bundle_version_content.get_hash()
        )),
        json,
    )
    .map_err(|err| BundleError::Other(err.to_string()))?;

    let json =
        serde_json::to_string_pretty(&index).map_err(|err| BundleError::Other(err.to_string()))?;
    fs::write(get_repo_path("/index.json"), json)
        .map_err(|err| BundleError::Other(err.to_string()))?;

    Ok(())
}

pub fn search_bundle_versions(name: &str) -> Result<Vec<BundleInfo>, BundleError> {
    let mut matches = read_index_or_empty()?.bundles;
    matches.sort_by_key(|bundle| bundle.name.clone());
    let matches = matches
        .iter()
        .filter(|bundle| bundle.name.contains(name))
        .map(|bundle| {
            let version = bundle
                .versions
                .iter()
                .filter_map(|version| version.tag.clone())
                .next_back()
                .unwrap_or("latest".to_string());
            let name = bundle.name.clone();
            let description = bundle.description.clone().unwrap_or_default();

            let info_xml = bundle
                .versions
                .iter()
                .filter(|version| version.tag.is_some())
                .map(InfoXML::from_bundle_version)
                .next_back()
                .unwrap_or(InfoXML {
                    datum_xml: "".to_string(),
                    name: "".to_string(),
                    version: "".to_string(),
                });

            BundleInfo {
                name,
                version: version.clone(),
                latest_version: version,
                info_xml,
                description: Some(description),
                license: bundle.license.clone(),
                repository: None,
                versions: sorted_version_info_desc(&bundle),
            }
        })
        .collect::<Vec<_>>();
    Ok(matches)
}

pub fn bundle_info(spec: &BundleVersionSpec) -> Result<BundleInfo, BundleError> {
    let bundle = read_index_or_empty()?
        .bundles
        .into_iter()
        .find(|bundle_version| bundle_version.name.clone() == spec.bundle_name)
        .ok_or(BundleError::Other(format!(
            "Bundle '{}' existiert nicht",
            spec.bundle_name
        )))?;

    let requested_version = bundle
        .versions
        .iter()
        .rfind(|bundle_version| {
            if spec.version_tag.is_none() {
                return true;
            }
            if let Some(spec) = &spec.version_tag
                && let Some(tag) = &bundle_version.tag
                && let Ok(version_spec) = VersionReq::parse(spec)
                && let Ok(version_tag) = Version::parse(tag)
            {
                return version_spec.matches(&version_tag);
            }
            false
        })
        .ok_or(BundleError::Other(format!(
            "Version '{}' für Bundle '{}' nicht gefunden",
            spec.clone().version_tag.unwrap_or_default(),
            spec.bundle_name
        )))?;

    let mut versions = bundle.versions.iter().collect::<Vec<_>>();
    versions.sort_by(|&a, &b| {
        let version_a: Version = a
            .tag
            .clone()
            .unwrap_or_default()
            .parse()
            .unwrap_or(Version::new(0, 0, 0));
        let version_b: Version = b
            .tag
            .clone()
            .unwrap_or_default()
            .parse()
            .unwrap_or(Version::new(0, 0, 0));
        version_a.cmp_precedence(&version_b)
    });
    let latest_version = versions.iter().last().ok_or(BundleError::Other(format!(
        "Keine Versionen für Bundle '{}' gefunden",
        spec.bundle_name
    )))?;

    let bundle_info = BundleInfo {
        name: bundle.name.clone(),
        version: requested_version.tag.clone().unwrap_or_default(),
        latest_version: latest_version.tag.clone().unwrap_or_default(),
        info_xml: InfoXML::from_bundle_version(requested_version),
        description: bundle.description.clone(),
        license: match &requested_version.license {
            Some(license) => Some(license.clone()),
            None => bundle.license.clone(),
        },
        repository: bundle.repository.clone(),
        versions: sorted_version_info_desc(&bundle),
    };

    Ok(bundle_info)
}

#[allow(clippy::expect_used)]
pub fn export_bundle_versions(spec: &BundleVersionSpec) -> Result<OnkostarEditor, BundleError> {
    let id_regex =
        Regex::new(r"^[0-9a-f]{7,64}").expect("Invalid regex pattern for bundle version ID");

    if let Some(bundle_version) = read_index_or_empty()?
        .bundles
        .iter()
        .flat_map(|bundle| &bundle.versions)
        .filter(|bundle_version| bundle_version.name.clone() == spec.bundle_name)
        .rfind(|bundle_version| {
            spec.version_tag.is_none()
                || if let Some(spec) = &spec.version_tag
                    && let Some(tag) = &bundle_version.tag
                    && let Ok(version_spec) = VersionReq::parse(spec)
                    && let Ok(version_tag) = Version::parse(tag)
                {
                    return version_spec.matches(&version_tag);
                } else {
                    false
                }
                || id_regex.is_match(&spec.clone().version_tag.unwrap_or_default())
                    && bundle_version
                        .id
                        .starts_with(&spec.clone().version_tag.unwrap_or_default())
        })
        && let Ok(json) = fs::read_to_string(get_repo_path(&format!(
            "/versions/{}.json",
            bundle_version.id
        )))
        && let Ok(bundle_version_content) = serde_json::from_str::<BundleVersionContent>(&json)
    {
        let property_catalogue = bundle_version_content
            .property_catalogues
            .iter()
            .map(|item| item.checksum.clone())
            .filter_map(|id| fs::read_to_string(get_repo_path(&format!("/objects/{id}.json"))).ok())
            .filter_map(|json| serde_json::from_str::<PropertyCatalogue>(&json).ok())
            .collect::<Vec<_>>();
        let data_catalogue = bundle_version_content
            .data_catalogues
            .iter()
            .map(|item| item.checksum.clone())
            .filter_map(|id| fs::read_to_string(get_repo_path(&format!("/objects/{id}.json"))).ok())
            .filter_map(|json| serde_json::from_str::<DataCatalogue>(&json).ok())
            .collect::<Vec<_>>();
        let data_form = bundle_version_content
            .data_forms
            .iter()
            .map(|item| item.checksum.clone())
            .filter_map(|id| fs::read_to_string(get_repo_path(&format!("/objects/{id}.json"))).ok())
            .filter_map(|json| serde_json::from_str::<Form<DataFormType>>(&json).ok())
            .collect::<Vec<_>>();
        let unterformular = bundle_version_content
            .sub_forms
            .iter()
            .map(|item| item.checksum.clone())
            .filter_map(|id| fs::read_to_string(get_repo_path(&format!("/objects/{id}.json"))).ok())
            .filter_map(|json| serde_json::from_str::<Form<UnterformularType>>(&json).ok())
            .collect::<Vec<_>>();

        return Ok(OnkostarEditor {
            editor: Editor {
                property_catalogue,
                data_catalogue,
                unterformular,
                data_form,
                ablaufschema: None,
                akte: None,
                record_linkage: None,
                rskript: None,
                formulare_loeschen: None,
                formulare_deaktivieren: None,
            },
            info_xml: InfoXML::from_bundle_version(bundle_version),
        });
    }

    match spec.version_tag.as_ref() {
        Some(version_tag) => Err(BundleError::Other(format!(
            "Bundle '{}' mit Version '{}' existiert nicht",
            spec.bundle_name, version_tag
        ))),
        None => Err(BundleError::Other(format!(
            "Bundle '{}' existiert nicht",
            spec.bundle_name
        ))),
    }
}

pub fn cleanup_bundle_objects() -> Result<(), BundleError> {
    let bundle_versions = read_index_or_empty()?
        .bundles
        .iter()
        .flat_map(|bundle| &bundle.versions)
        .map(|bundle_version| bundle_version.id.clone())
        .collect::<Vec<_>>();

    let used_objects = bundle_versions
        .iter()
        .filter_map(|bundle_version| {
            if let Ok(json) =
                fs::read_to_string(get_repo_path(&format!("/versions/{bundle_version}.json")))
                && let Ok(bundle_version_content) =
                    serde_json::from_str::<BundleVersionContent>(&json)
            {
                let mut objects = bundle_version_content
                    .property_catalogues
                    .iter()
                    .map(|item| item.checksum.clone())
                    .collect::<Vec<_>>();
                objects.extend(
                    bundle_version_content
                        .data_catalogues
                        .iter()
                        .map(|item| item.checksum.clone()),
                );
                objects.extend(
                    bundle_version_content
                        .data_forms
                        .iter()
                        .map(|item| item.checksum.clone()),
                );
                objects.extend(
                    bundle_version_content
                        .sub_forms
                        .iter()
                        .map(|item| item.checksum.clone()),
                );
                return Some(objects);
            }
            None
        })
        .flatten()
        .collect::<Vec<_>>();

    fs::read_dir(get_repo_path("/objects"))
        .map_err(|err| BundleError::Other(err.to_string()))?
        .filter_map(Result::ok)
        .filter_map(|entry| entry.file_name().into_string().ok())
        .map(|filename| filename.replace(".json", ""))
        .for_each(|id| {
            if !used_objects.contains(&id) {
                fs::remove_file(get_repo_path(&format!("/objects/{id}.json")))
                    .map_err(|err| BundleError::Other(err.to_string()))
                    .unwrap_or_default();
            }
        });

    fs::read_dir(get_repo_path("/versions"))
        .map_err(|err| BundleError::Other(err.to_string()))?
        .filter_map(Result::ok)
        .filter_map(|entry| entry.file_name().into_string().ok())
        .map(|filename| filename.replace(".json", ""))
        .for_each(|id| {
            if !bundle_versions.contains(&id) {
                fs::remove_file(get_repo_path(&format!("/versions/{id}.json")))
                    .map_err(|err| BundleError::Other(err.to_string()))
                    .unwrap_or_default();
            }
        });

    Ok(())
}

fn sorted_version_info_desc(bundle: &Bundle) -> Vec<BundleVersionInfo> {
    let versions = bundle
        .versions
        .iter()
        .map(|version| BundleVersionInfo {
            name: version.name.clone(),
            version: version.tag.clone().unwrap_or("-".to_string()),
            license: match &version.license {
                Some(license) => license.clone(),
                None => bundle.license.clone().unwrap_or_default(),
            },
        })
        .collect::<Vec<_>>();

    let mut versions = versions
        .into_iter()
        .filter(|version| version.version.parse::<Version>().is_ok())
        .collect::<Vec<_>>();

    versions.sort_by(|a, b| {
        let version_a: Version = a.version.clone().parse().unwrap_or(Version::new(0, 0, 0));
        let version_b: Version = b.version.clone().parse().unwrap_or(Version::new(0, 0, 0));
        version_b.cmp_precedence(&version_a)
    });

    versions
}
