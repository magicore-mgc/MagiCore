//! Supervision facade — export child wait and timeout handling.
//! Facade giám sát — export xử lý chờ và timeout process con.

#[path = "diagnostics/mod.rs"]
mod diagnostics;
#[path = "supervision.rs"]
mod implementation;
pub(super) use diagnostics::{forbidden_child_error, timeout_error};
pub(super) use implementation::wait_with_timeout;
