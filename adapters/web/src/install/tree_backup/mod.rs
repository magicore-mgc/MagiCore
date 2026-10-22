//! Tree-backup owner wiring and nearest tests.
//! Wiring owner backup cây và test gần module.

mod error;
mod tree_backup;
pub(crate) use tree_backup::TreeBackup;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
