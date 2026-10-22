//! Project a verified V4 dependency graph into the legacy Lib adapter model.
//! Chiếu graph dependency V4 đã xác minh sang model adapter Lib legacy.

use mgc_types::{MgError, MgResult};
use std::path::Path;

use crate::install::python_project_owner;

pub(in crate::install) fn project_v4_lock_to_lib_model(
    project_root: &Path,
    document: &mgc_lockfile::canonical::LockfileV4,
) -> MgResult<mgc_lockfile::Lockfile> {
    use std::collections::{HashMap, HashSet};

    let owner = python_project_owner(project_root)?;
    if !document.root_dependencies_by_owner.is_empty()
        && !document.root_dependencies_by_owner.contains_key(&owner)
    {
        // A new lane has no existing rows to reuse; fresh resolution must supply its own pins.
        // Lane mới chưa có row để tái dùng; resolve mới phải cấp pin riêng, không mượn sibling.
        // Frozen replay is independently rejected by the authoritative CLI owner/closure gate.
        // Cổng owner/closure của CLI vẫn từ chối frozen replay khi owner chưa tồn tại.
        return Ok(mgc_lockfile::Lockfile::new());
    }
    // Validate installability before projecting any V4 identity into the
    // legacy adapter-facing row shape.
    // (Xác minh graph V4 cài được trước khi chiếu identity sang row adapter.)
    mgc_lockfile::v4_graph::graph_from_v4_lockfile(document, &owner)
        .map_err(|error| MgError::Other(error.to_string()))?;

    let roots = mgc_lockfile::v4_graph::v4_root_pins_for_owner(document, &owner)
        .map_err(|error| MgError::Other(error.to_string()))?;
    let mut indices = HashMap::with_capacity(document.packages.len());
    for (index, package) in document.packages.iter().enumerate() {
        let id = format!("{}@{}", package.key.name, package.key.version);
        if indices.insert(id.clone(), index).is_some() {
            return Err(MgError::Other(format!(
                "cannot project V4 lib lock: duplicate package instance '{id}'"
            )));
        }
    }

    let mut reachable = HashSet::new();
    let mut pending = Vec::new();
    for root in roots {
        let pin = mgc_lockfile::parse_root_pin(root);
        if let Some(index) = indices.get(pin.package_id).copied() {
            pending.push(index);
        } else {
            return Err(MgError::Other(format!(
                "cannot project V4 lib lock: owner root '{root}' names no package"
            )));
        }
    }
    while let Some(index) = pending.pop() {
        if !reachable.insert(index) {
            continue;
        }
        for edge in &document.packages[index].edges {
            let id = format!("{}@{}", edge.target_key.name, edge.target_key.version);
            let Some(target) = indices.get(&id).copied() else {
                return Err(MgError::Other(format!(
                    "cannot project V4 lib lock: edge names missing package '{id}'"
                )));
            };
            pending.push(target);
        }
    }
    if !document.packages.is_empty() && roots.is_empty() {
        return Err(MgError::Other(format!(
            "cannot project V4 lib lock: no dependency roots are recorded for core '{owner}'"
        )));
    }
    let mut reachable_indices = reachable.into_iter().collect::<Vec<_>>();
    reachable_indices.sort_unstable();

    let sources = document
        .sources
        .iter()
        .map(|source| (source.id.as_str(), source.url.as_str()))
        .collect::<HashMap<_, _>>();
    let mut lock = mgc_lockfile::Lockfile::new();
    lock.version = document.version.clone();
    lock.root_dependencies = document.root_dependencies.clone();
    lock.root_dependencies_by_owner = document.root_dependencies_by_owner.clone();
    for index in reachable_indices {
        let locked = &document.packages[index];
        let artifact = locked.artifact.as_ref().ok_or_else(|| {
            MgError::Other(format!(
                "cannot project V4 lib lock: '{}@{}' has no artifact",
                locked.key.name, locked.key.version
            ))
        })?;
        let integrity = artifact.integrity_sri.clone().unwrap_or_default();
        let markers = locked.markers.clone().or_else(|| {
            if locked.key.ecosystem == mgc_lockfile::EcosystemTag::NuGet
                && let Some(digest) = integrity.strip_prefix("sha512-")
            {
                Some(vec![format!("sha512:{digest}")])
            } else {
                None
            }
        });
        let peer_names = locked
            .key
            .variant
            .peer_context
            .as_ref()
            .and_then(|digest| document.peer_contexts.get(digest))
            .map(|peers| peers.iter().map(|(name, _)| name.clone()).collect());
        lock.packages.push(mgc_lockfile::Package {
            owner_core: Some(owner.clone()),
            name: locked.key.name.clone(),
            version: locked.key.version.clone(),
            resolved: artifact.url.clone(),
            integrity,
            dependencies: locked
                .edges
                .iter()
                .map(|edge| {
                    if edge.target_key.ecosystem == mgc_lockfile::EcosystemTag::Rust {
                        format!("{}@{}", edge.target_key.name, edge.target_key.version)
                    } else {
                        edge.target_key.name.clone()
                    }
                })
                .collect(),
            ecosystem: locked.key.ecosystem,
            registry: sources
                .get(locked.key.source_id.as_str())
                .map(|url| (*url).to_string()),
            artifact: Some(artifact.clone()),
            provenance: locked.provenance.clone(),
            markers,
            extras: locked.key.variant.feature_set.clone(),
            peers: peer_names,
            store_ref: locked.store_ref.clone(),
            toolchain: locked.toolchain.clone(),
            scripts_policy: locked.scripts_policy.clone(),
        });
    }
    Ok(lock)
}
