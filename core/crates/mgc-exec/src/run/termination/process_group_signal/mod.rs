//! Private owner for guarded Unix process-group signal delivery.
//! Owner riêng cho việc gửi signal Unix có kiểm tra group.

#[path = "process_group_signal.rs"]
mod implementation;

pub(super) use implementation::{signal_process_group_ids, signal_process_groups};
