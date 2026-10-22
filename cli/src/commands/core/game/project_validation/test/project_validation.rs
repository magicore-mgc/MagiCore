//! Validate that a non-Bevy project is rejected before setup.
//! Xác nhận project không phải Bevy bị từ chối trước setup.
use super::super::validate_game_optimizer_project;

#[test]
fn validation_rejects_non_bevy_project() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::write(root.join("project.godot"), "config_version=5\n").unwrap();
    std::fs::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"godot-helper\"\nversion = \"0.1.0\"\n\n[dependencies]\n",
    )
    .unwrap();

    let error = validate_game_optimizer_project(root).unwrap_err();

    assert!(error.to_string().contains("Bevy"), "{error:#}");
}
