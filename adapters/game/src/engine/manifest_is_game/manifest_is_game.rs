//! Detect game project identity from MGC metadata and engine markers.
//! Nhận diện project game từ metadata MGC và marker engine.

use std::path::Path;

pub(crate) fn manifest_is_game(root: &Path) -> bool {
    // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
    if let Ok(Some(content)) =
        mgc_config::project::read_regular_project_text(&root.join("mgc.toml"), "project config")
        && let Ok(value) = toml::from_str::<toml::Value>(&content)
    {
        if value.get("ecosystem").and_then(|entry| entry.as_str()) == Some("game") {
            return true;
        }
        if value.get("game").is_some() {
            return true;
        }
    }
    root.join("project.godot").exists()
        || root.join("Packages").join("manifest.json").exists()
        || root.join("Cargo.toml").exists()
        || root.read_dir().ok().is_some_and(|directory| {
            directory.filter_map(|entry| entry.ok()).any(|entry| {
                entry
                    .path()
                    .extension()
                    .is_some_and(|extension| extension == "uproject")
            })
        })
}
