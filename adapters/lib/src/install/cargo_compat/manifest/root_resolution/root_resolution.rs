//! Resolve declared direct roots and preserve deterministic Cargo labels.
//! Resolve root trực tiếp và giữ nhãn Cargo có thứ tự ổn định.

use super::super::super::cargo_dependency_label;
use super::super::dependency_specs::validate_dependency_specs;
use super::super::error::CargoManifestError;
use mgc_lockfile::Package;
use std::collections::{BTreeMap, BTreeSet};

/// Require a one-to-one match between manifest roots and MGC lock roots.
/// Bắt buộc root manifest khớp một-một với root trong MGC lock.
pub(in crate::install) fn resolve_manifest_roots(
    manifest: &toml::Value,
    root_ids: &[String],
    by_id: &BTreeMap<String, &Package>,
    by_name: &BTreeMap<&str, Vec<&Package>>,
) -> Result<Vec<String>, CargoManifestError> {
    let manifest_roots = if let Some(dependencies) = manifest.get("dependencies") {
        dependencies
            .as_table()
            .ok_or(CargoManifestError::DependenciesTableRequired)?
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>()
    } else {
        BTreeSet::new()
    };
    let mut roots = Vec::new();
    let mut root_ids_seen = BTreeSet::new();
    for root_id in root_ids {
        let package = by_id
            .get(root_id)
            .ok_or_else(|| CargoManifestError::LockRootMissing {
                root_id: root_id.clone(),
            })?;
        if !manifest_roots.contains(&package.name) {
            return Err(CargoManifestError::RootAbsentFromManifest {
                name: package.name.clone(),
            });
        }
        roots.push(cargo_dependency_label(package, by_name));
        root_ids_seen.insert(root_id.clone());
    }
    if root_ids_seen.len() != manifest_roots.len()
        || manifest_roots.iter().any(|name| {
            !root_ids.iter().any(|root_id| {
                by_id
                    .get(root_id)
                    .is_some_and(|package| package.name == *name)
            })
        })
    {
        return Err(CargoManifestError::DirectRootsMismatch);
    }

    validate_dependency_specs(manifest, by_name)?;
    roots.sort();
    Ok(roots)
}

#[cfg(test)]
#[path = "test/root_resolution.rs"]
mod tests;
