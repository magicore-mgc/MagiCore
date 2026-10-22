//! Tests stable diagnostics for outdated query errors.
//! Kiểm tra thông báo lỗi ổn định của truy vấn outdated.

#![allow(clippy::unwrap_used)]

use super::super::OutdatedError;
use std::error::Error as _;

#[test]
fn current_pin_cardinality_error_keeps_the_existing_diagnostic() {
    let error = OutdatedError::CurrentDirectCardinality {
        package: "runtime".to_string(),
        found: 0,
    };
    assert_eq!(
        error.to_string(),
        "native outdated needs exactly one current locked version for direct dependency 'runtime' (found 0)"
    );
    assert!(error.source().is_none());
}

#[test]
fn latest_pin_cardinality_error_keeps_the_existing_diagnostic() {
    let error = OutdatedError::LatestDirectCardinality {
        package: "runtime".to_string(),
        found: 2,
    };
    assert_eq!(
        error.to_string(),
        "native outdated resolver did not return exactly one latest version for direct dependency 'runtime' (found 2)"
    );
    assert!(error.source().is_none());
}

#[test]
fn downgrade_error_keeps_the_existing_diagnostic() {
    let error = OutdatedError::Downgrade {
        package: "runtime".to_string(),
        current: "2.0.0".to_string(),
        latest: "1.9.0".to_string(),
    };
    assert_eq!(
        error.to_string(),
        "native outdated resolver returned 1.9.0 below locked version 2.0.0 for 'runtime'; refusing to report a downgrade"
    );
    assert!(error.source().is_none());
}
