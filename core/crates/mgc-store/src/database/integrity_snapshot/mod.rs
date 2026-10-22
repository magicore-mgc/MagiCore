//! Read-only CAS integrity count API wiring.
//! Wiring API chỉ đọc số liệu toàn vẹn CAS.

#[path = "error.rs"]
mod error;
#[path = "integrity_snapshot.rs"]
mod integrity_snapshot;

pub use error::CasIntegritySnapshotError;
pub use integrity_snapshot::CasIntegrityCounts;
