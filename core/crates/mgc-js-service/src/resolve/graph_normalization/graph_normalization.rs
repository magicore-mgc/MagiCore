//! Mark project roots and remove graph packages unreachable from direct roots.
//! Đánh dấu project root và bỏ package không tới được từ root trực tiếp.

use super::super::resolved_manifest::is_manifest_root_package;
use mgc_types::Manifest;
use mgc_types::adapter::ResolvedGraph;

pub fn prune_unreachable_packages(graph: &mut ResolvedGraph) {
    let indexes: std::collections::HashMap<_, _> = graph
        .packages
        .iter()
        .enumerate()
        .map(|(index, package)| (package.id.to_string(), index))
        .collect();
    let mut pending: Vec<_> = graph
        .packages
        .iter()
        .filter(|package| package.direct)
        .map(|package| package.id.to_string())
        .collect();
    let mut reachable = std::collections::HashSet::new();
    while let Some(id) = pending.pop() {
        if !reachable.insert(id.clone()) {
            continue;
        }
        let Some(index) = indexes.get(&id).copied() else {
            continue;
        };
        let package = &graph.packages[index];
        pending.extend(package.deps.iter().map(ToString::to_string));
        pending.extend(package.peer_deps.iter().map(ToString::to_string));
    }
    graph
        .packages
        .retain(|package| reachable.contains(&package.id.to_string()));
}

pub fn normalize_resolved_graph(manifest: &Manifest, mut graph: ResolvedGraph) -> ResolvedGraph {
    // A range can match both a root and transitive instances. Preserve an
    // explicit resolver root first (including an override outside the range);
    // only older graph inputs without direct flags use range inference.
    // Range có thể khớp root và nhiều bản transitive. Ưu tiên root tường minh
    // từ resolver (kể cả override ngoài range); chỉ graph cũ thiếu cờ direct
    // mới phải suy luận theo range.
    let mut candidates_by_name: std::collections::HashMap<
        String,
        Vec<&mgc_types::adapter::ResolvedPackage>,
    > = std::collections::HashMap::new();
    for package in &graph.packages {
        if package.direct || is_manifest_root_package(manifest, &package.id) {
            candidates_by_name
                .entry(package.id.name_str().to_string())
                .or_default()
                .push(package);
        }
    }
    let mut root_ids = std::collections::HashSet::new();
    for candidates in candidates_by_name.into_values() {
        let explicit_roots: Vec<_> = candidates
            .iter()
            .copied()
            .filter(|package| package.direct)
            .collect();
        let preferred = if explicit_roots.is_empty() {
            candidates
        } else {
            explicit_roots
        };
        if let Some(root) = preferred
            .into_iter()
            .max_by(|left, right| left.id.version().cmp(right.id.version()))
        {
            root_ids.insert(root.id.clone());
        }
    }
    for package in &mut graph.packages {
        package.direct = root_ids.contains(&package.id);
    }
    prune_unreachable_packages(&mut graph);
    graph
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
