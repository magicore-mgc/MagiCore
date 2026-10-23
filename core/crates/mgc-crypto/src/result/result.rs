//! Shared crypto result type alias — Alias kiểu kết quả crypto dùng chung.
//! Keep the public generic contract bound-free — Giữ contract generic public không thêm bound.

use crate::CryptoError;

/// Result type for crypto operations — Kiểu kết quả cho thao tác crypto.
pub type CryptoResult<T> = Result<T, CryptoError>;
