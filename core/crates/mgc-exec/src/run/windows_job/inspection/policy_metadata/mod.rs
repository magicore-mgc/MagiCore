//! Policy metadata owner facade.
//! Facade owner metadata policy.

#[path = "policy_metadata.rs"]
mod implementation;

#[cfg(any(windows, test))]
pub(in crate::run::implementation) use implementation::inspect_windows_job_processes_with_policy_metadata;
#[cfg(test)]
pub(in crate::run::implementation) use implementation::inspect_windows_job_processes_with_shadow;
