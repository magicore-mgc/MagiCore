//! Owns validation of externally supplied BLAKE3 text.
//! Quản lý xác thực chuỗi BLAKE3 từ bên ngoài.

mod external_hash;
pub use self::external_hash::validate_external_hash;
