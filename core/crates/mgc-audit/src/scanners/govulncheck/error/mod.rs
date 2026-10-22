//! Local typed Go scanner errors.
//! (Lỗi typed cục bộ của scanner Go.)

mod error;

pub(super) use error::GovulncheckError;

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
