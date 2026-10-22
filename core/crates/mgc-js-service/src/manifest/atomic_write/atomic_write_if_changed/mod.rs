//! Write-if-changed owner exports.
//! Export owner chỉ ghi khi khác.

mod atomic_write_if_changed;

pub use atomic_write_if_changed::atomic_write_if_changed;
