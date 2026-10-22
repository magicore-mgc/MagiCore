//! Feature graph closure owner wiring.
//! Wiring owner đóng feature graph.

mod error;
mod feature_graph;

pub(super) use error::{FeatureGraphError, FeatureGraphResult};
pub(in crate::protocols::crates) use feature_graph::resolve_feature_graph;
