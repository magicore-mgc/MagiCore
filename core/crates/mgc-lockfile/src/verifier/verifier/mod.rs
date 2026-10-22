//! Wires public verification entry points and version dispatch.
//! Nối entry point xác minh public và dispatch theo version.

mod verifier;

pub use self::verifier::{verify_lockfile, verify_lockfile_with_trust};
