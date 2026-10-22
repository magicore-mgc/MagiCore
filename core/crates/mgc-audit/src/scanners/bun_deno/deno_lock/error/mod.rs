//! Deno parser error facade — facade lỗi parser Deno.
//! Maps typed local failures to the scanner's stable `MgError::Other` boundary.
//! Chuyển lỗi cục bộ có kiểu về boundary `MgError::Other` ổn định của scanner.

mod error;

pub(super) use error::DenoLockError;

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
