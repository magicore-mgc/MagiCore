//! Cargo source checksum owner wiring.
//! Wiring owner checksum source Cargo.

#[path = "source_checksum.rs"]
mod implementation;

pub(super) use implementation::validate_sha256;
pub(crate) use implementation::{verify_source_checksum, write_source_checksum};
