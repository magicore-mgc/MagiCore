//! Shared crypto error exports — Export lỗi crypto dùng chung.
//! Keep the public error type at its original crate-root path — Giữ type lỗi public ở root path ban đầu.

mod crypto_error;
pub use crypto_error::CryptoError;
