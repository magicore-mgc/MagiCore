//! Wires legacy lock signing and its local error category.
//! Nối ký lock legacy và phân loại lỗi cục bộ.

mod error;
mod legacy_signing;

pub use self::legacy_signing::{sign_and_write_lockfile, sign_lockfile_with_default_key};
