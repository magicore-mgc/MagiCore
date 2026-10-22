//! Package Projection owner.
//! Phụ trách package projection của native engine.

use std::collections::{HashMap, HashSet};

use mgc_lockfile::{ArtifactRef, EcosystemTag, Package, Provenance, SOURCE_KIND_NATIVE_RESOLVE};
use mgc_resolver::protocols::ResolvedEntry;
use mgc_types::{MgResult, PackageId, PackageName, ResolvedGraph, ResolvedPackage, Version};

use crate::native::engine::engine::entry_index::{entry_key, resolve_entry_key};
use crate::native::engine::engine::integrity_projection::resolved_integrity;

/// A native resolution: the graph for install + the v3 lock entries.
/// Một lần resolve native: graph cho install + entry lock v3.
#[derive(Debug, Clone)]
pub struct NativeResolution {
    pub graph: ResolvedGraph,
    pub lock_packages: Vec<Package>,
}

pub(in crate::native::engine::engine) fn project_packages(
    ecosystem: EcosystemTag,
    registry: &str,
    entries: &[ResolvedEntry],
    chosen: &HashMap<String, ResolvedEntry>,
    by_name: &std::collections::BTreeMap<String, Vec<String>>,
    direct_ids: &HashSet<String>,
    runtime_reachable: &HashSet<String>,
) -> MgResult<NativeResolution> {
    let mut packages = Vec::with_capacity(entries.len());
    let mut lock_packages = Vec::with_capacity(entries.len());
    for entry in entries {
        let name = PackageName::new(entry.name.clone())
            .map_err(|_| mgc_types::MgError::InvalidPackageName(entry.name.clone()))?;
        let version = Version::parse(&entry.version)
            .map_err(|_| mgc_types::MgError::InvalidVersion(entry.version.clone()))?;
        let key = entry_key(&entry.name, &entry.version);
        let direct = direct_ids.contains(&key);

        let deps: Vec<PackageId> = entry
            .deps
            .iter()
            .map(|(dep_name, range)| {
                let target_key = resolve_entry_key(dep_name, range, chosen, by_name)?;
                let resolved = chosen.get(&target_key).ok_or_else(|| {
                    mgc_types::MgError::DependencyConflict(format!(
                        "resolved package {}@{} references missing dependency {} ({range})",
                        entry.name, entry.version, dep_name
                    ))
                })?;
                let package_name = PackageName::new(resolved.name.clone())
                    .map_err(|_| mgc_types::MgError::InvalidPackageName(dep_name.clone()))?;
                let dep_version = Version::parse(&resolved.version)
                    .map_err(|_| mgc_types::MgError::InvalidVersion(resolved.version.clone()))?;
                Ok(PackageId::new(package_name, dep_version))
            })
            .collect::<mgc_types::MgResult<Vec<_>>>()?;

        let integrity = resolved_integrity(ecosystem, entry)?;

        packages.push(ResolvedPackage {
            id: PackageId::new(name, version),
            integrity: integrity.clone(),
            tarball_url: entry.artifact_url.clone(),
            deps,
            peer_deps: Vec::new(),
            direct,
            dev: !runtime_reachable.contains(&key),
        });

        let lock_dependencies = entry
            .deps
            .iter()
            .map(|(name, range)| {
                let key = resolve_entry_key(name, range, chosen, by_name)?;
                Ok(if ecosystem == EcosystemTag::Rust {
                    key
                } else {
                    name.clone()
                })
            })
            .collect::<mgc_types::MgResult<Vec<_>>>()?;
        lock_packages.push(Package {
            owner_core: None,
            name: entry.name.clone(),
            version: entry.version.clone(),
            resolved: entry.artifact_url.clone(),
            integrity: integrity.clone(),
            dependencies: lock_dependencies,
            ecosystem,
            registry: Some(registry.to_string()),
            artifact: Some(ArtifactRef {
                url: entry.artifact_url.clone(),
                size_bytes: None,
                // blake3 filled at install (requires the downloaded bytes).
                content_hash: String::new(),
                downloaded_from: registry.to_string(),
                // NuGet's catalog SHA-512 is already verified by its protocol;
                // preserve it as installer-enforced SRI for V4 conversion.
                // SHA-512 catalog NuGet đã được protocol xác minh; giữ SRI để chuyển V4.
                integrity_sri: (ecosystem == EcosystemTag::NuGet).then(|| integrity.clone()),
            }),
            provenance: Some(Provenance {
                source_kind: SOURCE_KIND_NATIVE_RESOLVE.to_string(),
                tool: Some("mgc-resolver".to_string()),
                imported_from: None,
            }),
            markers: if entry.extra_markers.is_empty() {
                None
            } else {
                Some(entry.extra_markers.clone())
            },
            extras: None,
            peers: None,
            store_ref: None,
            toolchain: None,
            scripts_policy: None,
        });
    }

    Ok(NativeResolution {
        graph: ResolvedGraph { packages },
        lock_packages,
    })
}
