//! Probe the Lib identity in Cargo package metadata.
//! Probe identity Lib trong metadata package Cargo.

use std::path::Path;

pub(in crate::language::manifest_classification) fn probe_cargo_toml(
    path: &Path,
) -> Option<String> {
    let content = mgc_config::project::read_regular_project_text(path, "Cargo manifest").ok()??;
    let value: toml::Value = toml::from_str(&content).ok()?;
    value
        .get("package")
        .and_then(|package| package.get("metadata"))
        .and_then(|metadata| metadata.get("magicore"))
        .and_then(|magicore| magicore.get("core"))
        .and_then(toml::Value::as_str)
        .map(str::to_string)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
