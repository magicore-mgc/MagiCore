//! Unix process termination facade and focused behavior owners.
//! Facade termination Unix và các owner hành vi riêng.

#[path = "error.rs"]
mod error;
pub(super) use error::TerminationError;
#[path = "termination.rs"]
mod implementation;
#[path = "process_group_probe/mod.rs"]
mod process_group_probe;
#[path = "process_group_signal/mod.rs"]
mod process_group_signal;
#[path = "process_signal_plan/mod.rs"]
mod process_signal_plan;
#[cfg(all(unix, test))]
pub(super) use implementation::terminate_process_tree;
pub(super) use implementation::terminate_process_tree_tracked;
pub(super) use process_group_probe::{
    process_group_id, process_group_is_live, process_groups_have_descendants,
};
