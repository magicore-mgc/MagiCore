//! Project workflow policy findings into the shared audit report.
//! Chiếu workflow policy findings vào audit report dùng chung.

use mgc_types::adapter::{AuditReport, FindingClass, Vulnerability, VulnerabilitySeverity};
use mgc_types::{PackageId, PackageName, Version};

use super::super::types::WorkflowFinding;

/// Findings → report. Policy violations are severity Info findings (the
/// lane reports them for review; it is not a CVE database).
/// Finding → report. Vi phạm policy là finding Info (lane báo để rà;
/// không phải database CVE).
pub(crate) fn build_report(scanned: usize, findings: Vec<WorkflowFinding>) -> AuditReport {
    let mut vulnerabilities = Vec::new();
    for finding in &findings {
        // Package identity for a workflow finding: the workflow file
        // itself (version carries the line number).
        // Danh tính package của finding workflow: chính file workflow
        // (version mang số dòng).
        let Ok(name) = PackageName::new(finding.file.clone()) else {
            continue;
        };
        vulnerabilities.push(
            Vulnerability {
                package: PackageId::new(name, Version::new(finding.line as u64, 0, 0)),
                title: format!("{}: {}", finding.rule, finding.detail),
                severity: "info".to_string(),
                cve: finding.rule.to_string(),
                severity_level: VulnerabilitySeverity::Info,
                patched_versions: None,
                url: None,
                scanner: None,
                ecosystem: None,
                evidence_at: None,
                finding_class: FindingClass::Policy,
            }
            .with_evidence("github-actions-policy", "cicd/github-actions"),
        );
    }
    AuditReport {
        packages_audited: scanned,
        vulnerability_count: vulnerabilities.len(),
        vulnerabilities,
        scanner_status: mgc_types::adapter::ScannerStatus::Available,
    }
}

#[cfg(test)]
#[path = "test/report.rs"]
mod tests;
