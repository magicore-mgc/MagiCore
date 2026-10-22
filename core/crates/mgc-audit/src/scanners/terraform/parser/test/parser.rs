//! Test Terraform lock parser coordination.
//! Kiểm tra điều phối parser Terraform lock.
#![allow(clippy::unwrap_used)]

use super::parse;
use mgc_types::adapter::ScannerStatus;

#[test]
fn valid_providers_keep_the_existing_count() {
    let raw = r#"provider "registry.terraform.io/hashicorp/aws" {
  hashes = ["h1:one"]
}

provider "registry.terraform.io/hashicorp/null" {
  hashes = ["h1:two"]
}
"#;
    let report = parse(raw).unwrap();

    assert_eq!(report.packages_audited, 2);
    assert!(matches!(report.scanner_status, ScannerStatus::Available));
}

#[test]
fn multiple_missing_provider_checksums_remain_aggregated_in_order() {
    let raw = r#"provider "registry.terraform.io/hashicorp/aws" {
}
provider "registry.terraform.io/hashicorp/null" {
}
"#;
    let report = parse(raw).unwrap();

    match report.scanner_status {
        ScannerStatus::Failed { scanner, reason } => {
            assert_eq!(scanner.as_str(), "terraform-provider-lock");
            assert_eq!(
                reason.as_str(),
                concat!(
                    "provider 'registry.terraform.io/hashicorp/aws' has no checksums; ",
                    "provider 'registry.terraform.io/hashicorp/null' has no checksums"
                )
            );
        }
        other => panic!("expected Failed, got {other:?}"),
    }
}
