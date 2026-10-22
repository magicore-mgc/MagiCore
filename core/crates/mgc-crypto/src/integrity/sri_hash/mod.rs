//! SRI value owner wiring — Kết nối module sở hữu giá trị SRI.

mod sri_hash;
pub use sri_hash::SriHash;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
