//! Windows Job member inspection facade.
//! Facade kiểm tra member của Windows Job.

#[path = "error.rs"]
mod error;
#[path = "inspection.rs"]
mod implementation;
mod npm_probe;

#[cfg(windows)]
pub(in crate::run::implementation) use implementation::windows_job_has_active_descendants;
#[cfg(windows)]
pub(in crate::run::implementation) use implementation::{
    find_forbidden_job_descendant_with_timeout, windows_job_descendants,
};
