//! Minimum-release-age install policy facade.
//! Facade policy minimum-release-age cho install.

#[path = "error.rs"]
mod error;
#[path = "age_gate.rs"]
mod implementation;

#[cfg(test)]
pub(crate) use implementation::age_gate_lock_action;
pub(crate) use implementation::refresh_locked_graph_for_age_gate;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
