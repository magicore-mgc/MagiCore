//! Cache-size errors — xuất lỗi root và filesystem của cache-size owner.

mod error;

pub(super) use error::CacheSizeError;

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
