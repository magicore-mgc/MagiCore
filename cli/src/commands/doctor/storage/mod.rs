//! Store path and disk-space inspection.
//! Kiểm tra đường dẫn store và dung lượng đĩa.

mod error;
mod remediation;
mod storage;

pub(super) use error::StorageError;
pub(super) use remediation::{RemediationOutcome, RemediationStatus, initialize_missing};
pub(super) use storage::{StoreSnapshot, StoreStatus, inspect, inspect_at};

#[cfg(test)]
pub(super) use storage::nearest_existing_directory;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
