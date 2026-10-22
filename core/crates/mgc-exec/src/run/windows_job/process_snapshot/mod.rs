//! Bounded Job Object process-ID snapshot facade.
//! Facade snapshot PID của Job Object với retry và capacity hữu hạn.

#[path = "process_snapshot.rs"]
mod implementation;

#[cfg(windows)]
pub(in crate::run::implementation::windows_job::implementation) use implementation::active_process_ids;
