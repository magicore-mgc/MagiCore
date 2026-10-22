//! Root Selection owner.
//! Phụ trách root selection của native engine.

use super::super::entry_index::entry_key;
use std::collections::{BTreeMap, HashSet};

use mgc_resolver::protocols::{RegistryProtocol, RegistryRoot, ResolvedEntry};
use mgc_types::{DependencySpec, Manifest, MgResult, Version};

pub(in crate::native::engine::engine) type RootDependency<'a> = (&'a DependencySpec, bool);

pub(in crate::native::engine::engine) fn selected_roots<'a>(
    manifest: &'a Manifest,
    include_dev_dependencies: bool,
) -> Vec<RootDependency<'a>> {
    let mut roots = manifest
        .dependencies
        .iter()
        .map(|dependency| (dependency, false))
        .collect::<Vec<_>>();
    if include_dev_dependencies {
        roots.extend(
            manifest
                .dev_dependencies
                .iter()
                .map(|dependency| (dependency, true)),
        );
    }
    roots
}

pub(in crate::native::engine::engine) fn registry_roots(
    roots: &[RootDependency<'_>],
) -> Vec<RegistryRoot> {
    roots
        .iter()
        .map(|(dependency, _)| RegistryRoot {
            name: dependency.name.as_str().to_string(),
            range: dependency.range.as_str().to_string(),
            features: dependency.features.clone(),
            default_features: dependency.default_features,
        })
        .collect()
}

pub(in crate::native::engine::engine) fn resolve_root_ids(
    protocol: &dyn RegistryProtocol,
    roots: &[RootDependency<'_>],
    entries: &[ResolvedEntry],
    by_name: &BTreeMap<String, Vec<String>>,
) -> MgResult<HashSet<String>> {
    let mut selected_roots = HashSet::new();
    for (dependency, _) in roots {
        let name = dependency.name.as_str();
        let candidates = by_name.get(name).ok_or_else(|| {
            mgc_types::MgError::DependencyConflict(format!(
                "registry returned no resolved root package for '{name}'"
            ))
        })?;
        let mut matching = candidates
            .iter()
            .filter_map(|key| {
                let version = key.rsplit_once('@')?.1;
                let parsed = Version::parse(version).ok()?;
                protocol
                    .matches_requirement(dependency.range.as_str(), &parsed)
                    .then_some((parsed, key))
            })
            .collect::<Vec<_>>();
        matching.sort_by(|left, right| left.0.cmp(&right.0));
        let selected = matching.last().ok_or_else(|| {
            mgc_types::MgError::DependencyConflict(format!(
                "registry returned no version of '{name}' matching '{}'",
                dependency.range
            ))
        })?;
        selected_roots.insert(selected.1.clone());
    }
    let returned = entries
        .iter()
        .map(|entry| entry_key(&entry.name, &entry.version))
        .collect::<HashSet<_>>();
    if selected_roots.iter().any(|root| !returned.contains(root)) {
        return Err(mgc_types::MgError::Other(
            "registry root selection is absent from the resolved entries".to_string(),
        ));
    }
    Ok(selected_roots)
}
