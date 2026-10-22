//! Project core-identity rejection contract.
//! Contract từ chối project có core identity không khớp.

use super::super::ensure_project_core_identity;

#[test]
fn core_command_identity_rejects_a_foreign_project_with_a_matching_manifest() {
    let temp = tempfile::tempdir().expect("tempdir");
    mgc_config::project::ProjectConfig::new("ai-rust-project", "ai")
        .save(temp.path())
        .expect("save AI identity");
    std::fs::write(
        temp.path().join("Cargo.toml"),
        "[package]\nname = 'ai-rust-project'\nversion = '0.1.0'\n",
    )
    .expect("write adjacent Rust manifest");

    assert!(ensure_project_core_identity(temp.path(), "lib").is_err());
    assert!(ensure_project_core_identity(temp.path(), "web").is_err());
    assert!(ensure_project_core_identity(temp.path(), "app").is_err());
    assert!(ensure_project_core_identity(temp.path(), "iot").is_err());
    assert!(ensure_project_core_identity(temp.path(), "ai").is_ok());
}
