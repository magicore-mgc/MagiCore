//! Exact Cargo manifest snapshot and rollback wiring.
//! Wiring snapshot và rollback Cargo manifest chính xác.
mod manifest_snapshot;

pub(super) use manifest_snapshot::{read_manifest_snapshot, restore_manifest_snapshot};
