//! Manifest validation and exportable JSON owner exports.
//! Export owner kiểm tra manifest và tạo JSON publish.

mod sanitize;

pub use sanitize::sanitize;

#[cfg(test)]
#[path = "test/sanitize.rs"]
mod tests;
