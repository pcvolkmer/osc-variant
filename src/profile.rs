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

use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct Profile {
    pub forms: Vec<Form>,
}

impl FromStr for Profile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match serde_yaml::from_str::<Profile>(s) {
            Ok(profile) => Ok(profile),
            Err(err) => Err(err.to_string()),
        }
    }
}

#[derive(Deserialize)]
pub struct Form {
    pub name: String,
    #[serde(default)]
    pub form_references: Vec<FormReference>,
    pub menu_category: Option<MenuCategory>,
}

#[derive(Deserialize)]
pub struct FormReference {
    pub name: String,
    pub referenced_data_form: Option<String>,
    pub anzeige: Option<String>,
    pub anzeige_auswahl: Option<String>,
}

#[derive(Deserialize)]
pub struct MenuCategory {
    pub name: String,
    pub position: String,
    pub column: String,
}

#[cfg(test)]
mod tests {
    use crate::profile::Profile;
    use std::str::FromStr;

    #[test]
    fn should_deserialize_profile() {
        let content = "forms:
               - name: 'DNPM Therapieplan'
                 menu_category:
                   name: DNPM
                   position: 3.0
                   column: 1
                 form_references:
                   - name: ref_first_mtb
                     referenced_data_form: 'OS.Tumorkonferenz.VarianteUKW'
                     anzeige: 'Datum: {Datum}'
                     anzeige_auswahl: 'TK vom {Datum}'
            ";

        match Profile::from_str(content) {
            Ok(profile) => {
                assert_eq!(profile.forms.len(), 1);
                assert_eq!(profile.forms[0].name, "DNPM Therapieplan");
                assert!(profile.forms[0].menu_category.is_some());
                assert_eq!(profile.forms[0].form_references.len(), 1);
            }
            Err(e) => panic!("Cannot deserialize profile: {}", e),
        }
    }

    #[test]
    fn should_deserialize_form_reference() {
        let content = "forms:
               - name: 'DNPM Therapieplan'
                 form_references:
                   - name: ref_first_mtb
                     referenced_data_form: 'OS.Tumorkonferenz.VarianteUKW'
                     anzeige: 'Datum: {Datum}'
                     anzeige_auswahl: 'TK vom {Datum}'
            ";

        match Profile::from_str(content) {
            Ok(profile) => {
                assert_eq!(profile.forms.len(), 1);
                assert_eq!(profile.forms[0].name, "DNPM Therapieplan");
                assert_eq!(profile.forms[0].form_references.len(), 1);
                assert_eq!(profile.forms[0].form_references[0].name, "ref_first_mtb");
                assert_eq!(
                    profile.forms[0].form_references[0].referenced_data_form,
                    Some("OS.Tumorkonferenz.VarianteUKW".to_string())
                );
                assert_eq!(
                    profile.forms[0].form_references[0].anzeige,
                    Some("Datum: {Datum}".to_string())
                );
                assert_eq!(
                    profile.forms[0].form_references[0].anzeige_auswahl,
                    Some("TK vom {Datum}".to_string())
                );
            }
            Err(e) => panic!("Cannot deserialize profile: {}", e),
        }
    }

    #[test]
    fn should_deserialize_menu_category() {
        let content = "forms:
               - name: 'DNPM Therapieplan'
                 menu_category:
                   name: DNPM
                   position: 3.0
                   column: 1
            ";

        match Profile::from_str(content) {
            Ok(profile) => {
                assert_eq!(profile.forms.len(), 1);
                assert_eq!(profile.forms[0].name, "DNPM Therapieplan");
                assert!(profile.forms[0].menu_category.is_some());
                assert!(profile.forms[0]
                    .menu_category
                    .as_ref()
                    .is_some_and(|menu_category| {
                        assert_eq!(menu_category.name, "DNPM");
                        assert_eq!(menu_category.position, "3.0");
                        assert_eq!(menu_category.column, "1");
                        true
                    }));
            }
            Err(e) => panic!("Cannot deserialize profile: {}", e),
        }
    }

    #[test]
    fn should_deserialize_profile_with_no_changes() {
        let content = "forms:
               - name: 'DNPM Therapieplan'
                 form_references:
                   - name: ref_first_mtb
            ";

        match Profile::from_str(content) {
            Ok(profile) => {
                assert_eq!(profile.forms.len(), 1);
                assert_eq!(profile.forms[0].name, "DNPM Therapieplan");
                assert_eq!(profile.forms[0].form_references.len(), 1);
                assert_eq!(profile.forms[0].form_references[0].name, "ref_first_mtb");
                assert_eq!(
                    profile.forms[0].form_references[0].referenced_data_form,
                    None
                );
                assert_eq!(profile.forms[0].form_references[0].anzeige, None);
                assert_eq!(profile.forms[0].form_references[0].anzeige_auswahl, None);
            }
            Err(e) => panic!("Cannot deserialize profile: {}", e),
        }
    }

    #[test]
    fn should_not_deserialize_bad_profile() {
        let content = "forms:
               - name: 'DNPM Therapieplan'
               - form_references: Unknown
               # incomplete profile ...
            ";

        let actual = Profile::from_str(content);
        assert!(actual.is_err());
    }
}
