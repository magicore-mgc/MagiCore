//! Shared locked-graph module wiring.
//! Wiring module shared locked graph; phần xử lý nằm trong file cùng tên.
use super::*;
#[path = "locked_graph.rs"]
mod implementation;
#[cfg(test)]
pub(in crate::commands::core::shared::implementation::implementation) use implementation::graph_from_lockfile;
#[cfg(test)]
pub(crate) use implementation::{
    graph_from_v4_lockfile, lock_graph_matches_manifest_closure, v4_graph_matches_manifest_closure,
};
pub(crate) use implementation::{
    load_locked_graph, load_locked_graph_v4, lock_matches_manifest, lock_owner_core,
    locked_package_ids,
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
