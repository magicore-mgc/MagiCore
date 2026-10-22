//! Compose shared package-install policies.
//! Nối các policy install package dùng chung.

#[path = "age_gate/mod.rs"]
mod age_gate;

pub(crate) use age_gate::refresh_locked_graph_for_age_gate;
