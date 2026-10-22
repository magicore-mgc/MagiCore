//! Tests for source snapshot preflight — kiểm tra preflight snapshot nguồn.

use super::super::inspect_project_sources;

#[test]
fn empty_project_store_is_rejected_after_locking() {
    let project = tempfile::tempdir().unwrap();

    let error = inspect_project_sources(project.path()).err().unwrap();

    assert_eq!(
        error.to_string(),
        "project has no Web database or virtual-store data to back up"
    );
}
