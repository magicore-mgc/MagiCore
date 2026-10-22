//! Existing Lib identity and manifest probes.
//! Identity Lib và probe manifest hiện hành.

use std::path::Path;

use crate::language::is_regular_manifest;

/// Manifest metadata reader shared only by the classifier coordinator.
/// Reader metadata manifest chỉ được coordinator classifier dùng chung.
type ManifestProbe = fn(&Path) -> Option<String>;

/// Classify a project as Lib using the existing config and metadata probes.
/// Phân loại project Lib theo config và metadata probe hiện hành.
pub(crate) fn manifest_is_lib(root: &Path) -> bool {
    if let Ok(Some(content)) =
        mgc_config::project::read_regular_project_text(&root.join("mgc.toml"), "project config")
        && let Ok(value) = toml::from_str::<toml::Value>(&content)
    {
        if value
            .get("ecosystem")
            .and_then(|ecosystem| ecosystem.as_str())
            == Some("lib")
        {
            return true;
        }
        if value.get("lib").is_some() {
            return true;
        }
    }
    let probes: [(&Path, ManifestProbe); 3] = [
        (
            &root.join("package.json"),
            super::probe_package_json::probe_package_json,
        ),
        (
            &root.join("Cargo.toml"),
            super::probe_cargo_toml::probe_cargo_toml,
        ),
        (
            &root.join("pyproject.toml"),
            super::probe_pyproject::probe_pyproject,
        ),
    ];
    for (path, probe) in probes {
        if is_regular_manifest(path)
            && let Some(ecosystem) = probe(path)
            && ecosystem == "lib"
        {
            return true;
        }
    }
    false
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
