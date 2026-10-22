//! Cargo feature graph facade.
//! Facade graph feature Cargo.

mod activation;
mod feature_graph;
mod resolved_entry;

pub(in crate::protocols::crates) use feature_graph::resolve_feature_graph;
