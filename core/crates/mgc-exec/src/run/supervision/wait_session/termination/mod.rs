//! Process-tree termination helpers and typed-failure aggregation exports.
//! Export helper dừng cây process và gộp lỗi typed.

#[path = "termination.rs"]
mod implementation;

pub(super) use implementation::{kill_child, termination_failure};
