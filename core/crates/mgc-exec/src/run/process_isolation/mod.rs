//! Process isolation facade — export platform setup and inspection checks.
//! Facade cô lập process — export thiết lập nền tảng và kiểm tra inspector.

#[path = "process_isolation.rs"]
mod process_isolation;
#[cfg(not(any(unix, windows)))]
pub(super) use process_isolation::process_inspection_unavailable_error;
pub use process_isolation::process_tree_guard_available;
pub(super) use process_isolation::{
    configure_process_isolation, ensure_process_inspection_available,
};
