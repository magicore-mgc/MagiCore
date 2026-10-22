//! Public crypto error owner wiring — Wiring owner lỗi crypto public.
//! Export the stable enum and register its owner tests — Export enum ổn định và đăng ký test owner.

mod crypto_error;
pub use crypto_error::CryptoError;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
