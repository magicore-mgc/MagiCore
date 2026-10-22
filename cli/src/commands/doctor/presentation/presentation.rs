//! Renders the stable terminal and JSON doctor report formats.
//! Render format report doctor ổn định cho terminal và JSON.

use std::fmt::Write as _;

use serde::Serialize;

use super::super::{error::DoctorError, model::DoctorReport, storage::RemediationOutcome};

#[derive(Serialize)]
struct ReportOutput<'a> {
    #[serde(flatten)]
    report: &'a DoctorReport,
    remediation: &'a RemediationOutcome,
}

pub(in crate::commands::doctor) fn render_json(
    report: &DoctorReport,
    remediation: Option<&RemediationOutcome>,
) -> Result<String, DoctorError> {
    match remediation {
        Some(remediation) => Ok(serde_json::to_string_pretty(&ReportOutput {
            report,
            remediation,
        })?),
        None => Ok(serde_json::to_string_pretty(report)?),
    }
}

pub(in crate::commands::doctor) fn render_human(
    report: &DoctorReport,
    remediation: Option<&RemediationOutcome>,
) -> String {
    let mut output = String::new();
    let _ = writeln!(
        output,
        "MagiCore Doctor (v{}) — Environment Health: {}",
        env!("CARGO_PKG_VERSION"),
        report.health_status
    );
    let _ = writeln!(
        output,
        "─────────────────────────────────────────────────────────────"
    );
    for tool in &report.toolchain {
        let mark = if tool.path.is_some() {
            "  ✓ OK  "
        } else {
            "  ✗ MISS"
        };
        let _ = writeln!(
            output,
            "{mark} {:<10} {:<12} {}",
            tool.name,
            tool.version,
            tool.path.as_deref().unwrap_or("")
        );
    }
    let _ = writeln!(
        output,
        "  {} Store: {} (writable: {})",
        if report.store_writable { "✓" } else { "✗" },
        report.store_dir,
        report.store_writable
    );
    if report.disk_free_bytes > 0 {
        let _ = writeln!(
            output,
            "  ℹ Disk Free: {} MB",
            report.disk_free_bytes / (1024 * 1024)
        );
    }
    for registry in &report.registries_reachable {
        let mark = if registry.reachable { "✓" } else { "✗" };
        let _ = writeln!(output, "  {mark} Registry: {}", registry.host);
    }
    if !report.detected_issues.is_empty() {
        let _ = writeln!(output, "\nDetected Issues & AI Recommendations:");
        let _ = writeln!(
            output,
            "─────────────────────────────────────────────────────────────"
        );
        for issue in &report.detected_issues {
            let _ = writeln!(
                output,
                "[{}] {}: {}",
                issue.severity, issue.code, issue.message
            );
            if let Some(fix_command) = &issue.fix_command {
                let _ = writeln!(output, "  ↳ Fix: {fix_command}");
            }
        }
    }
    if let Some(remediation) = remediation {
        let _ = writeln!(
            output,
            "\nRemediation [{}]: {}",
            remediation.status.as_str(),
            remediation.message
        );
    }
    output
}
