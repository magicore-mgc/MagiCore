#![allow(clippy::unwrap_used)]

use super::super::*;

#[test]
fn reports_absent_slot_before_project_validation() {
    let project = tempfile::tempdir().expect("project directory");
    let identities = tempfile::tempdir().expect("identities directory");
    assert_eq!(
        verify(project.path(), identities.path()).expect("verify absent slot"),
        AttestationStatus::Absent
    );
}
