//! Verified lockfile loading and legacy resolved-graph projection.
//! Đọc lockfile đã kiểm tra và chuyển legacy lock sang resolved graph.

mod locked_graph;

#[cfg(test)]
use locked_graph::graph_from_lockfile;
pub(super) use locked_graph::load_locked_graph;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
