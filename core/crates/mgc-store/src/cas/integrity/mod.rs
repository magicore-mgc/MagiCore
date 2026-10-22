//! CAS integrity API wiring; keeps the former `cas::integrity` paths stable.
//! Wiring API integrity CAS; giữ các path `cas::integrity` cũ ổn định.

#[path = "integrity_hash/mod.rs"]
mod integrity_hash;
#[path = "tarball_entry/mod.rs"]
mod tarball_entry;
#[path = "validate_blake3_hex/mod.rs"]
mod validate_blake3_hex;

pub use self::integrity_hash::IntegrityHash;
pub use self::tarball_entry::TarballEntry;
pub use self::validate_blake3_hex::{BLAKE3_HEX_LEN, InvalidHashError, validate_blake3_hex};
