//! mgc-exec — exec passthrough + allowlist + audit sanitizer (MagiCore)
//! Forwards to trusted tools (allowlist), records exec audit with secret sanitization.
//! Passthrough execution with an allowlist and sanitized audit — thực thi passthrough theo allowlist và audit đã lọc bí mật.

pub mod allowlist;
pub mod audit;
pub mod parallel;
pub mod policy;
pub mod policy_manifest;
pub mod run;
pub mod sanitizer;
pub mod spawn_profile;

pub mod prelude {
    //! Mọi thứ adapter/core cần cho passthrough — một import duy nhất.
    pub use crate::allowlist::{
        ALLOWED_TOOLS, ExecutionScope, FORBIDDEN_TOOLS, QUALIFIED_COMPAT_RUNNERS, ScriptInvocation,
        check_tool, check_tool_scoped, compat_runner_allows, find_forbidden_tool_in_script,
        parse_compat_runner, parse_script_invocation, parse_simple_script,
        reject_forbidden_pm_script, reject_shell_control,
    };
    pub use crate::audit::{AuditEntry, append};
    pub use crate::policy::{
        DenyCode, Invocation, LaneProfile, PolicyContext, PolicyDecision, PolicyMode,
        ReadOnlyException, is_mutation_invocation, resolve,
    };
    pub use crate::run::{
        ExecOptions, ExecReport, clear_shutdown, ensure_signal_handlers,
        process_tree_guard_available, request_shutdown, run, run_inherited,
        run_inherited_cancellable, run_project_binary, run_project_binary_inherited,
        run_project_binary_inherited_cancellable, shutdown_requested,
    };
    pub use crate::sanitizer::redact_args;
}
