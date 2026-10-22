//! Model path error exports — xuất lỗi validation riêng cho owner tạo path.

mod error;

pub(super) use error::ModelCachePathError;

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
