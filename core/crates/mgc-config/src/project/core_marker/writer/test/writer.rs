//! Core marker claim and write-guard tests.
//! Test claim marker và guard ghi core.
use super::super::ensure_core_marker_at;
use crate::project::project::core_marker::error::CoreMarkerError;

#[test]
fn unknown_core_is_rejected_before_creating_the_project_directory() {
    let parent = tempfile::tempdir().expect("temporary parent should be created");
    let project = parent.path().join("new-project");

    let error =
        ensure_core_marker_at(&project, "monolith").expect_err("unknown core should be rejected");

    assert!(matches!(error, CoreMarkerError::UnknownCore { .. }));
    assert!(!project.exists());
}

#[test]
fn configured_core_reassignment_is_rejected_before_marker_creation() {
    let project = tempfile::tempdir().expect("temporary project should be created");
    std::fs::write(
        project.path().join("mgc.toml"),
        "name = \"marker-test\"\necosystem = \"web\"\n",
    )
    .expect("project config should be written");

    let error = ensure_core_marker_at(project.path(), "ai")
        .expect_err("stored core reassignment should be rejected");

    assert!(matches!(
        error,
        CoreMarkerError::ConfiguredCoreReassignment { .. }
    ));
    assert!(!project.path().join(".mgc.core").exists());
}
