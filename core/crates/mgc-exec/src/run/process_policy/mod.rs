//! Cross-platform process policy facade.
//! Facade policy process dùng chung nhiều nền tảng.

#[path = "error.rs"]
mod error;
#[path = "process_policy.rs"]
mod implementation;
#[path = "invocation_shape/mod.rs"]
mod invocation_shape;
#[path = "process_identity/mod.rs"]
mod process_identity;
#[path = "script_guard/mod.rs"]
mod script_guard;
#[path = "types/mod.rs"]
mod types;

#[cfg(any(windows, test))]
pub(super) use error::ProcessPolicyError;
#[cfg(test)]
pub(super) use implementation::forbidden_process_match;
#[cfg(any(windows, test))]
pub(super) use implementation::validate_monitored_child_command_line;
pub(super) use implementation::{
    forbidden_process_match_argv, forbidden_process_match_argv_with_scope, forbidden_process_name,
};
#[cfg(test)]
pub(super) use invocation_shape::matched_invocation_shape;
pub(super) use invocation_shape::matched_invocation_shape_argv;
pub(super) use process_identity::{is_npm_version_shell_probe, process_basename};
pub(super) use script_guard::reject_forbidden_script_file;
pub(super) use types::ForbiddenProcess;
