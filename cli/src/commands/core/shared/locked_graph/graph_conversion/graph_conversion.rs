//! Convert validated lock records into resolved graphs.
//! Chuyển lock record đã xác thực thành resolved graph.

use super::super::*;

#[allow(dead_code)]
pub(in crate::commands::core::shared::implementation::implementation) fn graph_from_lockfile(
    lock: &Lockfile,
    owner_core: &str,
) -> std::result::Result<ResolvedGraph, mgc_types::MgError> {
    let direct_roots: std::collections::HashSet<_> = lock
        .root_dependencies_by_owner
        .get(owner_core)
        .into_iter()
        .flatten()
        .cloned()
        .collect();
    let packages = lock
        .packages
        .iter()
        .map(|package| {
            let id = PackageId::parse(&format!("{}@{}", package.name, package.version))?;
            let package_id = id.to_string();
            let qualified_root = mgc_lockfile::format_root_pin(package.ecosystem, &package_id);
            let deps = package
                .dependencies
                .iter()
                .map(|dependency| {
                    // npm-style `name@version` first; Go-style bare module
                    // paths (`github.com/google/uuid`) resolve their
                    // version from the same lock by name — a bare dep that
                    // names nothing in the lock fails closed.
                    // (`name@version` trước; path trần kiểu Go tra version
                    // trong cùng lock.)
                    if let Ok(id) = PackageId::parse(dependency) {
                        return Ok::<PackageId, mgc_types::MgError>(id);
                    }
                    let target = lock.packages.iter().find(|p| p.name == *dependency)
                        .ok_or_else(|| {
                            mgc_types::MgError::Other(format!(
                                "lockfile dependency '{dependency}' names no locked package (fail-closed)"
                            ))
                        })?;
                    Ok(PackageId::new(
                        mgc_types::PackageName::new(&target.name)?,
                        mgc_types::Version::parse(&target.version)?,
                    ))
                })
                .collect::<std::result::Result<Vec<_>, _>>()?;
            let peer_deps = package
                .peers
                .iter()
                .flatten()
                .map(|peer| PackageId::parse(peer))
                .collect::<std::result::Result<Vec<_>, _>>()?;
            Ok(ResolvedPackage {
                id,
                integrity: package.integrity.clone(),
                tarball_url: package.resolved.clone(),
                deps,
                peer_deps,
                direct: direct_roots.contains(&qualified_root)
                    || direct_roots.contains(&package_id),
                dev: false,
            })
        })
        .collect::<Result<Vec<_>, mgc_types::MgError>>()?;
    Ok(ResolvedGraph { packages })
}

/// Build an install graph from a v4 lock document — canonical home is
/// `mgc_lockfile::v4_graph` (shared with adapter resolve short-circuits);
/// this delegate exists so CLI call sites and tests keep one stable path.
/// (Dựng graph từ lock v4 — implementation chuẩn ở `v4_graph`.)
pub(crate) fn graph_from_v4_lockfile(
    doc: &mgc_lockfile::canonical::LockfileV4,
    owner_core: &str,
) -> mgc_lockfile::LockfileResult<ResolvedGraph> {
    mgc_lockfile::v4_graph::graph_from_v4_lockfile(doc, owner_core)
}
