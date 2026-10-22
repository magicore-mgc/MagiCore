//! Integrity error owner wiring — Wiring owner lỗi integrity.

mod error;
pub(super) use error::IntegrityError;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
