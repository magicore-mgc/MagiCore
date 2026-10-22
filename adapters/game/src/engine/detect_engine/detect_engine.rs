//! Detect a supported game engine while preserving marker priority.
//! Nhận diện game engine được hỗ trợ và giữ nguyên thứ tự marker.

use crate::engine::GameEngine;
use std::path::Path;

pub fn detect_engine(root: &Path) -> Option<GameEngine> {
    // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
    if let Ok(Some(content)) =
        mgc_config::project::read_regular_project_text(&root.join("mgc.toml"), "project config")
        && let Ok(value) = toml::from_str::<toml::Value>(&content)
    {
        if value
            .get("ecosystem")
            .and_then(|entry| entry.as_str())
            .is_some_and(|ecosystem| ecosystem != "game")
            && value.get("game").is_none()
        {
            return None;
        }
        if let Some(engine) = value
            .get("game")
            .and_then(|game| game.get("engine"))
            .and_then(|engine| engine.as_str())
        {
            return GameEngine::from_str(engine);
        }
    }
    if root.join("project.godot").exists() {
        return Some(GameEngine::Godot);
    }
    if root.join("Packages").join("manifest.json").exists() {
        return Some(GameEngine::Unity);
    }
    if root
        .read_dir()
        .ok()?
        .filter_map(|entry| entry.ok())
        .any(|entry| {
            entry
                .path()
                .extension()
                .is_some_and(|extension| extension == "uproject")
        })
    {
        return Some(GameEngine::Unreal);
    }
    if root.join("Cargo.toml").exists() {
        return Some(GameEngine::Bevy);
    }
    None
}
