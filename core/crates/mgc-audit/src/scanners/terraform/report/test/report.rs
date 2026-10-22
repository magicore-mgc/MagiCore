//! Test Terraform report projection.
//! Kiểm tra ánh xạ report Terraform.
#![allow(clippy::unwrap_used)]

use super::project;
use crate::scanners::terraform::types::TerraformLockState;
use mgc_types::adapter::ScannerStatus;

#[test]
fn no_failures_project_to_available_without_vulnerabilities() {
    let state = TerraformLockState {
        providers: 3,
        ..TerraformLockState::default()
    };
    let report = project(state);

    assert_eq!(report.packages_audited, 3);
    assert_eq!(report.vulnerability_count, 0);
    assert!(report.vulnerabilities.is_empty());
    assert!(matches!(report.scanner_status, ScannerStatus::Available));
}

#[test]
fn failures_project_to_the_existing_scanner_failure() {
    let state = TerraformLockState {
        providers: 2,
        failures: vec!["first".to_string(), "second".to_string()],
        ..TerraformLockState::default()
    };
    let report = project(state);

    assert_eq!(report.packages_audited, 0);
    assert!(report.vulnerabilities.is_empty());
    assert!(matches!(
        report.scanner_status,
        ScannerStatus::Failed { ref scanner, ref reason }
            if scanner == "terraform-provider-lock" && reason == "first; second"
    ));
    assert_eq!(report.vulnerability_count, 0);
}
