//! Manifest Parsing behavior tests.
//! Test hành vi cho nhóm manifest parsing.

use crate::integration::support::tmp;
use mgc_game_adapter::adapter_for;
use mgc_types::adapter::PackageAdapter;

pub async fn parse_manifest_godot_returns_game_manifest() {
    let dir = tmp("my-godot-game");
    std::fs::write(dir.join("project.godot"), "[application]\n").unwrap();
    let a = adapter_for(&dir).unwrap();
    let manifest = a.parse_manifest(&dir).await.unwrap();
    assert!(manifest.name.contains("my-godot-game"));
}

pub async fn parse_manifest_bevy_reads_cargo_toml_dependencies() {
    let dir = tmp("my-bevy-game");
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"space-game\"\nversion = \"0.1.0\"\n\n[dependencies]\nbevy = \"0.14\"\n",
    )
    .unwrap();
    let a = adapter_for(&dir).unwrap();
    let manifest = a.parse_manifest(&dir).await.unwrap();
    assert_eq!(manifest.name, "space-game");
    assert_eq!(manifest.dependencies.len(), 1);
    assert_eq!(manifest.dependencies[0].name.as_str(), "bevy");
}
