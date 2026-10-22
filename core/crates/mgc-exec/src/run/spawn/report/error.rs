//! Typed failures for report-level exit enforcement.
//! Lỗi typed khi report áp dụng chính sách exit code.

use std::fmt;

#[derive(Debug)]
pub(super) enum ExecReportError {
    CommandExited {
        command: String,
        exit_code: i32,
        duration_ms: u64,
        output_tail: String,
    },
}

impl fmt::Display for ExecReportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CommandExited {
                command,
                exit_code,
                duration_ms,
                output_tail,
            } => write!(
                formatter,
                "'{command}' exited with code {exit_code} ({duration_ms} ms)\n--- tail ---\n{output_tail}"
            ),
        }
    }
}

impl std::error::Error for ExecReportError {}
