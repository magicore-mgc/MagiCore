//! Atomic write API exports.
//! Export API ghi atomic.

mod atomic_write;
mod atomic_write_if_changed;

pub use atomic_write::atomic_write;
pub use atomic_write_if_changed::atomic_write_if_changed;
