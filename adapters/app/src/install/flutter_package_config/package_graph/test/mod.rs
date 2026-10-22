//! Wire package-graph coordinator tests.
//! Nối test coordinator package graph.

#![allow(clippy::unwrap_used)]

use super::*;
use mgc_types::adapter::ResolvedGraph;

#[path = "root_first_graph.rs"]
mod root_first_graph;
#[path = "sdk_root_required.rs"]
mod sdk_root_required;
mod support;
