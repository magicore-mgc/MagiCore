//! Build sanitized execution reports and enforce declared exit-code policy.
//! Dựng report đã làm sạch và áp dụng chính sách exit code đã khai báo.

use super::super::super::super::ExecReport;
use super::super::super::super::output::{full, tail};
use super::error::ExecReportError;

/// Convert child output and status into the stable public report shape.
/// Chuyển output và trạng thái child sang cấu trúc report public ổn định.
pub(in crate::run::implementation::spawn::implementation) fn build_exec_report(
    cmd: &str,
    args: Vec<String>,
    exit_code: i32,
    duration_ms: u64,
    stdout: &[u8],
    stderr: &[u8],
    capture_full_stdout: bool,
) -> ExecReport {
    ExecReport {
        cmd: cmd.to_string(),
        args,
        exit_code,
        duration_ms,
        dry_run: false,
        stdout_tail: tail(stdout),
        stderr_tail: tail(stderr),
        stdout_full: if capture_full_stdout {
            full(stdout)
        } else {
            String::new()
        },
    }
}

/// Reject a nonzero exit unless the caller explicitly allowed that code.
/// Từ chối exit code khác 0 trừ khi caller đã cho phép mã đó.
pub(in crate::run::implementation::spawn::implementation) fn ensure_exit_code_allowed(
    report: &ExecReport,
    enforce_exit_code: bool,
    allowed_exit_codes: &[i32],
) -> anyhow::Result<()> {
    if !enforce_exit_code || report.exit_code == 0 || allowed_exit_codes.contains(&report.exit_code)
    {
        return Ok(());
    }

    let output_tail = if report.stderr_tail.is_empty() {
        report.stdout_tail.clone()
    } else {
        report.stderr_tail.clone()
    };
    Err(ExecReportError::CommandExited {
        command: report.cmd.clone(),
        exit_code: report.exit_code,
        duration_ms: report.duration_ms,
        output_tail,
    }
    .into())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
