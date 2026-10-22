//! Logical-size error exports — xuất lỗi phép đo cache filesystem.

mod error;

pub(in crate::cache) use error::{LogicalSizeError, LogicalSizeResult};

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
