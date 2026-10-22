//! Explicit compatibility opt-ins.
//! Các opt-in tương thích tường minh.

#[path = "compat.rs"]
mod compat;

pub use compat::CompatConfig;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
