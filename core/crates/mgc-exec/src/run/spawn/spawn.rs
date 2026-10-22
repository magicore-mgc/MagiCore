//! Shared child-process spawn, environment, audit, timeout, and output pipeline.
//! Luồng spawn process con, environment, audit, timeout và output dùng chung.

#[path = "cleanup/mod.rs"]
mod cleanup;
#[path = "command_builder/mod.rs"]
mod command_builder;
#[path = "command_environment/mod.rs"]
mod command_environment;
#[path = "error.rs"]
mod error;
#[path = "preflight/mod.rs"]
mod preflight;
#[path = "report/mod.rs"]
mod report;
use super::super::output::OutputMode;
use super::super::{
    CompatAuditContext, ExecOptions, ExecReport, NativeAuditContext, ProcessMonitor, ShadowPath,
    append, configure_process_isolation, ensure_process_inspection_available,
    ensure_signal_handlers, entry_from, wait_with_timeout,
};
#[cfg(unix)]
use super::super::{PROCESS_TREE_TRACKING_ENV, process_tree_tracking_token};
use anyhow::Result;
use error::SpawnError;
use std::sync::atomic::AtomicBool;
use std::time::Instant;

pub(in crate::run::implementation) fn execute_command(
    cmd: &str,
    args: &[String],
    opts: &ExecOptions,
    mode: OutputMode,
    cancel_flag: Option<&AtomicBool>,
    enforce_exit_code: bool,
) -> Result<ExecReport> {
    #[cfg(windows)]
    if let Some(cleanup) = super::super::windows_job::take_deferred_handle_close_error() {
        return Err(SpawnError::PendingWindowsHandleCleanup(cleanup).into());
    }

    let execution = execute_command_inner(cmd, args, opts, mode, cancel_flag, enforce_exit_code);

    #[cfg(windows)]
    let cleanup = super::super::windows_job::take_deferred_handle_close_error();
    #[cfg(not(windows))]
    let cleanup = None;

    cleanup::combine_execution_and_handle_cleanup(execution, cleanup)
}

fn execute_command_inner(
    cmd: &str,
    args: &[String],
    opts: &ExecOptions,
    mode: OutputMode,
    cancel_flag: Option<&AtomicBool>,
    enforce_exit_code: bool,
) -> Result<ExecReport> {
    let preflight::PreparedInvocation {
        mut safe_args,
        cwd,
        timeout,
        compat_manager,
        compat_log_path,
    } = preflight::prepare(cmd, args, opts, cancel_flag)?;

    // Native-deny (Spec rev4 §1/§7): forbidden PM tools stay blocked in every
    // cwd — unless this invocation declared `--compat-runner` naming exactly
    // this manager. The compat escape hatch is explicit, warned and audited;
    // every other tool keeps native-deny.
    // (Native-deny (Spec rev4 §1/§7): PM cấm vẫn bị chặn ở mọi cwd — trừ khi
    // invocation này khai báo `--compat-runner` đúng manager đó. Escape hatch
    // compat tường minh, có cảnh báo và audit; mọi tool khác giữ native-deny.)
    // No package manager or rival runtime receives a process-tree exemption
    // outside the declared compat subtree.
    let compat_exemption: Option<[&str; 1]> = compat_manager.as_deref().map(|runner| [runner]);
    let scoped_exempt: &[&str] = compat_exemption
        .as_ref()
        .map(|slots| slots.as_slice())
        .unwrap_or(&[]);

    if let Some(report) = preflight::complete(
        cmd,
        &mut safe_args,
        &cwd,
        opts.dry_run,
        opts.log_path.as_deref(),
    )? {
        return Ok(report);
    }

    // Install shutdown handlers before command/path setup can create resources.
    // Cài handler shutdown trước khi setup command/PATH có thể tạo tài nguyên.
    ensure_signal_handlers()?;

    let compat_audit = match (compat_manager.as_deref(), compat_log_path) {
        (Some(manager), Some(log_path)) => Some(CompatAuditContext {
            manager,
            log_path,
            cwd: &cwd,
            seen_processes: std::sync::Mutex::new(std::collections::HashSet::new()),
        }),
        _ => None,
    };

    let start = Instant::now();
    let mut command = command_builder::build_command(cmd, args, &cwd, opts, mode)?;
    // Isolation applies to EVERY spawn (not only clean_env): timeout must control the WHOLE tree.
    // Cô lập áp dụng cho MỌI lần spawn (không chỉ clean_env): timeout phải kiểm soát CẢ CÂY.
    let process_tree_guard = configure_process_isolation(&mut command)?;

    let mut shadow_path = ShadowPath::create(scoped_exempt)?;
    let execution = (|| -> Result<ExecReport> {
        command_environment::configure_command_environment(&mut command, opts, shadow_path.path())?;

        let monitor_forbidden_children = true;
        ensure_process_inspection_available(monitor_forbidden_children, cfg!(any(unix, windows)))?;
        #[cfg(unix)]
        let process_tree_tracking_token = process_tree_tracking_token();
        #[cfg(unix)]
        command.env(PROCESS_TREE_TRACKING_ENV, &process_tree_tracking_token);
        #[cfg_attr(not(windows), allow(unused_mut))]
        let mut child = command
            .spawn()
            .map_err(|source| SpawnError::CommandSpawnFailed {
                command: cmd.to_string(),
                source,
            })?;
        #[cfg(windows)]
        if let Err(activation) = process_tree_guard.activate(&child) {
            let activation = anyhow::Error::from(activation);
            let termination = process_tree_guard
                .terminate()
                .err()
                .map(anyhow::Error::from);
            let kill = child.kill().err();
            // Wait only after at least one stop operation succeeded; otherwise an unbounded wait
            // could hang forever while a child that resisted cleanup remains alive.
            // Chỉ wait khi ít nhất một lệnh dừng thành công; nếu không, wait vô hạn có thể treo.
            let wait = if termination.is_none() || kill.is_none() {
                child.wait().err()
            } else {
                None
            };
            return Err(SpawnError::WindowsProcessTreeSetupCleanup {
                activation,
                termination,
                kill,
                wait,
            }
            .into());
        }

        let native_audit = opts.log_path.as_deref().map(|log_path| NativeAuditContext {
            log_path,
            root_cmd: cmd,
            root_args: &safe_args,
            cwd: &cwd,
        });
        let process_monitor = ProcessMonitor {
            enabled: monitor_forbidden_children,
            execution_scope: opts
                .execution_scope
                .unwrap_or(crate::allowlist::ExecutionScope::Install),
            exempt: scoped_exempt,
            compat_audit: compat_audit.as_ref(),
            native_audit: native_audit.as_ref(),
            #[cfg(unix)]
            shadow_dir: shadow_path.path(),
            #[cfg(windows)]
            shadow_path: &shadow_path,
            #[cfg(unix)]
            tracking_token: &process_tree_tracking_token,
        };
        let outcome = wait_with_timeout(
            child,
            timeout,
            process_monitor,
            process_tree_guard,
            mode,
            cancel_flag,
        )?;
        let duration_ms = start.elapsed().as_millis() as u64;
        let exit_code = outcome.status.code().unwrap_or(-1);

        let report = report::build_exec_report(
            cmd,
            safe_args,
            exit_code,
            duration_ms,
            &outcome.stdout,
            &outcome.stderr,
            opts.capture_full_stdout,
        );
        if let Some(path) = &opts.log_path {
            append(path, &entry_from(&report, &cwd))?;
        }

        report::ensure_exit_code_allowed(&report, enforce_exit_code, &opts.allowed_exit_codes)?;
        Ok(report)
    })();
    let cleanup_result = shadow_path.cleanup();
    cleanup::combine_execution_and_environment_cleanup(execution, cleanup_result)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
