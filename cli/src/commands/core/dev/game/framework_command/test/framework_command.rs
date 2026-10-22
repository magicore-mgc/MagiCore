//! Tests for Game framework command mapping.

use std::path::Path;

use super::super::framework_command::game_dev_command;

#[test]
fn game_dev_bevy_runs_cargo() {
    let dir = std::env::temp_dir().join(format!("mgc-dev-bevy-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("Cargo.toml"),
        concat!(
            "[package]\n",
            "name = \"demo\"\n",
            "version = \"0.1.0\"\n\n",
            "[package.metadata.magicore]\n",
            "core = \"game\"\n\n",
            "[dependencies]\n",
        ),
    )
    .unwrap();
    let (cmd, args) = game_dev_command(&dir).unwrap();
    assert_eq!(cmd, "cargo");
    assert_eq!(args, vec!["run", "--locked", "--offline"]);
}

#[test]
fn game_dev_godot_opens_editor() {
    let dir = std::env::temp_dir().join(format!("mgc-dev-godot-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("project.godot"), "[application]\nname=\"demo\"\n").unwrap();
    let (cmd, args) = game_dev_command(&dir).unwrap();
    assert_eq!(cmd, "godot");
    assert!(args.iter().any(|a| a == "--editor"));
    assert!(args.iter().any(|a| a == "--path"));
}

#[test]
fn game_dev_unity_opens_editor_cli() {
    let dir = std::env::temp_dir().join(format!("mgc-dev-unity-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("Packages")).unwrap();
    std::fs::write(dir.join("Packages").join("manifest.json"), "{}").unwrap();
    let (cmd, args) = game_dev_command(&dir).unwrap();
    assert_eq!(cmd, "unity");
    assert!(args.iter().any(|a| a == "-projectPath"));
}

#[test]
fn game_dev_unreal_hints_editor() {
    let dir = std::env::temp_dir().join(format!("mgc-dev-unreal-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("Game.uproject"), "{}").unwrap();
    let err = game_dev_command(&dir).unwrap_err();
    assert!(err.to_string().contains("Game.uproject"));
}

#[test]
fn game_dev_unknown_engine_bails() {
    assert!(game_dev_command(Path::new("/nonexistent")).is_err());
}
