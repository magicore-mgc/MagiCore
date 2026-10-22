//! Timeout configuration facade — export the execution timeout resolver.
//! Facade cấu hình timeout — export bộ phân giải timeout thực thi.

#[path = "timeout.rs"]
mod implementation;
pub(super) use implementation::execution_timeout;
