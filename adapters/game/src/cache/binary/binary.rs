//! Cached engine binary path and presence/removal operations.
//! Đường dẫn binary engine đã cache cùng kiểm tra/tồn tại và xoá.

use crate::engine::GameEngine;
use mgc_types::{MgError, MgResult};
use std::path::PathBuf;

use super::super::dir::cache_dir;

/// Get cached binary path for specific engine version
pub fn cached_binary_path(engine: GameEngine, version: &str) -> MgResult<PathBuf> {
    let cache = cache_dir(engine)?;

    let binary_name = match engine {
        GameEngine::Godot => format!("godot-{}", version),
        GameEngine::Unity => format!("unity-{}", version),
        GameEngine::Unreal => format!("unreal-{}", version),
        GameEngine::Bevy => return Err(MgError::Other("Bevy uses Cargo - no binary cache".into())),
    };

    Ok(cache.join(binary_name))
}

/// Check if engine binary is cached
pub fn is_cached(engine: GameEngine, version: &str) -> MgResult<bool> {
    if matches!(engine, GameEngine::Bevy) {
        // Bevy = Cargo dep, không cache binary
        return Ok(false);
    }

    let path = cached_binary_path(engine, version)?;
    Ok(path.exists())
}

/// Remove cached binary
pub fn remove_cached_binary(engine: GameEngine, version: &str) -> MgResult<()> {
    let path = cached_binary_path(engine, version)?;

    if path.exists() {
        std::fs::remove_file(&path)?;
    }

    Ok(())
}
