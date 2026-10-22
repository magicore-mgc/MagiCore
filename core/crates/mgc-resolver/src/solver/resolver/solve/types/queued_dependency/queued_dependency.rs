//! Retain request provenance while dependencies move through the resolver queue.
//! Giữ nguồn gốc request khi dependency đi qua queue resolver.

use mgc_types::PackageName;

/// A queued package request, optionally tied to one root-manifest position.
/// Request package trong queue, có thể gắn với vị trí trong manifest root.
#[derive(Debug, Clone)]
pub(in crate::solver::resolver::solve) struct QueuedDependency {
    pub(in crate::solver::resolver::solve) name: PackageName,
    pub(in crate::solver::resolver::solve) spec: String,
    pub(in crate::solver::resolver::solve) root_index: Option<usize>,
}

impl QueuedDependency {
    pub(in crate::solver::resolver::solve) fn root(
        index: usize,
        name: PackageName,
        spec: String,
    ) -> Self {
        Self {
            name,
            spec,
            root_index: Some(index),
        }
    }

    pub(in crate::solver::resolver::solve) fn transitive(name: PackageName, spec: String) -> Self {
        Self {
            name,
            spec,
            root_index: None,
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
