//! Build verification tests — preserves strict canonical-root matching.
//! Test xác minh build — giữ so khớp canonical root fail-closed.
use super::super::{DedupeBuildVerificationError, same_project_root};
use std::path::PathBuf;

#[test]
fn root_gate_requires_both_canonical_roots_to_match() {
    assert!(same_project_root(
        Some(PathBuf::from("/canonical/project")),
        Some(PathBuf::from("/canonical/project")),
    ));
    assert!(!same_project_root(None, None));
    assert!(!same_project_root(
        Some(PathBuf::from("/canonical/project")),
        None,
    ));
    assert!(!same_project_root(
        Some(PathBuf::from("/canonical/project-a")),
        Some(PathBuf::from("/canonical/project-b")),
    ));
}

#[test]
fn wrong_root_diagnostic_keeps_existing_wording() {
    let error = DedupeBuildVerificationError::WrongProject {
        cwd: "/tmp/other".to_string(),
        project_root: "/tmp/project".to_string(),
    };
    assert_eq!(
        error.to_string(),
        "dedupe verification refused: process directory '/tmp/other' resolves to a different project than '/tmp/project' — run dedupe from the project root"
    );
}
