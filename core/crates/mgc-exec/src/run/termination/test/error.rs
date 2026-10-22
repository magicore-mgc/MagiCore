//! Termination error and source-chain tests.
//! Test lỗi termination và chuỗi source.

use super::{TerminationFailures, TerminationPhase};
use std::error::Error;

#[test]
fn termination_error_keeps_first_source_and_bounds_additional_failures() {
    let mut failures = TerminationFailures::default();
    failures.process_scan(
        TerminationPhase::TermSweep,
        anyhow::anyhow!("process table unavailable"),
    );
    failures.process_group_signal(
        4242,
        libc::SIGKILL,
        std::io::Error::other("permission denied"),
    );

    let error = failures
        .into_result()
        .expect_err("return termination failures");

    assert!(error.to_string().contains("process table unavailable"));
    assert!(error.to_string().contains("additional termination failure"));
    assert!(error.to_string().contains("permission denied"));
    assert_eq!(
        error.source().map(ToString::to_string).as_deref(),
        Some("process table unavailable")
    );
}

#[test]
fn termination_error_caps_retained_failure_details_and_reports_omitted_count() {
    let mut failures = TerminationFailures::default();
    for group in 1..=16 {
        failures.process_group_probe(group, std::io::Error::other("probe failed"));
    }

    let error = failures
        .into_result()
        .expect_err("return termination failures");

    assert!(error.to_string().contains("process group 8"));
    assert!(!error.to_string().contains("process group 9"));
    assert!(
        error
            .to_string()
            .contains("8 additional termination failure(s) omitted")
    );
}

#[test]
fn process_group_probe_error_keeps_io_source() {
    let error = super::super::TerminationError::process_group_probe(
        4242,
        std::io::Error::other("probe denied"),
    );

    assert!(error.to_string().contains("process group 4242"));
    assert_eq!(
        error.source().map(ToString::to_string).as_deref(),
        Some("probe denied")
    );
}

#[test]
fn process_group_identity_error_keeps_io_source() {
    let error = super::super::TerminationError::process_group_identity(
        4242,
        std::io::Error::other("identity denied"),
    );

    assert!(error.to_string().contains("PID 4242"));
    assert_eq!(
        error.source().map(ToString::to_string).as_deref(),
        Some("identity denied")
    );
}
