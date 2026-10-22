//! Shared install-policy module facade.
//! Facade module policy install dùng chung.

#[path = "install_policy.rs"]
mod implementation;

pub(crate) use implementation::refresh_locked_graph_for_age_gate;
