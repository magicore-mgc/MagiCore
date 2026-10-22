//! Test GitHub Actions workflow discovery and audit aggregation.
//! Kiểm tra tìm workflow GitHub Actions và tổng hợp audit.
#![allow(clippy::unwrap_used)]

use mgc_types::adapter::ScannerStatus;

use super::audit_github_actions;

#[test]
fn missing_workflows_directory_is_reported_as_unsupported() {
    let root = tempfile::tempdir().unwrap();
    let report = audit_github_actions(root.path()).unwrap();
    assert!(matches!(
        report.scanner_status,
        ScannerStatus::UnsupportedEcosystem { ref ecosystem }
            if ecosystem == "cicd/github-actions (no .github/workflows directory found)"
    ));
}

#[test]
fn audit_scans_only_lowercase_yaml_extensions() {
    let root = tempfile::tempdir().unwrap();
    let workflows = root.path().join(".github").join("workflows");
    std::fs::create_dir_all(&workflows).unwrap();
    let clean = r#"permissions:
  contents: read
jobs:
  check:
    steps:
      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683
"#;
    std::fs::write(workflows.join("ci.yml"), clean).unwrap();
    std::fs::write(workflows.join("release.yaml"), clean).unwrap();
    std::fs::write(workflows.join("ignored.txt"), "uses: actions/checkout@v4\n").unwrap();
    std::fs::write(workflows.join("ignored.YML"), "uses: actions/checkout@v4\n").unwrap();

    let report = audit_github_actions(root.path()).unwrap();
    assert_eq!(report.packages_audited, 2);
    assert!(report.vulnerabilities.is_empty());
    assert_eq!(report.vulnerability_count, 0);
}
