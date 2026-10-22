#![cfg(test)]
#![allow(clippy::unwrap_used)]

//! Recognize the existing metadata and file identity markers.
//! Nhận diện metadata và file marker identity hiện có.

use crate::engine::manifest_is_game;
use tempfile::tempdir;

#[test]
fn mgc_game_identity_and_game_table_are_accepted() {
    let project = tempdir().unwrap();
    std::fs::write(
        project.path().join("mgc.toml"),
        "ecosystem = 'game'
",
    )
    .unwrap();
    assert!(manifest_is_game(project.path()));
    std::fs::write(
        project.path().join("mgc.toml"),
        "[game]
engine = 'bevy'
",
    )
    .unwrap();
    assert!(manifest_is_game(project.path()));
}

#[test]
fn supported_engine_markers_are_accepted() {
    let project = tempdir().unwrap();
    assert!(!manifest_is_game(project.path()));
    std::fs::write(project.path().join("project.godot"), "").unwrap();
    assert!(manifest_is_game(project.path()));
    std::fs::remove_file(project.path().join("project.godot")).unwrap();
    std::fs::create_dir(project.path().join("Packages")).unwrap();
    std::fs::write(project.path().join("Packages/manifest.json"), "{}").unwrap();
    assert!(manifest_is_game(project.path()));
    std::fs::remove_dir_all(project.path().join("Packages")).unwrap();
    std::fs::write(
        project.path().join("Cargo.toml"),
        "[package]
",
    )
    .unwrap();
    assert!(manifest_is_game(project.path()));
    std::fs::remove_file(project.path().join("Cargo.toml")).unwrap();
    std::fs::write(project.path().join("Game.uproject"), "{}").unwrap();
    assert!(manifest_is_game(project.path()));
}
