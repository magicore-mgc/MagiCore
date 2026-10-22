//! Reader wiring / Nối module reader.
mod reader;
pub(crate) use reader::{lock_on_disk_is_v4, read_web_lockfile_document, scope_web_lock_packages};
pub use reader::{read_web_lockfile, read_web_lockfile_checked};
