//! Policy metadata owner facade.
//! Facade owner metadata policy.

#[path = "policy_metadata.rs"]
mod implementation;

#[cfg(any(windows, test))]
pub(super) use implementation::inspect_windows_job_processes_with_policy_metadata;
#[cfg(test)]
pub(super) use implementation::inspect_windows_job_processes_with_shadow;
