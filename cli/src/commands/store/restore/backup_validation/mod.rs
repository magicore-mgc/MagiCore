//! Wires safe source-backup validation.
//! Nối owner xác thực nguồn backup an toàn.

mod backup_validation;
mod error;

pub(super) use backup_validation::{ValidatedBackup, validate_backup};
pub(super) use error::BackupValidationError;
