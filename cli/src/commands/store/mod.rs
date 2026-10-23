//! CLI store command facade.
//! Facade lệnh store của CLI.

mod backup;
mod backup_storage;
mod command_runner;
mod command_types;
mod doctor_database_integrity;
mod doctor_dispatch;
mod doctor_report;
mod doctor_root_safety;
mod doctor_staging_repair;
mod doctor_temp_repair;
mod doctor_verdict;
mod global_root_doctor;
mod maintenance_lock;
mod path_overlap;
mod process_liveness;
mod project_ownership;
mod prune;
mod reclaim;
mod restore;
mod route_selection;
mod staging_lease;
mod store_paths;
mod store_status;
mod web_doctor;

pub use command_runner::run;
pub use command_types::{PruneReport, ReclaimCandidate, ReclaimReport, StoreCmd};
#[cfg(test)]
use global_root_doctor::inspect_shared_root;
pub(super) use maintenance_lock::{MaintenanceLockError, acquire_maintenance_lock};
pub(super) use path_overlap::paths_overlap;
#[cfg(test)]
use project_ownership::ensure_web_store_owner;
#[cfg(test)]
use project_ownership::store_framework_name_matches;
#[cfg(test)]
use route_selection::{
    LIB_CARGO_ROOTS, LIB_GO_ROOTS, LIB_MAVEN_ROOTS, LIB_NUGET_ROOTS, LIB_PYPI_ROOTS, StoreRoute,
    WEB_NATIVE_FRAMEWORKS, store_route,
};
#[cfg(test)]
use staging_lease::{StagingDisposition, classify_staging_lease, classify_staging_lease_pure};
#[cfg(test)]
#[path = "../../test/store_test.rs"]
mod tests;
