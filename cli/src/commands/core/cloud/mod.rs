//! Cloud core command ownership.
//! Ownership lệnh Cloud core.

#[path = "error.rs"]
mod error;
#[path = "cloud.rs"]
mod implementation;

pub use implementation::adapter;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
