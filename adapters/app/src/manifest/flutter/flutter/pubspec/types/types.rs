//! Deserialized Flutter pubspec shape used by the parser.
//! Cấu trúc pubspec Flutter được deserialize cho parser.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub(in crate::manifest::flutter::flutter::pubspec) struct PubspecYaml {
    pub(in crate::manifest::flutter::flutter::pubspec) name: String,
    #[serde(default)]
    pub(in crate::manifest::flutter::flutter::pubspec) dependencies:
        Option<HashMap<String, serde_yaml::Value>>,
    #[serde(default)]
    pub(in crate::manifest::flutter::flutter::pubspec) dev_dependencies:
        Option<HashMap<String, serde_yaml::Value>>,
    #[serde(default)]
    pub(in crate::manifest::flutter::flutter::pubspec) dependency_overrides:
        Option<HashMap<String, serde_yaml::Value>>,
    #[serde(default)]
    pub(in crate::manifest::flutter::flutter::pubspec) workspace: Option<Vec<String>>,
}
