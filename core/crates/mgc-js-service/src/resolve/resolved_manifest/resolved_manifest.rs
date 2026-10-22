//! Match project roots and project manifests to resolver-selected specs.
//! Nhận diện project root và đổi manifest sang spec resolver đã chọn.

use mgc_types::{Manifest, MgError, MgResult, PackageId, PackageName, VersionRange};

pub fn is_manifest_root_package(manifest: &Manifest, package_id: &PackageId) -> bool {
    manifest.all_dependencies().any(|dependency| {
        dependency.name == *package_id.name() && dependency.range.matches(package_id.version())
    })
}

/// Build the resolver's view of a manifest using the exact specs it solved.
/// Mutable registry dist-tags and npm aliases are not semver ranges themselves.
/// Dựng manifest đầu vào resolver bằng spec đã chọn; registry dist-tag thay đổi
/// được và npm alias không phải semver range.
pub fn manifest_with_resolved_specs(
    manifest: &Manifest,
    resolved_specs: &[(PackageName, String)],
) -> MgResult<Manifest> {
    let mut resolved_manifest = manifest.clone();
    let mut specs = resolved_specs.iter();
    for dependency in resolved_manifest
        .dependencies
        .iter_mut()
        .chain(resolved_manifest.dev_dependencies.iter_mut())
        .chain(resolved_manifest.peer_dependencies.iter_mut())
        .chain(resolved_manifest.optional_dependencies.iter_mut())
    {
        let Some((name, spec)) = specs.next() else {
            return Err(MgError::Other(
                "resolved dependency count does not match the project manifest".to_string(),
            ));
        };
        if dependency.name != *name {
            return Err(MgError::Other(format!(
                "resolved dependency '{}' does not match manifest dependency '{}'",
                name.as_str(),
                dependency.name.as_str()
            )));
        }
        dependency.range = VersionRange::parse(spec)?;
    }
    if specs.next().is_some() {
        return Err(MgError::Other(
            "resolved dependency count does not match the project manifest".to_string(),
        ));
    }
    Ok(resolved_manifest)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
