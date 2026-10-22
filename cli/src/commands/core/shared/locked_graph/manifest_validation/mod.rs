//! Manifest validation owner wiring for lock reuse.
//! Wiring owner xác thực manifest trước khi tái sử dụng lock.

#[path = "manifest_validation.rs"]
mod implementation;

pub(crate) use implementation::{
    lock_graph_matches_manifest_closure, lock_matches_manifest, v4_graph_matches_manifest_closure,
};
