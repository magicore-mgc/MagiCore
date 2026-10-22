//! Wire typed cargo-audit parser errors and their tests.
//! Nối lỗi parser cargo-audit typed và test.

#[path = "error.rs"]
mod error;

pub(super) use self::error::CargoAuditParseError;

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
