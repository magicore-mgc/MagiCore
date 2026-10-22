//! Read explicit core hints from Cargo and Python TOML manifests.
//! Đọc core hint tường minh từ manifest TOML của Cargo và Python.

use anyhow::Result;
use mgc_config::project::read_regular_project_text;
use std::path::Path;

pub(in crate::dispatch) fn cargo_core(root: &Path) -> Result<Option<String>> {
    read_core(
        root,
        "Cargo.toml",
        "Cargo manifest",
        &["package", "metadata", "magicore", "core"],
    )
}

pub(in crate::dispatch) fn python_core(root: &Path) -> Result<Option<String>> {
    read_core(
        root,
        "pyproject.toml",
        "Python manifest",
        &["tool", "magicore", "core"],
    )
}

fn read_core(
    root: &Path,
    file_name: &str,
    label: &str,
    key_path: &[&str],
) -> Result<Option<String>> {
    let path = root.join(file_name);
    let Some(content) = read_regular_project_text(&path, label)? else {
        return Ok(None);
    };
    let manifest: toml::Value = toml::from_str(&content)?;
    let mut value = &manifest;
    for key in key_path {
        let Some(child) = value.get(*key) else {
            return Ok(None);
        };
        value = child;
    }
    Ok(value.as_str().map(str::to_owned))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
