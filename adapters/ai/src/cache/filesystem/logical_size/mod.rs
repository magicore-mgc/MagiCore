//! Logical-size exports — xuất phép đo file thường và lỗi filesystem typed.

mod error;
mod logical_size;

pub(in crate::cache) use error::LogicalSizeError;
pub(in crate::cache) use logical_size::logical_size;

#[cfg(test)]
#[path = "test/logical_size.rs"]
mod tests;
