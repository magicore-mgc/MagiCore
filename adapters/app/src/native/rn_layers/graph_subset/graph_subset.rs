//! Retain packages accepted by a tier ownership predicate.
//! Giữ package được predicate ownership tier chấp nhận.

use mgc_types::ResolvedGraph;

/// Return a graph containing packages whose names pass `keep`.
/// Trả graph gồm package có tên thỏa predicate `keep`.
pub(in crate::native::rn_layers) fn subset(
    graph: &ResolvedGraph,
    keep: impl Fn(&str) -> bool,
) -> ResolvedGraph {
    ResolvedGraph {
        packages: graph
            .packages
            .iter()
            .filter(|package| keep(package.id.name_str()))
            .cloned()
            .collect(),
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
