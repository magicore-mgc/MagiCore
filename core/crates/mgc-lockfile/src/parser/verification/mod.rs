//! Wires hash, key-identity, and signature verification.
//! Nối xác minh hash, identity khóa và chữ ký.

mod error;
mod verification;

pub use self::verification::load_and_verify_lockfile;
