//! Tests for cargo-audit report consistency gates.
//! Kiểm thử gate consistency của report cargo-audit.

use super::super::document::CargoVulnerabilities;
use super::super::error::CargoAuditParseError;
use super::*;

fn vulnerabilities(found: bool, count: Option<usize>) -> CargoVulnerabilities {
    CargoVulnerabilities {
        found,
        count,
        list: Vec::new(),
    }
}

#[test]
fn matching_empty_report_passes_with_optional_count() {
    assert!(validate_vulnerabilities(&vulnerabilities(false, Some(0))).is_ok());
    assert!(validate_vulnerabilities(&vulnerabilities(false, None)).is_ok());
}

#[test]
fn count_and_found_mismatches_fail_with_typed_variants() {
    assert!(matches!(
        validate_vulnerabilities(&vulnerabilities(false, Some(1))),
        Err(CargoAuditParseError::CountMismatch {
            count: 1,
            actual: 0
        })
    ));
    assert!(matches!(
        validate_vulnerabilities(&vulnerabilities(true, None)),
        Err(CargoAuditParseError::FoundListMismatch)
    ));
}
