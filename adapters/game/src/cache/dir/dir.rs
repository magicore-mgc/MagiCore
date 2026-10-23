//! Engine cache directory resolution.
//! Resolve thư mục cache engine.

use crate::engine::GameEngine;
use mgc_types::MgResult;
use std::path::PathBuf;

/// Get cache directory for game engines
pub fn cache_dir(engine: GameEngine) -> MgResult<PathBuf> {
    let cache_path = mgc_platform::paths::mgc_data_dir()
        .join("cache")
        .join("game")
        .join(engine.as_str());

    Ok(cache_path)
}
