//! Produces warnings for missing required tools and unavailable version probes.
//! Tạo cảnh báo khi thiếu tool bắt buộc hoặc không đọc được version.

use crate::commands::doctor::{DiagnosticIssue, Tool};

pub(in crate::commands::doctor::analysis) fn diagnostics(
    toolchain: &[Tool],
) -> (Vec<DiagnosticIssue>, Vec<String>) {
    let mut issues = Vec::new();
    let mut actions = Vec::new();
    for tool in toolchain {
        if tool.path.is_none() && matches!(tool.name, "git" | "node") {
            issues.push(DiagnosticIssue {
                code: "ERR_TOOLCHAIN_MISSING",
                severity: "WARNING",
                message: format!("Core tool '{}' is not installed or not in PATH.", tool.name),
                fix_command: Some(format!(
                    "Install '{}' via system package manager (brew/apt/winget)",
                    tool.name
                )),
            });
            actions.push(format!("Please install {}", tool.name));
        } else if tool.path.is_some() && tool.version == "?" {
            issues.push(DiagnosticIssue {
                code: "WARN_TOOLCHAIN_VERSION_UNAVAILABLE",
                severity: "WARNING",
                message: format!(
                    "Tool '{}' was found but its MGC-managed version probe failed, timed out, or produced no output.",
                    tool.name
                ),
                fix_command: None,
            });
            actions.push(format!(
                "Check that '{}' responds to its fixed version probe.",
                tool.name
            ));
        }
    }
    (issues, actions)
}
