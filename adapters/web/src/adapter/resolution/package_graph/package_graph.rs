//! `package_graph.rs` — Resolved package graph assembly / lắp graph package đã resolve.
use super::super::super::WebAdapter;
use super::super::types::{ResolutionIndex, ResolutionMetadata};
use super::error::PackageGraphError;
use crate::select_resolved_candidate;
use mgc_resolver::solver::{Resolution, RootResolution};
use mgc_types::{
    Manifest, MgError, MgResult, PackageId, PackageName, VersionRange, adapter::ResolvedPackage,
};
use std::collections::{HashMap, HashSet};

pub(in crate::adapter::resolution) fn build_resolution_index(
    resolutions: &[Resolution],
) -> ResolutionIndex<'_> {
    resolutions
        .iter()
        .fold(HashMap::new(), |mut acc, resolution| {
            acc.entry(resolution.package_id.name_str().to_string())
                .or_default()
                .push(resolution);
            acc
        })
}

pub(in crate::adapter::resolution) fn assemble_resolved_packages(
    adapter: &WebAdapter,
    resolved_manifest: &Manifest,
    resolutions: &[Resolution],
    root_resolutions: &[RootResolution],
    metadata: &ResolutionMetadata,
    index: &ResolutionIndex<'_>,
) -> MgResult<Vec<ResolvedPackage>> {
    let direct_root_ids = direct_root_ids(resolutions, root_resolutions)
        .map_err(|error| MgError::Anyhow(anyhow::Error::new(error)))?;

    Ok(resolutions
        .iter()
        .map(|r| {
            // Only the resolver's indexed selection is the direct root. A
            // transitive package with the same name/range must not be promoted.
            // Chỉ lựa chọn root có index từ resolver mới là direct; không nâng
            // package transitive trùng tên/range thành root.
            let is_direct = direct_root_ids.contains(&r.package_id);
            let is_dev = resolved_manifest
                .dev_dependencies
                .iter()
                .any(|d| d.name == *r.package_id.name());
            let (tarball_url, integrity) = metadata
                .get(r.package_id.name_str())
                .and_then(|package| package.versions.get(&r.package_id.version().to_string()))
                .and_then(|version| version.dist.as_ref())
                .map(|dist| {
                    (
                        dist.tarball.clone(),
                        dist.integrity.clone().unwrap_or_default(),
                    )
                })
                .unwrap_or_else(|| (String::new(), r.integrity.clone()));
            ResolvedPackage {
                id: r.package_id.clone(),
                integrity,
                tarball_url,
                deps: r
                    .dep_specs
                    .iter()
                    .filter_map(|(name, spec)| {
                        // Peer edges are materialized in their own nested
                        // lookup scope below; do not also treat them as
                        // regular dependencies.
                        // (Peer được nối riêng bên dưới, tránh cài/nối hai lần.)
                        if r.peer_deps.iter().any(|peer| peer == name) {
                            return None;
                        }
                        let constraint = VersionRange::parse(spec).ok()?;
                        let dep_name = PackageName::new(name.as_str()).ok()?;
                        let source = adapter.provider.source_package_name(&dep_name);
                        index
                            // Alias resolutions keep the requested name as their package ID.
                            // Edge ánh xạ trước theo tên alias; chỉ fallback tên nguồn nếu cần.
                            .get(dep_name.as_str())
                            .or_else(|| index.get(source.as_str()))
                            .and_then(|candidates| {
                                select_resolved_candidate(
                                    candidates,
                                    &constraint,
                                    resolved_manifest,
                                )
                            })
                            .map(|candidate| candidate.package_id.clone())
                    })
                    .collect(),
                peer_deps: metadata
                    .get(r.package_id.name_str())
                    .and_then(|pkg_meta| pkg_meta.versions.get(&r.package_id.version().to_string()))
                    .and_then(|ver_meta| {
                        ver_meta.peer_dependencies.as_ref().map(|peers| {
                            peers
                                .iter()
                                // Optional peers bind when already resolved; provider policy
                                // still prevents fetching them solely for this declaration.
                                // Peer optional đã có được nối; policy provider vẫn không tự tải.
                                .filter(|(peer_name, _)| {
                                    // Optional inference cannot overwrite a declared regular edge.
                                    // Peer optional suy ra không được thay dependency thường đã khai báo.
                                    r.peer_deps.iter().any(|name| name == *peer_name)
                                        || !r.dep_specs.iter().any(|(name, _)| name == *peer_name)
                                })
                                .filter_map(|(peer_name, peer_range)| {
                                    // A satisfying project root takes precedence over
                                    // a newer transitive instance; an incompatible
                                    // candidate is never selected as a fallback.
                                    // (Ưu tiên root thỏa range; không chọn bản lệch range.)
                                    let constraint = VersionRange::parse(peer_range).ok()?;
                                    index
                                        .get(peer_name.as_str())
                                        .and_then(|candidates| {
                                            select_resolved_candidate(
                                                candidates,
                                                &constraint,
                                                resolved_manifest,
                                            )
                                        })
                                        .map(|candidate| candidate.package_id.clone())
                                })
                                .collect()
                        })
                    })
                    .unwrap_or_default(),
                direct: is_direct,
                dev: is_dev,
            }
        })
        .collect())
}

fn direct_root_ids(
    resolutions: &[Resolution],
    root_resolutions: &[RootResolution],
) -> Result<HashSet<PackageId>, PackageGraphError> {
    let resolved_ids: HashSet<(&str, &mgc_types::Version)> = resolutions
        .iter()
        .map(|resolution| {
            (
                resolution.package_id.name_str(),
                resolution.package_id.version(),
            )
        })
        .collect();
    let mut direct_ids = HashSet::with_capacity(root_resolutions.len());
    for root in root_resolutions {
        let root_key = (root.package_id.name_str(), root.package_id.version());
        if !resolved_ids.contains(&root_key) {
            return Err(PackageGraphError::MissingResolvedRoot {
                package_id: root.package_id.to_string(),
            });
        }
        direct_ids.insert(root.package_id.clone());
    }
    Ok(direct_ids)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
