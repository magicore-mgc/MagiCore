//! Engine Config behavior tests.
//! Test hành vi cho nhóm engine config.

use crate::integration::support::tmp;
use mgc_game_adapter::{GameEngine, detect_engine};

pub fn detect_via_mgc_toml_overrides_file_marker() {
    let dir = tmp("mgc-override");
    // Cargo.toml marks Bevy; mgc.toml takes precedence and selects Godot.
    // Cargo.toml -> Bevy, mgc.toml -> Godot
    std::fs::write(dir.join("Cargo.toml"), "[package]\nname=\"x\"\n").unwrap();
    std::fs::write(
        dir.join("mgc.toml"),
        "ecosystem = \"game\"\n\n[game]\nengine = \"godot\"\n",
    )
    .unwrap();
    assert_eq!(detect_engine(&dir), Some(GameEngine::Godot));
}

pub fn detect_returns_none_for_empty_dir() {
    let dir = tmp("empty");
    assert!(detect_engine(&dir).is_none());
}
