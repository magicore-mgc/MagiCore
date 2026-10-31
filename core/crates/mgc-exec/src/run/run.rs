//! Exec runner — chạy tool qua allowlist, args Vec riêng (00-index §5.5–§5.8)
//! (passthrough run: dry-run, audit log, không shell injection, fail → bail kèm log trích)

use crate::allowlist::FORBIDDEN_TOOLS;
use crate::audit::{append, append_compat_scope, append_native_child_denial};
use crate::sanitizer::redact_args;
use anyhow::{Result, bail};
use std::sync::atomic::{AtomicBool, Ordering};

#[path = "environment/mod.rs"]
pub(in crate::run::implementation) mod environment;
// Consumers of these live in `run::implementation` sub-modules (the Windows job
// inspection, the spawn paths), so `pub(super)` — which stops at `run` — was too
// narrow for the Windows branches. macOS never compiled them.
// (Consumer của các item này nằm trong các sub-module `run::implementation`
// (Windows job inspection, spawn), nên `pub(super)` — dừng ở `run` — quá hẹp
// cho nhánh Windows. macOS không biên dịch nhánh đó.)
pub(in crate::run::implementation) use environment::{
    EnvironmentError, ShadowPath, guarded_path_env, is_path_env_key,
};
#[cfg(unix)]
pub(in crate::run::implementation) use environment::{
    NPM_VERSION_PROBE_SHIM_CONTENT, is_generated_shadow_directory,
};
#[cfg(windows)]
pub(in crate::run::implementation) use environment::{
    windows_current_executable_sha256, windows_sha256_file,
};

const WAIT_POLL_INTERVAL_MS: u64 = 20;
/// Bound Job Object re-enumeration while descendants are changing during inspection.
/// Giới hạn số lần liệt kê lại Job Object khi descendants thay đổi trong lúc kiểm tra.
#[cfg(any(windows, test))]
const WINDOWS_JOB_MEMBER_SNAPSHOT_RETRIES: usize = 20;
#[cfg(any(windows, test))]
const WINDOWS_COMMAND_LINE_RETRY_DELAY_MS: u64 = 25;
/// Deadline for draining the output pipes AFTER the tree kill. Once the
/// group is SIGKILLed every write end closes and EOF arrives in
/// milliseconds; the deadline only exists so a tool that ESCAPED the group
/// (setsid) can never hang us past the timeout again (P0-3, 2026-09-15).
/// (Deadline tiêu pipe SAU khi kill cây. Group bị SIGKILL thì mọi đầu ghi
/// đóng, EOF tới trong vài ms; deadline tồn tại để tool THOÁT khỏi group
/// (setsid) không thể treo ta quá hạn timeout lần nữa.)
const POST_KILL_DRAIN_MS: u64 = 200;
/// Report whether process-tree kill is active on this platform.
/// Unix: the child runs in its own process group and the timeout signals
/// the WHOLE group. Windows: a Job Object owns the suspended-at-start child
/// tree and terminates it when the guard closes. Both platforms can reap
/// grandchildren; the old /proc walk
/// matched nothing on macOS (no /proc) and only ever killed the direct
/// child (P0-3, 2026-09-15).
/// Báo rõ nền tảng hiện tại có guard kill process-tree thật hay không.
/// Unix: child chạy trong process group riêng và timeout signal CẢ group.
/// Windows: Job Object giữ cả cây process và dọn cây khi guard đóng.
/// Cả hai nền tảng đều dọn được grandchild; walk /proc cũ trên macOS không khớp gì
/// (không có /proc) và chỉ giết được child trực tiếp.
#[path = "types/mod.rs"]
pub(in crate::run::implementation) mod types;
pub use types::{ExecOptions, ExecReport};

#[path = "project_binary/mod.rs"]
pub(in crate::run::implementation) mod project_binary;
pub use project_binary::{
    run_project_binary, run_project_binary_inherited, run_project_binary_inherited_cancellable,
};

#[path = "process_isolation/mod.rs"]
pub(in crate::run::implementation) mod process_isolation;
#[cfg(not(any(unix, windows)))]
use process_isolation::process_inspection_unavailable_error;
pub use process_isolation::process_tree_guard_available;
use process_isolation::{configure_process_isolation, ensure_process_inspection_available};

#[path = "shutdown/mod.rs"]
pub(in crate::run::implementation) mod shutdown;
pub use shutdown::{clear_shutdown, ensure_signal_handlers, request_shutdown, shutdown_requested};

#[path = "spawn/mod.rs"]
pub(in crate::run::implementation) mod spawn;
use spawn::execute_command;

#[path = "timeout/mod.rs"]
pub(in crate::run::implementation) mod timeout;
use timeout::execution_timeout;

#[path = "supervision/mod.rs"]
pub(in crate::run::implementation) mod supervision;
use supervision::{forbidden_child_error, timeout_error, wait_with_timeout};

#[path = "validation/mod.rs"]
pub(in crate::run::implementation) mod validation;
use validation::validate_args_no_traversal;

#[path = "termination/mod.rs"]
#[cfg(unix)]
pub(in crate::run::implementation) mod termination;
#[cfg(unix)]
use termination::{process_groups_have_descendants, terminate_process_tree_tracked};

#[path = "output/mod.rs"]
pub(in crate::run::implementation) mod output;
use output::{OutputMode, read_stream, tail};

#[path = "windows_job/mod.rs"]
pub(in crate::run::implementation) mod windows_job;
use windows_job::ProcessTreeGuard;

#[path = "process_policy/mod.rs"]
pub(in crate::run::implementation) mod process_policy;
#[cfg(test)]
use process_policy::forbidden_process_match;
#[cfg(test)]
use process_policy::matched_invocation_shape;
#[cfg(any(windows, test))]
use process_policy::validate_monitored_child_command_line;
use process_policy::{
    ForbiddenProcess, forbidden_process_match_argv_with_scope, is_npm_version_shell_probe,
    matched_invocation_shape_argv, process_basename,
};

#[path = "invocation_policy/mod.rs"]
pub(in crate::run::implementation) mod invocation_policy;
pub(crate) use invocation_policy::{
    check_invocation_policy, compat_runner_declared, compat_runner_exempt,
};
use invocation_policy::{manifest_exception_allows, self_answered_report};

#[path = "audit_entry/mod.rs"]
pub(in crate::run::implementation) mod audit_entry;
pub(super) use audit_entry::entry_from;

#[path = "dependency_policy/mod.rs"]
pub(in crate::run::implementation) mod dependency_policy;
#[cfg(test)]
use dependency_policy::is_go_toolchain_version_probe;
#[path = "process_scan/mod.rs"]
pub(in crate::run::implementation) mod process_scan;
#[cfg(unix)]
#[cfg(unix)]
pub(in crate::run::implementation) use process_scan::RootExitScan;
#[cfg(unix)]
use process_scan::find_forbidden_descendant_tracking;
#[cfg(windows)]
use process_scan::{WindowsProcessScan, WindowsProcessSnapshot};

#[path = "process_monitor/mod.rs"]
pub(in crate::run::implementation) mod process_monitor;
use process_monitor::{
    CompatAuditContext, NativeAuditContext, ProcessMonitor, audit_compat_process,
};
#[cfg(unix)]
use process_monitor::{PROCESS_TREE_TRACKING_ENV, process_tree_tracking_token};

#[path = "rustc_child/mod.rs"]
pub(in crate::run::implementation) mod rustc_child;
use rustc_child::{
    is_rustc_compilation_argv, is_rustc_toolchain_child, is_rustc_toolchain_wrapper_argv,
    is_rustc_wrapper_argv, is_rustdoc_test_compilation_argv,
};

#[path = "runtime_invocation/mod.rs"]
pub(in crate::run::implementation) mod runtime_invocation;
use runtime_invocation::{
    argv_text, forbidden_entry_name, process_invocation_program,
    runtime_invocation as parse_runtime_invocation, windows_cmd_invocation,
};

/// Preserve the crate-visible run facade for script-interpreter screening.
/// Giữ facade run crate-visible cho bước kiểm tra interpreter của script.
pub(crate) fn reject_forbidden_script_file(cmd: &str) -> Result<()> {
    process_policy::reject_forbidden_script_file(cmd)
}

/// Chạy `cmd args` sau khi check allowlist. Không dùng shell — args là Vec riêng (§5.6).
pub fn run(cmd: &str, args: &[String], opts: &ExecOptions) -> Result<ExecReport> {
    if !compat_runner_exempt(cmd, opts)
        && let Some(answer) = check_invocation_policy(cmd, args, opts)?
    {
        return self_answered_report(cmd, args, opts, &answer);
    }
    execute_command(cmd, args, opts, OutputMode::Capture, None, true)
}

/// Run a preflighted parallel task with the shared process-tree monitor.
/// Chạy task song song đã preflight qua monitor cây tiến trình dùng chung.
pub(crate) fn run_cancellable_capture(
    cmd: &str,
    args: &[String],
    opts: &ExecOptions,
    cancel_flag: &AtomicBool,
) -> Result<ExecReport> {
    if cancel_flag.load(Ordering::SeqCst) {
        bail!("execution cancelled by its supervisor before preflight");
    }
    if !compat_runner_exempt(cmd, opts)
        && let Some(answer) = check_invocation_policy(cmd, args, opts)?
    {
        return self_answered_report(cmd, args, opts, &answer);
    }
    execute_command(
        cmd,
        args,
        opts,
        OutputMode::Capture,
        Some(cancel_flag),
        false,
    )
}

/// Run an allowlisted tool with inherited stdio for streaming commands.
/// On Unix, the private session intentionally does not inherit `/dev/tty` or job control.
/// Chạy tool allowlist với stdio trực tiếp cho lệnh streaming.
/// Trên Unix, session riêng không kế thừa `/dev/tty` hoặc job control.
pub fn run_inherited(cmd: &str, args: &[String], opts: &ExecOptions) -> Result<ExecReport> {
    run_inherited_with_cancel(cmd, args, opts, None)
}

/// Run an allowlisted tool with inherited stdio and supervisor cancellation.
/// Chạy tool allowlist với stdio trực tiếp và hỗ trợ supervisor hủy process tree.
pub fn run_inherited_cancellable(
    cmd: &str,
    args: &[String],
    opts: &ExecOptions,
    cancel_flag: &AtomicBool,
) -> Result<ExecReport> {
    run_inherited_with_cancel(cmd, args, opts, Some(cancel_flag))
}

fn run_inherited_with_cancel(
    cmd: &str,
    args: &[String],
    opts: &ExecOptions,
    cancel_flag: Option<&AtomicBool>,
) -> Result<ExecReport> {
    if !compat_runner_exempt(cmd, opts)
        && let Some(answer) = check_invocation_policy(cmd, args, opts)?
    {
        return self_answered_report(cmd, args, opts, &answer);
    }
    execute_command(cmd, args, opts, OutputMode::Inherit, cancel_flag, true)
}

/// Reject dependency resolution through the stable runner API.
/// Từ chối resolve dependency qua API runner ổn định.
pub(crate) fn reject_external_dependency_resolution(
    command: &str,
    args: &[String],
    env: &[(String, String)],
) -> Result<()> {
    dependency_policy::reject_external_dependency_resolution(command, args, env)
        .map_err(anyhow::Error::new)
}

/// Shared reason query used by execution and policy-resolver callers.
/// Truy vấn reason dùng chung cho execution và policy-resolver.
pub(crate) fn dependency_resolution_deny_reason(
    command: &str,
    args: &[String],
    env: &[(String, String)],
) -> Option<String> {
    dependency_policy::dependency_resolution_deny_reason(command, args, env)
}
