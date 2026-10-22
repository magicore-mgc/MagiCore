//! Project direct resolved package IDs / Chiếu ID package resolve trực tiếp.

use mgc_types::ResolvedGraph;

pub(crate) fn direct_package_ids(graph: &ResolvedGraph) -> Vec<String> {
    graph
        .packages
        .iter()
        .filter(|package| package.direct)
        .map(|package| package.id.to_string())
        .collect()
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
