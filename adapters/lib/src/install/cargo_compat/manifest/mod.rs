//! Cargo manifest parsing and lock-root projection owners.
//! Các owner parse manifest Cargo và chiếu root từ lockfile.

mod dependency_specs;
mod error;
mod project_manifest;
mod root_resolution;

use super::CargoProjectManifest;
use mgc_lockfile::Package;
use mgc_types::MgResult;
use std::collections::BTreeMap;

/// Keep the existing projection boundary while delegating parsing to its owner.
/// Giữ nguyên ranh giới projection hiện có và giao parse cho owner tương ứng.
pub(in crate::install) fn read_project_manifest(
    project_root: &std::path::Path,
) -> MgResult<CargoProjectManifest> {
    project_manifest::read_project_manifest(project_root).map_err(Into::into)
}

/// Preserve root mapping behavior while separating dependency-option policy.
/// Giữ cách ánh xạ root và tách policy option dependency.
pub(in crate::install) fn resolve_manifest_roots(
    manifest: &toml::Value,
    root_ids: &[String],
    by_id: &BTreeMap<String, &Package>,
    by_name: &BTreeMap<&str, Vec<&Package>>,
) -> MgResult<Vec<String>> {
    root_resolution::resolve_manifest_roots(manifest, root_ids, by_id, by_name).map_err(Into::into)
}
