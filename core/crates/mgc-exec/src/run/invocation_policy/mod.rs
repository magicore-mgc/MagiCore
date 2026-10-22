//! Pre-spawn invocation policy facade.
//! Facade policy invocation trước spawn.

#[path = "invocation_policy.rs"]
mod implementation;

pub(crate) use implementation::{
    check_invocation_policy, compat_runner_declared, compat_runner_exempt,
};
pub(super) use implementation::{manifest_exception_allows, self_answered_report};
