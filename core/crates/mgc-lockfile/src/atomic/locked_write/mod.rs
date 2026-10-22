//! Wires lock-protected atomic publication and local write errors.
//! Nối publish atomic có lock và lỗi ghi cục bộ.

mod error;
mod locked_write;
pub use self::locked_write::atomic_write_locked;
