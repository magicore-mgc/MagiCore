//! Lib adapter ownership tests.
//! Test quyền sở hữu adapter Lib.

use super::super::adapter;

#[test]
fn lib_adapter_rejects_a_project_owned_by_another_core() {
    let project = tempfile::tempdir().expect("tempdir");
    mgc_config::project::ProjectConfig::new("ai-rust-project", "ai")
        .save(project.path())
        .expect("save AI identity");
    std::fs::write(
        project.path().join("Cargo.toml"),
        "[package]\nname = 'ai-rust-project'\nversion = '0.1.0'\n",
    )
    .expect("write adjacent Rust manifest");

    let error = adapter(project.path())
        .err()
        .expect("foreign core must be rejected");
    assert!(error.to_string().contains("belongs to core 'ai'"));
}
