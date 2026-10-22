//! Private owner for pure Unix signal-target planning.
//! Owner riêng cho policy thuần chọn đích signal Unix.

#[path = "process_signal_plan.rs"]
mod implementation;
#[path = "types.rs"]
mod types;

pub(super) use implementation::process_signal_plan;
