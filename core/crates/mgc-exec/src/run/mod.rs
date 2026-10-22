//! Run API facade — export stable execution entry points.
//! Facade API run — export entry point thực thi ổn định.

#[path = "run.rs"]
mod implementation;

pub use implementation::{
    ExecOptions, ExecReport, clear_shutdown, ensure_signal_handlers, process_tree_guard_available,
    request_shutdown, run, run_inherited, run_inherited_cancellable, run_project_binary,
    run_project_binary_inherited, run_project_binary_inherited_cancellable, shutdown_requested,
};
pub(crate) use implementation::{
    check_invocation_policy, compat_runner_declared, dependency_resolution_deny_reason,
};

// Keep the cancellation runner crate-internal for parallel execution.
// Giữ runner có cancellation trong crate để parallel gọi qua facade ổn định.
pub(crate) use implementation::run_cancellable_capture;
