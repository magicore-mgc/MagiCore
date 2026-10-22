//! Test the public Terraform scanner facade.
//! Kiểm tra facade scanner Terraform public.
#![allow(clippy::unwrap_used)]

use mgc_types::adapter::ScannerStatus;

use super::{audit_terraform_lock, parse_terraform_lock};

#[test]
fn missing_lockfile_stays_unsupported() {
    let root = tempfile::tempdir().unwrap();
    let report = audit_terraform_lock(root.path()).unwrap();

    assert!(matches!(
        report.scanner_status,
        ScannerStatus::UnsupportedEcosystem { ref ecosystem }
            if ecosystem == concat!(
                "cloud/terraform (no .terraform.lock.hcl found — run terraform init ",
                "to pin provider checksums)"
            )
    ));
}

#[test]
fn malformed_header_keeps_the_public_error_text() {
    let error = parse_terraform_lock("provider \"broken").unwrap_err();

    assert_eq!(
        error.to_string(),
        "malformed provider header in .terraform.lock.hcl: provider \"broken"
    );
}
