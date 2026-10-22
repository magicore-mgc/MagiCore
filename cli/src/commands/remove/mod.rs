//! Stable public routes for the core-neutral remove entry.
//! Giữ route công khai ổn định cho entry remove dùng chung.

mod dispatch;
mod remove;

pub use remove::run_many;
