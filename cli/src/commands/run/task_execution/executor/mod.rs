//! Guarded process-execution owner wiring.
//! Wiring owner thực thi process có rào chắn.

mod error;
mod executor;
pub(super) use executor::execute_task_with_bin;
