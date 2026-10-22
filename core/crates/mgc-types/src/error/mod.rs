//! Shared error API exports and nearest enum tests.
//! Tái xuất API lỗi dùng chung và test enum gần owner.

mod error;
mod mg_result;

pub use self::error::MgError;
pub use self::mg_result::MgResult;

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
