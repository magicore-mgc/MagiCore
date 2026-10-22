//! Graph operations for Web dependency materialization.
//! Thao tác graph thuộc materialize dependency Web.

use mgc_types::PackageId;
use mgc_types::adapter::{ResolvedGraph, ResolvedPackage};

pub fn select_root_packages(graph: &ResolvedGraph) -> Vec<&ResolvedPackage> {
    let direct_packages = graph
        .packages
        .iter()
        .filter(|pkg| pkg.direct)
        .collect::<Vec<_>>();
    let mut selected: std::collections::HashMap<String, &ResolvedPackage> =
        std::collections::HashMap::new();

    for pkg in &graph.packages {
        selected
            .entry(pkg.id.name_str().to_string())
            .and_modify(|current| {
                let prefer_candidate = (pkg.direct && !current.direct)
                    || (pkg.direct == current.direct && pkg.id.version() > current.id.version());
                if prefer_candidate {
                    *current = pkg;
                }
            })
            .or_insert(pkg);
    }

    if !direct_packages.is_empty() {
        let single_version_names: std::collections::HashSet<String> = graph
            .packages
            .iter()
            .map(|pkg| pkg.id.name_str().to_string())
            .fold(
                std::collections::HashMap::<String, usize>::new(),
                |mut counts, name| {
                    *counts.entry(name).or_insert(0) += 1;
                    counts
                },
            )
            .into_iter()
            .filter(|(_, count)| *count == 1)
            .map(|(name, _)| name)
            .collect();
        return graph
            .packages
            .iter()
            .filter(|pkg| {
                let name = pkg.id.name_str();
                let is_selected_version = selected
                    .get(name)
                    .is_some_and(|selected| selected.id == pkg.id);
                is_selected_version && (pkg.direct || single_version_names.contains(name))
            })
            .collect();
    }

    graph
        .packages
        .iter()
        .filter(|pkg| {
            selected
                .get(pkg.id.name_str())
                .map(|selected| selected.id == pkg.id)
                .unwrap_or(false)
        })
        .collect()
}

pub fn graph_without_packages(
    graph: &ResolvedGraph,
    excluded: &std::collections::HashSet<PackageId>,
) -> ResolvedGraph {
    ResolvedGraph {
        packages: graph
            .packages
            .iter()
            .filter(|pkg| !excluded.contains(&pkg.id))
            .cloned()
            .collect(),
    }
}

#[cfg(test)]
#[path = "test/graph.rs"]
mod tests;
