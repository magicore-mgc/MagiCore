//! Lockfile migration diagnostics.
//! Lỗi migrate lockfile.

mod error;

pub use error::{
    migrate_no_lockfile, migrate_unknown_target, migrate_unsupported_version, migrate_v4_lossy,
    migrate_v4_roundtrip_mismatch,
};
