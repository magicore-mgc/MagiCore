//! Wires the stable atomic-lockfile API to behavior owners.
//! Nối API atomic ổn định tới các owner hành vi.

mod failpoint;
mod locked_write;
mod replace_file;
mod stale_temp;
mod v4_write;

pub use self::failpoint::{LOCK_FAILPOINTS, lock_failpoint};
pub use self::locked_write::atomic_write_locked;
pub use self::replace_file::atomic_replace_file;
pub use self::stale_temp::cleanup_stale_temps;
pub use self::v4_write::{atomic_write_v4_bytes, ensure_v4_unsigned_for_rewrite};
