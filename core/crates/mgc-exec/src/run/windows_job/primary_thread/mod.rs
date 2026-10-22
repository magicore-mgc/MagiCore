//! Suspended child primary-thread discovery and resume facade.
//! Facade tìm và resume primary thread đang bị suspend của child.

#[path = "primary_thread.rs"]
mod implementation;

#[cfg(windows)]
pub(in crate::run::implementation::windows_job::implementation) use implementation::resume_primary_thread;
