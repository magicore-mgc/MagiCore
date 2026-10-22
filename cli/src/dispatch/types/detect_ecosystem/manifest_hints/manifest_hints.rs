//! Collect declared MagiCore core hints from supported project manifests.
//! Thu thập core hint MagiCore khai báo trong manifest project được nhận diện.

#[path = "package_json/mod.rs"]
mod package_json;
#[path = "toml_manifest/mod.rs"]
mod toml_manifest;

use anyhow::Result;
use std::path::Path;

pub(in crate::dispatch) fn read(root: &Path) -> Result<Vec<String>> {
    let mut declared = Vec::new();
    if let Some(core) = package_json::declared_core(root)? {
        declared.push(core);
    }
    if let Some(core) = toml_manifest::cargo_core(root)? {
        declared.push(core);
    }
    if let Some(core) = toml_manifest::python_core(root)? {
        declared.push(core);
    }
    Ok(declared)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
