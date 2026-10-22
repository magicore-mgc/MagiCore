//! Guarded capture and inherited-stdio execution for project/package binaries.
//! Chạy binary project/package có guard với output capture hoặc stdio kế thừa.

use super::super::output::OutputMode;
use super::super::{
    ExecOptions, ExecReport, compat_runner_exempt, execute_command, manifest_exception_allows,
    process_basename, reject_external_dependency_resolution, reject_forbidden_script_file,
    self_answered_report,
};
use anyhow::{Result, bail};
use std::path::Path;
use std::sync::atomic::AtomicBool;

/// Run a concrete project/package binary without the static host-tool allowlist.
/// Scope-sensitive device/runtime denies for the root executable and package
/// mutation guards still apply.
/// The caller's `ExecOptions` control cwd, environment, timeout and optional audit.
///
/// Chạy binary project/package cụ thể, không áp allowlist tĩnh của tool host.
/// Vẫn áp deny scope cho executable gốc với lệnh thiết bị/runtime và guard package.
/// `ExecOptions` của caller quyết định cwd, env, timeout và audit tùy chọn.
pub fn run_project_binary(path: &Path, args: &[String], opts: &ExecOptions) -> Result<ExecReport> {
    execute_project_binary(path, args, opts, OutputMode::Capture, None)
}

/// Run a project/package binary with inherited stdio.
/// Chạy binary project/package trực tiếp với stdio kế thừa.
pub fn run_project_binary_inherited(
    path: &Path,
    args: &[String],
    opts: &ExecOptions,
) -> Result<ExecReport> {
    execute_project_binary(path, args, opts, OutputMode::Inherit, None)
}

/// Run a project binary with inherited stdio and supervisor cancellation.
/// Chạy binary project với stdio trực tiếp và hỗ trợ supervisor hủy process tree.
pub fn run_project_binary_inherited_cancellable(
    path: &Path,
    args: &[String],
    opts: &ExecOptions,
    cancel_flag: &AtomicBool,
) -> Result<ExecReport> {
    execute_project_binary(path, args, opts, OutputMode::Inherit, Some(cancel_flag))
}

fn execute_project_binary(
    path: &Path,
    args: &[String],
    opts: &ExecOptions,
    mode: OutputMode,
    cancel_flag: Option<&AtomicBool>,
) -> Result<ExecReport> {
    let canonical = path.canonicalize().map_err(|error| {
        anyhow::anyhow!(
            "project binary '{}' is not executable: {error}",
            path.display()
        )
    })?;
    if !canonical.is_file() {
        bail!("project binary '{}' is not a file", canonical.display());
    }

    let canonical_command = canonical.display().to_string();
    let basename = process_basename(&canonical_command);
    let requested_basename = process_basename(&path.display().to_string());
    let compat_exempt = compat_runner_exempt(&canonical_command, opts);
    // Self-answered probes return before any deny (any program, like run()).
    // (Probe tự trả lời trả về trước mọi deny (mọi program, như run()).)
    if !compat_exempt && let Some(answer) = crate::policy::self_answer_for(&canonical_command, args)
    {
        return self_answered_report(&canonical_command, args, opts, &answer);
    }
    let forbidden_name = [requested_basename.as_str(), basename.as_str()]
        .into_iter()
        .find(|name| crate::allowlist::FORBIDDEN_TOOLS.contains(name));
    if !compat_exempt && let Some(forbidden_name) = forbidden_name {
        // Exact manifest exceptions lift only the name-based deny (verb and
        // script guards below still run).
        // (Exception manifest chính xác chỉ gỡ deny theo tên (guard verb và
        // script dưới vẫn chạy).)
        if manifest_exception_allows(&canonical_command, args, opts) {
            eprintln!(
                "WARNING: exact policy exception permits '{}'; verb and script guards still apply (audit-logged)",
                canonical.display()
            );
        } else {
            bail!(
                "project binary '{}' resolves to forbidden package manager '{}'",
                canonical.display(),
                forbidden_name
            );
        }
    }

    if !compat_exempt {
        // Canonicalization must not erase restrictions attached to the name
        // the caller selected, even when a package shim points at another file.
        // Canonicalize không được xóa giới hạn của tên caller chọn dù shim
        // package trỏ sang file có tên khác.
        crate::allowlist::check_project_binary_scope(
            &requested_basename,
            opts.execution_scope
                .unwrap_or(crate::allowlist::ExecutionScope::Install),
        )?;
        reject_external_dependency_resolution(&canonical_command, args, &opts.env)?;
        reject_forbidden_script_file(&canonical_command)?;
        crate::allowlist::check_project_binary_scope(
            &basename,
            opts.execution_scope
                .unwrap_or(crate::allowlist::ExecutionScope::Install),
        )?;
    }
    execute_command(&canonical_command, args, opts, mode, cancel_flag, true)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
