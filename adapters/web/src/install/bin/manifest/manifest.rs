//! Parse the installed package manifest's `bin` field.
//! Phân tích trường `bin` trong manifest package đã cài.

use mgc_types::{MgError, MgResult};
use serde::Deserialize;
use std::path::{Path, PathBuf};

use super::super::error::BinError;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum PackageBinField {
    Single(String),
    Multiple(std::collections::HashMap<String, String>),
}

#[derive(Debug, Deserialize)]
pub struct InstalledPackageManifest {
    pub name: String,
    pub bin: Option<PackageBinField>,
}

pub fn package_bin_entries(package_dir: &Path) -> MgResult<Vec<(String, PathBuf)>> {
    package_bin_entries_inner(package_dir).map_err(MgError::from)
}

fn package_bin_entries_inner(package_dir: &Path) -> Result<Vec<(String, PathBuf)>, BinError> {
    let package_json = package_dir.join("package.json");
    if !package_json.exists() {
        return Ok(vec![]);
    }

    let source = std::fs::read_to_string(&package_json)
        .map_err(|error| BinError::Manifest(MgError::Io(error)))?;
    let manifest: InstalledPackageManifest = serde_json::from_str(&source).map_err(|err| {
        BinError::Manifest(MgError::Other(format!(
            "failed to parse package manifest '{}': {}",
            package_json.display(),
            err
        )))
    })?;

    let entries = match manifest.bin {
        Some(PackageBinField::Single(path)) => vec![(
            manifest
                .name
                .rsplit('/')
                .next()
                .unwrap_or(manifest.name.as_str())
                .to_string(),
            PathBuf::from(path),
        )],
        Some(PackageBinField::Multiple(entries)) => entries
            .into_iter()
            .map(|(name, path)| (name, PathBuf::from(path)))
            .collect(),
        None => vec![],
    };

    Ok(entries)
}
