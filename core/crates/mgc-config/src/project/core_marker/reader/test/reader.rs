//! Core marker read-flow tests.
//! Test luồng đọc core marker.
use super::super::read_core_marker;
use crate::project::project::core_marker::error::{CoreMarkerError, RecoveryAction};

#[test]
fn malformed_marker_fails_closed_before_attestation() {
    let project = tempfile::tempdir().expect("temporary project should be created");
    std::fs::write(project.path().join(".mgc.core"), "unknown-core\n")
        .expect("marker should be written");

    let error = read_core_marker(project.path()).expect_err("unknown marker should be rejected");

    assert!(matches!(error, CoreMarkerError::InvalidMarker { .. }));
    assert_eq!(
        error.recovery_action(),
        RecoveryAction::RepairProjectIdentity
    );
}
