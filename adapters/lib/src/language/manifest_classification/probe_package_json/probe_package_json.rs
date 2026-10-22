//! Probe the Lib identity in package.json.
//! Probe identity Lib trong package.json.

use std::path::Path;

pub(in crate::language::manifest_classification) fn probe_package_json(
    path: &Path,
) -> Option<String> {
    let content =
        mgc_config::project::read_regular_project_text(path, "package manifest").ok()??;
    let value: serde_json::Value = serde_json::from_str(&content).ok()?;
    value
        .get("magicore")
        .and_then(|metadata| metadata.get("core"))
        .and_then(serde_json::Value::as_str)
        .map(str::to_string)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
