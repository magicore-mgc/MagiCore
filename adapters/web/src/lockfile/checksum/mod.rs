//! Checksum wiring / Nối module checksum.
mod checksum;
pub use checksum::{
    allow_unsigned_integrity, compute_sha512_b64, compute_tarball_integrity,
    compute_tarball_integrity_reader, maybe_warn_missing_lockfile_checksum,
    strict_integrity_enforced,
};
