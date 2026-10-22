//! Wires stale lock-temp discovery and cleanup.
//! Nối việc tìm và dọn file temp lock cũ.

mod stale_temp;
pub use self::stale_temp::cleanup_stale_temps;
pub(in crate::atomic) use self::stale_temp::temp_name;
