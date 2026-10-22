//! Crates graph test owner facade.
//! Facade owner test crates graph theo hành vi.

#[path = "crates_graph.rs"]
mod implementation;

pub(crate) use implementation::{
    native_engine_preserves_multiple_versions_and_exact_dependency_edges,
    resolve_with_protocol_builds_graph_and_v3_lock_entries,
    resolve_with_protocol_rejects_cross_root_version_conflicts,
};
