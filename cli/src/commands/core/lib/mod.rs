//! Library core command ownership.
//! Ownership lệnh Lib core.

#[path = "error.rs"]
mod error;
#[path = "lib.rs"]
mod implementation;

pub use implementation::adapter;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
