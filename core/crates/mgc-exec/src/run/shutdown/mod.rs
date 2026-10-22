//! Shutdown facade — expose process-wide request and signal-handler entry points.
//! Facade shutdown — export lệnh dừng toàn process và điểm cài signal handler.

#[path = "error.rs"]
mod error;
#[path = "shutdown.rs"]
mod implementation;
pub use implementation::{
    clear_shutdown, ensure_signal_handlers, request_shutdown, shutdown_requested,
};
