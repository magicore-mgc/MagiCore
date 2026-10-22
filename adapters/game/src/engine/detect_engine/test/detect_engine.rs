#![cfg(test)]
#![allow(clippy::unwrap_used)]

//! Preserve configured selection and filesystem marker priority.
//! Giữ lựa chọn config và thứ tự marker filesystem.

use crate::engine::{GameEngine, detect_engine};
use tempfile::tempdir;

#[test]
fn configured_engine_wins_and_foreign_unconfigured_project_stops() {
    let project = tempdir().unwrap();
    std::fs::write(project.path().join("project.godot"), "").unwrap();
    std::fs::write(
        project.path().join("mgc.toml"),
        "ecosystem = 'game'
[game]
engine = 'unity'
",
    )
    .unwrap();
    assert_eq!(detect_engine(project.path()), Some(GameEngine::Unity));

    std::fs::write(
        project.path().join("mgc.toml"),
        "ecosystem = 'web'
",
    )
    .unwrap();
    assert_eq!(detect_engine(project.path()), None);
}

#[test]
fn marker_priority_is_godot_then_unity_then_unreal_then_bevy() {
    let project = tempdir().unwrap();
    std::fs::write(project.path().join("project.godot"), "").unwrap();
    std::fs::create_dir(project.path().join("Packages")).unwrap();
    std::fs::write(project.path().join("Packages/manifest.json"), "{}").unwrap();
    std::fs::write(project.path().join("Game.uproject"), "{}").unwrap();
    std::fs::write(
        project.path().join("Cargo.toml"),
        "[package]
",
    )
    .unwrap();
    assert_eq!(detect_engine(project.path()), Some(GameEngine::Godot));

    std::fs::remove_file(project.path().join("project.godot")).unwrap();
    assert_eq!(detect_engine(project.path()), Some(GameEngine::Unity));
    std::fs::remove_dir_all(project.path().join("Packages")).unwrap();
    assert_eq!(detect_engine(project.path()), Some(GameEngine::Unreal));
    std::fs::remove_file(project.path().join("Game.uproject")).unwrap();
    assert_eq!(detect_engine(project.path()), Some(GameEngine::Bevy));
}

#[test]
fn configured_unknown_engine_does_not_fall_back_to_markers() {
    let project = tempdir().unwrap();
    std::fs::write(project.path().join("project.godot"), "").unwrap();
    std::fs::write(
        project.path().join("mgc.toml"),
        "[game]
engine = 'unknown'
",
    )
    .unwrap();
    assert_eq!(detect_engine(project.path()), None);
}
