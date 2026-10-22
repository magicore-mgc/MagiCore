//! Read workflow files and coordinate the GitHub Actions audit.
//! Đọc file workflow và điều phối audit GitHub Actions.

use mgc_types::adapter::AuditReport;
use mgc_types::{MgError, MgResult};
use std::path::Path;

use super::report::build_report;
use super::workflow_text::scan_workflow_text;

/// Scan every workflow under .github/workflows. No workflows → honest
/// unsupported for the cicd lane (nothing to verify).
/// Quét mọi workflow dưới .github/workflows. Không có workflow →
/// unsupported trung thực cho lane cicd (không có gì để kiểm).
pub fn audit_github_actions(project_root: &Path) -> MgResult<AuditReport> {
    let workflows_dir = project_root.join(".github").join("workflows");
    if !workflows_dir.is_dir() {
        return Ok(AuditReport::unsupported_ecosystem(
            "cicd/github-actions (no .github/workflows directory found)",
        ));
    }
    let mut findings = Vec::new();
    let mut scanned = 0usize;
    let mut entries: Vec<_> = std::fs::read_dir(&workflows_dir)
        .map_err(|e| MgError::Other(format!("read workflows dir: {e}")))?
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.extension().and_then(|e| e.to_str()) == Some("yml")
                || p.extension().and_then(|e| e.to_str()) == Some("yaml")
        })
        .collect();
    entries.sort();
    for path in &entries {
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("workflow.yml")
            .to_string();
        let raw = crate::scanners::read_bounded_project_input(path, "workflow file")?;
        findings.extend(scan_workflow_text(&file_name, &raw)?);
        scanned += 1;
    }
    Ok(build_report(scanned, findings))
}

#[cfg(test)]
#[path = "test/gh_actions.rs"]
mod tests;
