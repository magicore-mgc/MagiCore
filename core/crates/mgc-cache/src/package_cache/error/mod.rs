//! Cache-local typed errors.
//! Lỗi có kiểu thuộc riêng cache.

mod error;

pub(super) use error::{CacheError, LockOperation};
