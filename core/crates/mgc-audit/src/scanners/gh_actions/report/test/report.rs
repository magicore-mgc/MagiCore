//! Test the stable GitHub Actions report projection.
//! Kiểm tra phép chiếu report GitHub Actions ổn định.
#![allow(clippy::unwrap_used)]

use mgc_types::adapter::{FindingClass, VulnerabilitySeverity};

use super::super::super::types::WorkflowFinding;
use super::build_report;

#[test]
fn report_keeps_policy_identity_severity_and_evidence() {
    let report = build_report(
        2,
        vec![WorkflowFinding {
            file: "ci.yml".to_string(),
            line: 7,
            rule: "actions-unpinned-ref",
            detail: "mutable ref".to_string(),
        }],
    );

    assert_eq!(report.packages_audited, 2);
    assert_eq!(report.vulnerability_count, 1);
    let finding = &report.vulnerabilities[0];
    assert_eq!(finding.title, "actions-unpinned-ref: mutable ref");
    assert_eq!(finding.severity, "info");
    assert_eq!(finding.cve, "actions-unpinned-ref");
    assert_eq!(finding.severity_level, VulnerabilitySeverity::Info);
    assert_eq!(finding.finding_class, FindingClass::Policy);
    assert_eq!(finding.scanner.as_deref(), Some("github-actions-policy"));
    assert_eq!(finding.ecosystem.as_deref(), Some("cicd/github-actions"));
    assert!(finding.evidence_at.is_some());
}

#[test]
fn report_skips_finding_when_workflow_path_is_not_a_package_name() {
    let report = build_report(
        1,
        vec![WorkflowFinding {
            file: "../invalid workflow.yml".to_string(),
            line: 0,
            rule: "permissions-undeclared",
            detail: "no permissions".to_string(),
        }],
    );

    assert_eq!(report.packages_audited, 1);
    assert_eq!(report.vulnerability_count, 0);
    assert!(report.vulnerabilities.is_empty());
}
