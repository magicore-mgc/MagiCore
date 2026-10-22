//! Reflink module wiring and stable public exports.
//! Nối module reflink và giữ nguyên các export public ổn định.

mod error;
#[cfg(target_os = "macos")]
mod macos_clone;
mod reflink;
#[cfg(all(unix, not(target_os = "macos")))]
mod unix_clone;
#[cfg(not(unix))]
mod unsupported;

pub use self::error::ReflinkError;
pub use self::reflink::reflink_clone;
