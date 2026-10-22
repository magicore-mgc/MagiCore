//! Probe the Lib identity in pyproject tool metadata.
//! Probe identity Lib trong tool metadata của pyproject.

use std::path::Path;

pub(in crate::language::manifest_classification) fn probe_pyproject(path: &Path) -> Option<String> {
    let content =
        mgc_config::project::read_regular_project_text(path, "Python manifest").ok()??;
    let value: toml::Value = toml::from_str(&content).ok()?;
    value
        .get("tool")
        .and_then(|tool| tool.get("magicore"))
        .and_then(|magicore| magicore.get("core"))
        .and_then(toml::Value::as_str)
        .map(str::to_string)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
