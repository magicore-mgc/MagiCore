//! Engine Markers behavior tests.
//! Test hành vi cho nhóm engine markers.

use crate::integration::support::tmp;
use mgc_game_adapter::{GameEngine, detect_engine};

pub fn detect_godot_via_project_godot() {
    let dir = tmp("godot");
    std::fs::write(
        dir.join("project.godot"),
        "[application]\nconfig/name=\"Demo\"\n",
    )
    .unwrap();
    assert_eq!(detect_engine(&dir), Some(GameEngine::Godot));
}

pub fn detect_unity_via_packages_manifest() {
    let dir = tmp("unity");
    std::fs::create_dir_all(dir.join("Packages")).unwrap();
    std::fs::write(
        dir.join("Packages").join("manifest.json"),
        "{\"dependencies\":{}}\n",
    )
    .unwrap();
    assert_eq!(detect_engine(&dir), Some(GameEngine::Unity));
}

pub fn detect_unreal_via_uproject() {
    let dir = tmp("unreal");
    std::fs::write(dir.join("Game.uproject"), "{\"FileVersion\":3}\n").unwrap();
    assert_eq!(detect_engine(&dir), Some(GameEngine::Unreal));
}

pub fn detect_bevy_via_cargo_toml() {
    let dir = tmp("bevy");
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"game\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();
    assert_eq!(detect_engine(&dir), Some(GameEngine::Bevy));
}
