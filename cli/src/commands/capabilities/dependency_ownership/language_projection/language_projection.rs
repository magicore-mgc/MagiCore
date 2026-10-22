//! Language ownership projection — projection ownership theo language.
//! Applies the existing manifest qualification for split-core ecosystems.
//! Áp dụng điều kiện manifest hiện có cho ecosystem core phân nhánh.

use serde_json::{Map, Value};

use crate::commands::dep_gate::SPLIT_LANGUAGES;

use super::super::operation_projection::operations_for;

/// Return only language rows that differ from the core's base operation row.
/// Chỉ trả hàng language khác với hàng operation cơ sở của core.
pub(in crate::commands::capabilities::dependency_ownership) fn language_projection(
    core: &str,
    base: &Value,
) -> Map<String, Value> {
    let mut languages = Map::new();
    for language in SPLIT_LANGUAGES {
        let manifest_format = manifest_format(core, language);
        let split = operations_for(core, Some(language), manifest_format);
        if &split != base {
            languages.insert((*language).to_string(), split);
        }
    }
    languages
}

fn manifest_format(core: &str, language: &str) -> Option<&'static str> {
    match (core, language) {
        ("ai", "python") => Some("mgc-pyproject"),
        ("lib", "ts") => Some("package-json"),
        ("lib", "rust") => Some("cargo-toml"),
        ("lib", "python") => Some("pep621-native"),
        ("lib", "go") => Some("go-mod"),
        ("lib", "java") => Some("maven-pom"),
        ("lib", "dotnet") => Some("csproj"),
        ("game", "bevy") | ("iot", "esp32-rust") => Some("cargo-toml"),
        _ => None,
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
