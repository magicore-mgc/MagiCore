//! File descriptor budget owner.
//! Owner ngân sách file descriptor.

#[path = "fd_budget.rs"]
mod implementation;

pub use implementation::{descriptor_budget, global_descriptor_semaphore};
