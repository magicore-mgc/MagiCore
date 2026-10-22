//! Lock-document loading owner wiring.
//! Wiring owner nạp tài liệu lock.

#[path = "error/mod.rs"]
mod error;
#[path = "loading.rs"]
mod implementation;

pub(crate) use implementation::{
    load_locked_graph, load_locked_graph_v4, lock_owner_core, locked_package_ids,
};
