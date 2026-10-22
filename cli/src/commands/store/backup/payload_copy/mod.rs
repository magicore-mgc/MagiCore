//! Backup payload owner wiring — nối owner copy payload backup.

mod payload_copy;

pub(super) use payload_copy::copy_backup_payload;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
