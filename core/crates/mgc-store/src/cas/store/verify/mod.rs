//! CAS blob verification, identity, quarantine, and membership owners.
//! Các owner xác minh blob CAS, identity, cách ly và membership.

mod external_hash;
mod file_identity;
mod membership;
mod open_file;
mod quarantine;
mod verified_blob;
mod verify;

pub use self::external_hash::validate_external_hash;
pub(super) use self::open_file::open_regular_file;
pub(super) use self::verify::verify_open_file;
