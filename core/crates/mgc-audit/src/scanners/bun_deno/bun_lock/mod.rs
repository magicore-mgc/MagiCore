//! Bun lockfile owner — owner đọc lockfile Bun.
//! Owns JSONC parsing and Bun-specific malformed-entry diagnostics.
//! Sở hữu parse JSONC và diagnostic riêng cho entry Bun sai cấu trúc.

mod bun_lock;
mod error;

pub use bun_lock::read_bun_lock;

#[cfg(test)]
#[path = "test/bun_lock.rs"]
mod tests;
