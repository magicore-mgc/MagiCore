//! Wires v3 serialization and the legacy file-writing entry point.
//! Nối serialization v3 và entry ghi file legacy.

mod serialize_v3;

pub use self::serialize_v3::{
    recompute_lockfile_hash, serialize_lockfile, with_lockfile_hash, write_lockfile,
};
