//! Backup destination validation owner wiring — nối owner xác thực đích backup.

mod destination_validation;

pub(super) use destination_validation::{regular_file_len, validate_backup_destination};

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub(in crate::commands::store::backup) use destination_validation::prepare_backup_destination;
