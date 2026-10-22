//! Converts a v4 lock document to an install graph without collapsing package identity; validates package references, artifact URLs and installer-enforced SRI.
//! Chuyển lock v4 thành install graph mà không gộp identity; chặn package reference, URL artifact và SRI không hợp lệ.

use crate::LockfileResult;
use crate::canonical::LockfileV4;
use crate::policy::check_install_sri;
use crate::root_pin::{format_root_pin, parse_root_pin};
use crate::v4::EdgeKind;
use crate::v4_graph::error::error::unusable_for_install;
use crate::v4_graph::v4_root_pins_for_owner::v4_root_pins_for_owner::v4_root_pins_for_owner;
use mgc_types::{PackageId, ResolvedGraph, ResolvedPackage};
use std::collections::{HashMap, HashSet};

/// Build an install graph from a v4 lock document WITHOUT flattening v4
/// identity — every lossy shape fails closed instead of guessing:
/// duplicate `name@version` instances (distinct variant/source would
/// collapse into one node), pins without installer-enforced SRI or
/// without an artifact URL (the materializer could never verify them),
/// edges naming no locked package, and unparseable ids all refuse.
/// Edge kinds map losslessly: Normal/Optional/Dev → `deps`,
/// Peer → `peer_deps`; `dev` = reachable from owner roots only through
/// Dev edges (roots themselves are never dev).
/// (Dựng graph install từ lock v4 KHÔNG ép identity — shape hao hụt nào
/// cũng fail-closed thay vì đoán.)
pub fn graph_from_v4_lockfile(doc: &LockfileV4, owner_core: &str) -> LockfileResult<ResolvedGraph> {
    // 1. Index by name@version; a second instance would collapse.
    // Lập chỉ mục theo name@version; bản ghi khác identity trùng key sẽ bị gộp mất.
    let mut index: HashMap<String, usize> = HashMap::new();
    for (position, package) in doc.packages.iter().enumerate() {
        let id = format!("{}@{}", package.key.name, package.key.version);
        if index.insert(id.clone(), position).is_some() {
            return Err(unusable_for_install(format!(
                "duplicate locked instance '{id}' (distinct variant/source cannot share one graph node)"
            )));
        }
    }

    // 2. Owner roots (qualified pins normalize to plain ids).
    // Đọc root theo owner và chuẩn hóa các pin có scope về package id.
    let raw_roots = v4_root_pins_for_owner(doc, owner_core)?;
    let scoped_roots = !doc.root_dependencies_by_owner.is_empty() || !raw_roots.is_empty();
    let root_ids: HashSet<String> = raw_roots
        .iter()
        .map(|root| parse_root_pin(root).package_id.to_string())
        .collect();

    // 3. Adjacency for edge mapping + dev reachability.
    // Lập danh sách kề để ánh xạ cạnh và tính khả năng tiếp cận của dev dependency.
    let mut adjacency: Vec<Vec<(usize, bool)>> = vec![Vec::new(); doc.packages.len()];
    for (position, package) in doc.packages.iter().enumerate() {
        for edge in &package.edges {
            let target_id = format!("{}@{}", edge.target_key.name, edge.target_key.version);
            let Some(&target) = index.get(&target_id) else {
                return Err(unusable_for_install(format!(
                    "edge from '{}@{}' names no locked package ('{target_id}')",
                    package.key.name, package.key.version
                )));
            };
            adjacency[position].push((target, matches!(edge.kind, EdgeKind::Dev)));
        }
    }
    // Reachable through any edge vs reachable without Dev edges.
    // Tính riêng đường đi qua mọi cạnh và đường đi không qua cạnh Dev.
    let mut reachable = vec![false; doc.packages.len()];
    let mut prod_reachable = vec![false; doc.packages.len()];
    let mut stack = Vec::new();
    for id in &root_ids {
        stack.push(*index.get(id).ok_or_else(|| {
            unusable_for_install(format!("owner root '{id}' names no locked package"))
        })?);
    }
    for &root in &stack {
        reachable[root] = true;
        prod_reachable[root] = true;
    }
    while let Some(current) = stack.pop() {
        for &(target, is_dev) in &adjacency[current] {
            if !reachable[target] {
                reachable[target] = true;
                stack.push(target);
            }
            if prod_reachable[current] && !is_dev && !prod_reachable[target] {
                prod_reachable[target] = true;
                stack.push(target);
            }
        }
    }

    if scoped_roots {
        // Validate union coverage before filtering: replay must not silently discard orphan pins.
        // Kiểm phủ hợp root trước khi lọc để replay không lặng bỏ pin mồ côi.
        let all_roots: Vec<&String> = if doc.root_dependencies_by_owner.is_empty() {
            raw_roots.iter().collect()
        } else {
            doc.root_dependencies_by_owner.values().flatten().collect()
        };
        let mut covered = vec![false; doc.packages.len()];
        let mut pending = Vec::new();
        for root in all_roots {
            let id = parse_root_pin(root).package_id;
            pending.push(*index.get(id).ok_or_else(|| {
                unusable_for_install(format!("owner root '{root}' names no locked package"))
            })?);
        }
        while let Some(position) = pending.pop() {
            if covered[position] {
                continue;
            }
            covered[position] = true;
            pending.extend(adjacency[position].iter().map(|(target, _)| *target));
        }
        if let Some(position) = covered.iter().position(|covered| !covered) {
            let package = &doc.packages[position];
            return Err(unusable_for_install(format!(
                "locked pin '{}@{}' is not reachable from any root ownership entry",
                package.key.name, package.key.version
            )));
        }
    }

    // 4. Packages.
    // Tạo các package thuộc install graph.
    let mut packages = Vec::with_capacity(doc.packages.len());
    for (position, package) in doc.packages.iter().enumerate() {
        // Only this owner's closure is installed; sibling artifacts are not executable inputs.
        // Chỉ cài closure của owner đang chọn; artifact sibling không thuộc đầu vào thực thi.
        if scoped_roots && !reachable[position] {
            continue;
        }
        let id = format!("{}@{}", package.key.name, package.key.version);
        let package_id = PackageId::parse(&id).map_err(|error| {
            unusable_for_install(format!(
                "locked instance '{id}' is not a valid package id: {error}"
            ))
        })?;
        let artifact = package.artifact.as_ref().ok_or_else(|| {
            unusable_for_install(format!(
                "pin '{id}' has no artifact reference (no URL to fetch)"
            ))
        })?;
        if artifact.url.is_empty() {
            return Err(unusable_for_install(format!(
                "pin '{id}' has an empty artifact URL"
            )));
        }
        let Some(sri) = artifact.integrity_sri.as_deref() else {
            return Err(unusable_for_install(format!(
                "pin '{id}' has no installer-enforced SRI (re-resolve a lane that supplies registry SRI)"
            )));
        };
        if let Err(error) = check_install_sri(sri) {
            return Err(unusable_for_install(format!("pin '{id}' carries {error}")));
        }
        let mut deps = Vec::new();
        let mut peer_deps = Vec::new();
        for edge in &package.edges {
            let target_id = format!("{}@{}", edge.target_key.name, edge.target_key.version);
            let target = PackageId::parse(&target_id).map_err(|error| {
                unusable_for_install(format!(
                    "edge target '{target_id}' is not a valid package id: {error}"
                ))
            })?;
            match edge.kind {
                EdgeKind::Normal | EdgeKind::Optional | EdgeKind::Dev => deps.push(target),
                EdgeKind::Peer => peer_deps.push(target),
            }
        }
        let qualified_root = format_root_pin(package.key.ecosystem, &id);
        packages.push(ResolvedPackage {
            id: package_id,
            integrity: sri.to_string(),
            tarball_url: artifact.url.clone(),
            deps,
            peer_deps,
            direct: root_ids.contains(&id) || raw_roots.contains(&qualified_root),
            dev: reachable[position] && !prod_reachable[position],
        });
    }
    Ok(ResolvedGraph { packages })
}
