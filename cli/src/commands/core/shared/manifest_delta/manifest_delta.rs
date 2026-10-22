//! Build exact-version dependency deltas and merge their resolved graphs.
//! Tạo delta dependency ghim version chính xác và gộp resolved graph.

use super::error::ManifestDeltaError;
use mgc_types::{DependencySpec, Manifest, PackageId, ResolvedGraph, VersionRange};

#[derive(Debug, Clone)]
pub(crate) struct AddedPackage {
    pub(in crate::commands::core::shared::implementation::implementation) id: PackageId,
    pub(in crate::commands::core::shared::implementation::implementation) dev: bool,
    pub(in crate::commands::core::shared::implementation::implementation) optional: bool,
    pub(in crate::commands::core::shared::implementation::implementation) peer: bool,
}

pub(crate) fn build_delta_manifest(
    manifest: &Manifest,
    added_packages: &[AddedPackage],
) -> Result<Manifest, ManifestDeltaError> {
    let mut delta = Manifest::new(&manifest.name, manifest.ecosystem);
    for package in added_packages {
        let mut spec = DependencySpec::new(
            package.id.name().clone(),
            VersionRange::parse(&format!("={}", package.id.version()))?,
        );
        spec.dev = package.dev;
        spec.optional = package.optional;
        spec.peer = package.peer;
        delta.add_dep(spec, package.dev, package.optional, package.peer);
    }
    Ok(delta)
}

pub(crate) fn merge_graphs(mut base: ResolvedGraph, delta: ResolvedGraph) -> ResolvedGraph {
    let mut positions = std::collections::HashMap::new();
    for (idx, pkg) in base.packages.iter().enumerate() {
        positions.insert(pkg.id.clone(), idx);
    }
    for pkg in delta.packages {
        if let Some(idx) = positions.get(&pkg.id).copied() {
            let existing = &mut base.packages[idx];
            existing.direct |= pkg.direct;
            existing.dev |= pkg.dev;
            if existing.integrity.is_empty() && !pkg.integrity.is_empty() {
                existing.integrity = pkg.integrity;
            }
            if existing.tarball_url.is_empty() && !pkg.tarball_url.is_empty() {
                existing.tarball_url = pkg.tarball_url;
            }
            for dep in pkg.deps {
                if !existing.deps.contains(&dep) {
                    existing.deps.push(dep);
                }
            }
        } else {
            positions.insert(pkg.id.clone(), base.packages.len());
            base.packages.push(pkg);
        }
    }
    base
}
