//! Materialization phase wiring and ordering.
//! Wiring và thứ tự phase materialization.

mod dependency_graph;
mod finalize;
mod legacy_flat;
mod materialization;
pub(super) use materialization::materialize;
