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
        } else if tool.path.is_none()
            && let Some(core) = tool.required_by_core
        {
            // A missing tool that the detected core needs is a readiness gate:
            // the core cannot build without it, so it is reported as a failure.
            // Tool thiếu mà core đang cần là cổng sẵn sàng: core không build được
            // nếu thiếu, nên báo ở mức failure.
            issues.push(DiagnosticIssue {
                code: "ERR_CORE_TOOLCHAIN_MISSING",
                severity: "CRITICAL",
                message: format!(
                    "Core '{}' requires tool '{}', which is not installed or not in PATH.",
                    core, tool.name
                ),
                fix_command: Some(format!(
                    "Install '{}' so the '{}' core can build, or switch to a core whose toolchain is present",
                    tool.name, core
                )),
            });
            actions.push(format!(
                "Install '{}' required by the '{}' core",
                tool.name, core
            ));
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
