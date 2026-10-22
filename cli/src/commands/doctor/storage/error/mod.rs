//! Storage inspection and repair errors.
//! Lỗi kiểm tra và sửa chữa store.

mod error;

pub(in crate::commands::doctor) use error::StorageError;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
