//! Wires the shared atomic file-replacement owner and platform backend.
//! Nối owner thay file atomic dùng chung và backend theo OS.

mod atomic_replace;
mod error;
#[cfg(windows)]
mod windows_replace;

pub use self::atomic_replace::{atomic_replace_file, atomic_replace_file_with_write_through};
pub use self::error::AtomicReplaceError;
#[cfg(windows)]
pub(super) use self::windows_replace::windows_replace;
