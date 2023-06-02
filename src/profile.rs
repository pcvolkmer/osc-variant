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

use std::str::FromStr;
use serde::Deserialize;

#[derive(Deserialize)]
struct Profile {
    forms: Vec<Form>
}

impl FromStr for Profile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match serde_yaml::from_str::<Profile>(s) {
            Ok(profile) => Ok(profile),
            Err(err) => Err(err.to_string())
        }
    }
}

#[derive(Deserialize)]
struct Form {
    name: String,
    fields: Vec<FieldChanges>
}

#[derive(Deserialize)]
struct FieldChanges {
    name: String,
    referenced_data_form: Option<String>
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::profile::Profile;

    #[test]
    fn should_deserialize_profile() {
        let content =
            "forms:
               - name: 'DNPM Therapieplan'
                 fields:
                   - name: ref_first_mtb
                     referenced_data_form: 'OS.Tumorkonferenz.VarianteUKW'
            ";

        match Profile::from_str(content) {
            Ok(profile) => {
                assert_eq!(profile.forms.len(), 1);
                assert_eq!(profile.forms[0].name, "DNPM Therapieplan");
                assert_eq!(profile.forms[0].fields.len(), 1);
                assert_eq!(profile.forms[0].fields[0].name, "ref_first_mtb");
                assert_eq!(profile.forms[0].fields[0].referenced_data_form, Some("OS.Tumorkonferenz.VarianteUKW".to_string()));
            },
            Err(e) => panic!("Cannot deserialize profile: {}", e)
        }
    }

    #[test]
    fn should_not_deserialize_bad_profile() {
        let content =
            "forms:
               - name: 'DNPM Therapieplan'
               # incomplete profile ...
            ";

        let actual = Profile::from_str(content);
        assert!(actual.is_err());
    }

}