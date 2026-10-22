//! `graph_restore.rs` — locked graph reconstruction from owner roots and exact edges.
//! dựng graph từ root owner và cạnh chính xác trong lock

use super::super::{is_web_lock_package, locked_edge_target};
use mgc_lockfile::Lockfile;
use mgc_types::{
    Manifest, MgResult, PackageId,
    adapter::{ResolvedGraph, ResolvedPackage},
};

pub fn build_graph_from_lockfile(
    lockfile: &Lockfile,
    manifest: &Manifest,
    owner_core: &str,
) -> MgResult<Option<ResolvedGraph>> {
    let mut packages_by_id = std::collections::HashMap::new();
    let mut ids_by_name: std::collections::HashMap<String, Vec<PackageId>> =
        std::collections::HashMap::new();
    for package in lockfile
        .packages
        .iter()
        .filter(|package| is_web_lock_package(lockfile, package))
    {
        let id = PackageId::parse(&format!("{}@{}", package.name, package.version))?;
        if packages_by_id.insert(id.clone(), package).is_some() {
            // An ambiguous exact identity cannot safely reconstruct edges.
            // (Identity trùng khiến không thể phục hồi cạnh một cách an toàn.)
            return Ok(None);
        }
        ids_by_name
            .entry(id.name_str().to_string())
            .or_default()
            .push(id);
    }

    let mut pinned_roots = std::collections::HashSet::new();
    if let Some(roots) = lockfile.root_dependencies_by_owner.get(owner_core) {
        for root in roots {
            let parsed = mgc_lockfile::parse_root_pin(root);
            if parsed
                .ecosystem
                .is_some_and(|ecosystem| ecosystem != mgc_lockfile::EcosystemTag::Web)
            {
                continue;
            }
            let Ok(id) = PackageId::parse(parsed.package_id) else {
                return Ok(None);
            };
            if packages_by_id.contains_key(&id) {
                pinned_roots.insert(id);
            }
        }
    }
    if pinned_roots.is_empty() && lockfile.version == "2" {
        for root in &lockfile.root_dependencies {
            if let Ok(id) = PackageId::parse(root)
                && packages_by_id.contains_key(&id)
            {
                pinned_roots.insert(id);
            }
        }
    }

    let mut root_ids = Vec::new();
    for dependency in manifest.all_dependencies() {
        let candidates: Vec<PackageId> = if pinned_roots.is_empty() {
            ids_by_name
                .get(dependency.name.as_str())
                .into_iter()
                .flatten()
                .filter(|id| dependency.range.matches(id.version()))
                .cloned()
                .collect()
        } else {
            pinned_roots
                .iter()
                .filter(|id| {
                    id.name_str() == dependency.name.as_str()
                        && dependency.range.matches(id.version())
                })
                .cloned()
                .collect()
        };
        if candidates.len() != 1 {
            // Missing or ambiguous root pins fall back to a fresh resolution;
            // never silently choose a different version.
            // (Thiếu hoặc trùng root pin thì resolve mới; không tự chọn version khác.)
            return Ok(None);
        }
        root_ids.push(candidates[0].clone());
    }

    let mut pending = std::collections::VecDeque::from(root_ids.clone());
    let mut visited = std::collections::HashSet::new();
    let mut graph_packages = Vec::new();
    while let Some(id) = pending.pop_front() {
        if !visited.insert(id.clone()) {
            continue;
        }
        let Some(package) = packages_by_id.get(&id).copied() else {
            return Ok(None);
        };
        let mut dependencies = Vec::with_capacity(package.dependencies.len());
        for edge in &package.dependencies {
            let Some(target) = locked_edge_target(edge, &packages_by_id, &ids_by_name) else {
                return Ok(None);
            };
            pending.push_back(target.clone());
            dependencies.push(target);
        }
        let mut peer_dependencies = Vec::new();
        for edge in package.peers.as_deref().unwrap_or_default() {
            let Some(target) = locked_edge_target(edge, &packages_by_id, &ids_by_name) else {
                return Ok(None);
            };
            pending.push_back(target.clone());
            peer_dependencies.push(target);
        }
        graph_packages.push(ResolvedPackage {
            id: id.clone(),
            integrity: package.integrity.clone(),
            tarball_url: package.resolved.clone(),
            deps: dependencies,
            peer_deps: peer_dependencies,
            direct: root_ids.contains(&id),
            dev: root_ids.contains(&id)
                && manifest
                    .dev_dependencies
                    .iter()
                    .any(|dependency| dependency.name.as_str() == id.name_str()),
        });
    }
    Ok(Some(ResolvedGraph {
        packages: graph_packages,
    }))
}
