//! Test facade — register the process_isolation behavior tests.
//! Facade test — đăng ký test hành vi của process_isolation.

#[cfg(not(any(unix, windows)))]
pub(super) use super::super::process_inspection_unavailable_error;
pub(super) use super::super::{
    configure_process_isolation, ensure_process_inspection_available, process_tree_guard_available,
};

#[path = "process_isolation.rs"]
mod process_isolation;
