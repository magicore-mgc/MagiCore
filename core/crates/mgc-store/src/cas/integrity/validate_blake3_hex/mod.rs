//! Hash validation owner wiring and local error boundary.
//! Wiring owner validation hash và ranh giới lỗi cục bộ.

mod error;
#[path = "validate_blake3_hex.rs"]
mod implementation;

pub use self::error::{BLAKE3_HEX_LEN, InvalidHashError};
pub use self::implementation::validate_blake3_hex;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
