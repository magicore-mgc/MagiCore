//! Snapshot restoration module wiring.
//! Wiring module phục hồi snapshot.

#[path = "restore.rs"]
mod implementation;

pub(crate) use implementation::restore_mutation_snapshot;
