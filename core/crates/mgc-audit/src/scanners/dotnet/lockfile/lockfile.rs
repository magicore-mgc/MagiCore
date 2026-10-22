//! Parse NuGet package pins from `packages.lock.json`.
//! Parse pin NuGet từ `packages.lock.json`.

use crate::scanners::osv::OsvPin;
use mgc_types::{MgError, MgResult};
use serde::Deserialize;
use std::collections::BTreeSet;

use super::super::error::DotnetLockParseError;

#[derive(Debug, Deserialize)]
struct NuGetLock {
    #[serde(default)]
    dependencies:
        std::collections::BTreeMap<String, std::collections::BTreeMap<String, NuGetEntry>>,
}

#[derive(Debug, Deserialize)]
struct NuGetEntry {
    #[serde(default)]
    #[allow(dead_code)]
    r#type: String,
    #[serde(default)]
    version: Option<String>,
    #[serde(default)]
    resolved: Option<String>,
}

/// Read lockfile pins, deduplicated across target frameworks.
/// Đọc pin lockfile và khử trùng giữa các target framework.
pub fn read_packages_lock(raw: &str) -> MgResult<(Vec<OsvPin>, Vec<String>)> {
    let lock: NuGetLock = serde_json::from_str(raw)
        .map_err(DotnetLockParseError::from)
        .map_err(MgError::from)?;
    let mut pins = Vec::new();
    let mut skipped = Vec::new();
    let mut seen = BTreeSet::new();
    for tfm_dependencies in lock.dependencies.values() {
        for (name, entry) in tfm_dependencies {
            let version = entry
                .resolved
                .as_deref()
                .filter(|value| !value.is_empty())
                .or_else(|| entry.version.as_deref().filter(|value| !value.is_empty()));
            let Some(version) = version else {
                skipped.push(format!("{name}: no version pin in packages.lock.json"));
                continue;
            };
            if !seen.insert(format!("{name}@{version}")) {
                continue;
            }
            pins.push(OsvPin {
                name: name.clone(),
                version: version.to_string(),
                ecosystem: "NuGet",
            });
        }
    }
    Ok((pins, skipped))
}
