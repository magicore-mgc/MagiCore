//! Alias the crate's shared fallible result.
//! Alias kiểu kết quả có lỗi dùng chung của crate.

use crate::error::MgError;

/// Shared fallible result type used across crates and adapters.
/// Kiểu kết quả có lỗi dùng chung giữa các crate và adapter.
pub type MgResult<T> = Result<T, MgError>;
