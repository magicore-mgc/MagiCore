//! Empty resolved-graph install transaction wiring.
//! Wiring transaction install khi graph đã resolve bị rỗng.

mod empty_graph;

pub(in crate::install) use empty_graph::install_empty_resolved_graph;
