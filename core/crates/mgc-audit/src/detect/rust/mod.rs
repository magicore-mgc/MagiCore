//! Wire the Rust detector and its nearest tests.
//! Nối detector Rust với test gần owner nhất.

#[path = "rust.rs"]
mod rust;

pub use rust::rust_step;

#[cfg(test)]
#[path = "test/rust.rs"]
mod tests;
