//! Cache-presence error exports — xuất lỗi filesystem của owner is-cached.

mod error;

pub(super) use error::IsCachedError;

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
