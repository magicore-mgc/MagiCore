//! Sanitize and map executor reports to the stable audit record.
//! Redact và ánh xạ report executor sang bản ghi audit ổn định.

use super::super::ExecReport;
use crate::audit::{AuditEntry, now_ts};
use crate::sanitizer::redact_args;
use std::path::Path;

/// Build an audit record, redacting argv before it leaves the executor.
/// Tạo audit entry, redact argv trước khi dữ liệu rời executor.
pub(in crate::run) fn entry_from(report: &ExecReport, cwd: &Path) -> AuditEntry {
    AuditEntry {
        cmd: report.cmd.clone(),
        args: redact_args(&report.args),
        cwd: cwd.display().to_string(),
        exit_code: report.exit_code,
        duration_ms: report.duration_ms,
        dry_run: report.dry_run,
        ts: now_ts(),
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
