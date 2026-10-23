//! Shared crypto result export — Export kết quả crypto dùng chung.
//! Preserve the existing public crate-root alias — Giữ alias public ở crate root như hiện tại.

mod result;
pub use result::CryptoResult;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
