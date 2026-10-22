//! `graph_match.rs` — lockfile and resolved-graph equality.
//! so khớp lockfile và resolved graph

use mgc_lockfile::{LOCKFILE_SCHEMA_VERSION, Lockfile, Package};
use mgc_types::adapter::{ResolvedGraph, ResolvedPackage};

pub fn web_lockfile_matches_graph(lockfile: &Lockfile, graph: &ResolvedGraph) -> bool {
    // Accept both "2" (legacy on-disk locks must still short-circuit the
    // rewrite) and the current schema version ("3") that this writer emits.
    // Chấp nhận cả "2" (lock cũ trên đĩa vẫn được phép bỏ qua ghi lại) lẫn
    // version schema hiện tại ("3") mà writer này ghi ra.
    if lockfile.version != "2" && lockfile.version != LOCKFILE_SCHEMA_VERSION {
        return false;
    }

    let web_packages: Vec<_> = lockfile
        .packages
        .iter()
        .filter(|package| is_web_lock_package(lockfile, package))
        .collect();
    if web_packages.len() != graph.packages.len() {
        return false;
    }

    // Check packages match
    web_packages
        .iter()
        .zip(graph.packages.iter())
        .all(|(locked, resolved)| {
            locked.name == resolved.id.name_str()
                && locked.version == resolved.id.version().to_string()
                && locked.dependencies.len() == resolved.deps.len()
                && locked
                    .dependencies
                    .iter()
                    .zip(resolved.deps.iter())
                    .all(|(left, right)| left == &right.to_string())
                && (resolved.integrity.is_empty() || locked.integrity == resolved.integrity)
        })
}

/// v2 locks predate ecosystem tags and were web-only; in v3, ownership is
/// explicit and only `web` entries belong to this adapter.
/// (Lock v2 chưa có ecosystem nên mặc định là web; v3 chỉ nhận tag web.)
pub(crate) fn is_web_lock_package(lockfile: &Lockfile, package: &Package) -> bool {
    package.ecosystem == mgc_lockfile::EcosystemTag::Web
        || (lockfile.version == "2" && package.ecosystem == mgc_lockfile::EcosystemTag::Other)
}

/// Material-field equality between a replayed v4 graph and the installed
/// graph (id/integrity/URL/deps/peer sets). Flags the writer drops
/// (`direct`, `dev`) are excluded — v3 cannot persist them, so they can
/// never prove divergence. Order-insensitive: adjacency is a set.
/// (Bằng nhau trên trường material mà writer giữ; bỏ qua flag writer
/// không lưu; không phụ thuộc thứ tự.)
pub(crate) fn resolved_graphs_match_material(
    first: &ResolvedGraph,
    second: &ResolvedGraph,
) -> bool {
    use std::collections::HashSet;
    if first.packages.len() != second.packages.len() {
        return false;
    }
    let key_of = |package: &ResolvedPackage| {
        let mut deps: Vec<String> = package.deps.iter().map(|id| id.to_string()).collect();
        deps.sort();
        let mut peers: Vec<String> = package.peer_deps.iter().map(|id| id.to_string()).collect();
        peers.sort();
        (
            package.id.to_string(),
            package.integrity.clone(),
            package.tarball_url.clone(),
            deps,
            peers,
        )
    };
    let first_set: HashSet<_> = first.packages.iter().map(key_of).collect();
    let second_set: HashSet<_> = second.packages.iter().map(key_of).collect();
    first_set == second_set
}
