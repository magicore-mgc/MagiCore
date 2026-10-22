//! Wires v3 serialization and the legacy file-writing entry point.
//! Nối serialization v3 và entry ghi file legacy.

mod serialize_v3;

pub(in crate::writer) use self::serialize_v3::write_lockfile_unchecked;
pub use self::serialize_v3::{serialize_lockfile, write_lockfile};
