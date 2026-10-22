//! Typed, redacted failures for Windows command preparation.
//! Lỗi typed, đã lược dữ liệu cho bước chuẩn bị lệnh Windows.

use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::run::implementation) enum WindowsCommandError {
    UnsupportedBatchSyntax,
    SystemRootUnavailable,
    SystemToolUnavailable,
}

impl fmt::Display for WindowsCommandError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::UnsupportedBatchSyntax => {
                "batch-script command contains unsupported shell metacharacters"
            }
            Self::SystemRootUnavailable => "SystemRoot is unavailable",
            Self::SystemToolUnavailable => "Windows system tool is unavailable",
        })
    }
}

impl std::error::Error for WindowsCommandError {}
