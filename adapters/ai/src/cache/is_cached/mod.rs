//! Cache-presence exports — khai báo owner kiểm tra model đã có trong cache.

mod error;
mod is_cached;

pub use is_cached::is_cached;

#[cfg(test)]
#[path = "test/is_cached.rs"]
mod tests;
