//! Wires archive staging and staged-file data types.
//! Nối bước staging archive và các kiểu file đã staging.

mod cas_spool;
pub(super) use self::cas_spool::{StagedArchive, spool_archive};
#[cfg(test)]
mod test;
