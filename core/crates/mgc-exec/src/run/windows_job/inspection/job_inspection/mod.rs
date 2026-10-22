//! Native Job inspection owner facade.
//! Facade owner kiểm tra Job native.

#[path = "job_inspection.rs"]
mod implementation;

#[cfg(windows)]
pub(super) use implementation::{inspect_windows_job, windows_job_descendants};
