//! Shared locked-graph owner wiring.
//! Wiring các owner nạp lock, kiểm tra manifest và chuyển graph dùng chung.

use super::*;

#[path = "graph_conversion/mod.rs"]
mod graph_conversion;
#[path = "loading/mod.rs"]
mod loading;
#[path = "manifest_validation/mod.rs"]
mod manifest_validation;

pub(in crate::commands::core::shared::implementation::implementation) use graph_conversion::graph_from_lockfile;
pub(crate) use graph_conversion::graph_from_v4_lockfile;
pub(crate) use loading::{
    load_locked_graph, load_locked_graph_v4, lock_owner_core, locked_package_ids,
};
pub(crate) use manifest_validation::{
    lock_graph_matches_manifest_closure, lock_matches_manifest, v4_graph_matches_manifest_closure,
};
