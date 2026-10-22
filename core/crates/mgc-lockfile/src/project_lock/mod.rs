//! Wires the stable project-lock API to acquisition and path-safety owners.
//! Nối API project lock ổn định tới owner acquire và an toàn đường dẫn.

mod contention;
mod error;
mod path_safety;
mod project_lock;

pub use self::path_safety::path_is_link_or_reparse;
pub use self::project_lock::{ProjectWriteLock, WRITE_LOCK_FILE};
