//! Report owner facade — export report construction and exit policy.
//! Facade owner report — export dựng report và chính sách exit code.

#[path = "error.rs"]
mod error;
#[path = "report.rs"]
mod implementation;

pub(super) use implementation::{build_exec_report, ensure_exit_code_allowed};
