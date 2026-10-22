//! Wiring for CAS integrity snapshot contract tests.
//! Nối test hợp đồng snapshot toàn vẹn CAS.

#[path = "integrity_snapshot.rs"]
mod integrity_snapshot;
pub(crate) use integrity_snapshot::*;
