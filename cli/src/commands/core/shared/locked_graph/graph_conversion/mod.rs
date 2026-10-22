//! Resolved-graph conversion owner wiring.
//! Wiring owner chuyển dữ liệu sang resolved graph.

#[path = "graph_conversion.rs"]
mod implementation;

pub(in crate::commands::core::shared::implementation::implementation) use implementation::graph_from_lockfile;
pub(crate) use implementation::graph_from_v4_lockfile;
