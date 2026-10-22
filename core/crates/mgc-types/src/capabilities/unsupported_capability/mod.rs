//! Unsupported error constructor module wiring.
//! Wiring module hàm tạo lỗi unsupported.

mod unsupported_capability;
pub use self::unsupported_capability::unsupported_capability;
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
