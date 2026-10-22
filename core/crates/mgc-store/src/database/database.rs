//! SQLite database API and owner wiring.
//! API SQLite database và wiring các owner.

#[path = "cas_claims/mod.rs"]
mod cas_claims;
#[path = "claim_writer_gate/mod.rs"]
mod claim_writer_gate;
#[path = "connection/mod.rs"]
mod connection;
#[path = "errors/mod.rs"]
mod errors;
#[path = "generation_guard/mod.rs"]
mod generation_guard;
#[path = "generation_lifecycle/mod.rs"]
mod generation_lifecycle;
#[path = "integrity_cache/mod.rs"]
mod integrity_cache;
#[path = "integrity_snapshot/mod.rs"]
mod integrity_snapshot;
#[path = "numeric/mod.rs"]
mod numeric;
#[path = "package_files/mod.rs"]
mod package_files;
#[path = "package_records/mod.rs"]
mod package_records;
#[path = "project_install_lock/mod.rs"]
mod project_install_lock;
#[path = "project_references/mod.rs"]
mod project_references;
#[path = "release_policy/mod.rs"]
mod release_policy;
#[path = "schema/mod.rs"]
mod schema;
#[path = "staging_leases/mod.rs"]
mod staging_leases;
#[path = "trust_policy/mod.rs"]
mod trust_policy;
#[path = "types/mod.rs"]
mod types;
#[path = "version_lookup/mod.rs"]
mod version_lookup;

pub use errors::CasGenerationError;
pub use generation_guard::CasGenerationGuard;
pub use integrity_snapshot::{CasIntegrityCounts, CasIntegritySnapshotError};
pub(crate) use numeric::u64_from_sqlite;
pub use project_install_lock::ProjectInstallLock;
pub use types::{Database, DatabaseEntry, StagingLease};
