//! Tests for AI root validation and persisted core identity.
//! Test xác thực root AI và core identity đã lưu.

use super::super::ai_project_root_at;

#[test]
fn ai_root_requires_matching_adapter_and_saved_core_identity() {
    let project = tempfile::tempdir().expect("tempdir");
    mgc_config::project::ProjectConfig::new("ai-owner-test", "ai")
        .save(project.path())
        .expect("save AI identity");
    std::fs::write(
        project.path().join("pyproject.toml"),
        "[tool.magicore]\nframework = \"python-agent\"\n",
    )
    .expect("write AI framework marker");

    assert_eq!(ai_project_root_at(project.path()).unwrap(), project.path());
}
