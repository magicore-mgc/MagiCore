//! Public manifest result type owner exports.
//! Export type kết quả manifest công khai.

mod types;

pub use types::Sanitized;

#[cfg(test)]
#[path = "test/types.rs"]
mod tests;
