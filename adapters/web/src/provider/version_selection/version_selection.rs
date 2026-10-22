//! `version_selection.rs` — version selection.
//! Chọn candidate semver.

use crate::native::npm_registry;
use crate::provider::NpmDependencyProvider;
use mgc_resolver::{DependencyError, ResolvedDep};
use mgc_types::{PackageId, Version, VersionRange};

impl NpmDependencyProvider {
    pub fn version_key(package_id: &PackageId) -> String {
        format!("{}@{}", package_id.name_str(), package_id.version())
    }

    pub fn optional_enqueue_key(dep: &ResolvedDep) -> String {
        format!("{}@{}", dep.package.as_str(), dep.spec)
    }

    pub fn select_best_version(
        versions: &[Version],
        spec: &str,
    ) -> Result<Option<Version>, DependencyError> {
        let constraint = VersionRange::parse(spec)
            .map_err(|e| DependencyError(format!("invalid spec '{}': {}", spec, e)))?;
        let mut matches: Vec<Version> = versions
            .iter()
            .filter(|v| constraint.matches(v))
            .cloned()
            .collect();
        if matches.is_empty() {
            return Ok(None);
        }

        let allows_prerelease = spec.contains('-');
        if !allows_prerelease {
            let stable = matches.iter().filter(|v| v.pre.is_none()).cloned().max();
            if stable.is_some() {
                return Ok(stable);
            }
        }

        matches.sort();
        Ok(matches.into_iter().max())
    }

    pub fn metadata_versions(metadata: &npm_registry::PackageMetadata) -> Vec<Version> {
        metadata
            .versions
            .keys()
            .filter_map(|v| Version::parse(v).ok())
            .collect()
    }
}
