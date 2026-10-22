//! Validate and prepare an invocation before command construction or spawn.
//! Xác thực và chuẩn bị invocation trước khi dựng command hoặc spawn.

use super::types::PreparedInvocation;
use crate::run::implementation::{
    ExecOptions, ExecReport, append, append_compat_scope, entry_from, execution_timeout,
    redact_args, validate_args_no_traversal,
};
use anyhow::Result;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

use super::super::error::SpawnError;

#[cfg(windows)]
use super::super::super::windows_command::windows_tool_path_units;
#[cfg(windows)]
use std::ffi::OsString;
#[cfg(windows)]
use std::path::PathBuf;

/// Resolve security checks, execution timeout, cwd and declared compat audit.
/// Phân giải kiểm tra bảo mật, timeout, cwd và audit compat đã khai báo.
pub(in crate::run::implementation) fn prepare<'a>(
    cmd: &str,
    args: &[String],
    opts: &'a ExecOptions,
    cancel_flag: Option<&AtomicBool>,
) -> Result<PreparedInvocation<'a>> {
    if cancel_flag.is_some_and(|flag| flag.load(Ordering::SeqCst)) {
        return Err(SpawnError::CancelledBeforeSpawn.into());
    }

    // Validate arguments before timeout parsing, cwd resolution, or any audit write.
    // Xác thực đối số trước khi parse timeout, resolve cwd hoặc ghi audit.
    validate_args_no_traversal(args, opts.cwd.as_deref())?;
    let timeout = execution_timeout(opts.disable_timeout, opts.timeout)?;

    // Redact before console, compat audit, native audit or report projection.
    // Redact trước console, compat audit, native audit hoặc report.
    let safe_args = redact_args(args);
    let cwd = match opts.cwd.clone() {
        Some(cwd) => cwd,
        None => std::env::current_dir().map_err(SpawnError::ResolveCurrentDirectory)?,
    };

    // Expand 8.3 aliases before Node/Vite compare cwd with filesystem realpaths.
    // Mở rộng alias 8.3 trước khi Node/Vite đối chiếu cwd với realpath.
    #[cfg(windows)]
    let cwd = {
        use std::os::windows::ffi::{OsStrExt, OsStringExt};
        let canonical =
            std::fs::canonicalize(&cwd).map_err(SpawnError::CanonicalizeWorkingDirectory)?;
        let units = canonical.as_os_str().encode_wide().collect::<Vec<_>>();
        PathBuf::from(OsString::from_wide(&windows_tool_path_units(&units)))
    };

    // Require and record compat scope before any command or child process starts.
    // Bắt buộc và ghi scope compat trước khi command hoặc process con bắt đầu.
    let compat_manager = opts
        .compat_runner
        .as_deref()
        .map(|raw| crate::allowlist::parse_compat_runner(Some(raw)))
        .transpose()?
        .flatten();
    let compat_log_path = if compat_manager.is_some() {
        Some(
            opts.log_path
                .as_deref()
                .ok_or(SpawnError::CompatRunnerRequiresAuditLog)?,
        )
    } else {
        None
    };
    if let (Some(manager), Some(log_path)) = (compat_manager.as_deref(), compat_log_path) {
        append_compat_scope(log_path, manager, cmd, &safe_args, &cwd, opts.dry_run)?;
        if opts.dry_run {
            eprintln!(
                concat!(
                    "WARNING: --compat-runner '{manager}' would delegate its launched process tree; ",
                    "dry-run did not spawn a process"
                ),
                manager = manager
            );
        } else {
            eprintln!(
                concat!(
                    "WARNING: --compat-runner '{manager}' delegates matching package-manager processes ",
                    "in this command's process tree; dependency/lifecycle ownership is reduced ",
                    "(scope recorded in '{log_path}')"
                ),
                manager = manager,
                log_path = log_path.display()
            );
        }
    }

    Ok(PreparedInvocation {
        safe_args,
        cwd,
        timeout,
        compat_manager,
        compat_log_path,
    })
}

/// Produce the dry-run report and optional audit row; return `None` for live execution.
/// Tạo report dry-run và audit tùy chọn; trả `None` khi sẽ chạy thật.
pub(in crate::run::implementation) fn complete(
    cmd: &str,
    safe_args: &mut Vec<String>,
    cwd: &Path,
    dry_run: bool,
    audit_log_path: Option<&Path>,
) -> Result<Option<ExecReport>> {
    if !dry_run {
        return Ok(None);
    }

    // dry-run prints only redacted args and returns before signal setup, command resolution or spawn.
    // dry-run chỉ in args đã redact và trả về trước signal setup, resolve command hoặc spawn.
    println!("[dry-run] {} {}", cmd, safe_args.join(" "));
    let report = ExecReport {
        cmd: cmd.to_string(),
        args: std::mem::take(safe_args),
        exit_code: 0,
        duration_ms: 0,
        dry_run: true,
        stdout_tail: String::new(),
        stderr_tail: String::new(),
        stdout_full: String::new(),
    };
    if let Some(path) = audit_log_path {
        append(path, &entry_from(&report, cwd))?;
    }
    Ok(Some(report))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
