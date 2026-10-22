//! Resolution-stage wiring / wiring các bước resolve.
pub(crate) mod compat;
mod lockfile_lookup;
mod offline_gate;
mod package_graph;
mod peer_validation;
mod publication;
mod request;
mod resolution;
mod shared_cache_lookup;
mod solver;
mod types;
mod v4_short_circuit;
