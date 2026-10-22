//! Graph comparison wiring / Nối module so khớp graph.
mod graph_match;
pub use graph_match::web_lockfile_matches_graph;
pub(crate) use graph_match::{is_web_lock_package, resolved_graphs_match_material};
