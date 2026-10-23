//! MagiCore-owned cache directory per library ecosystem.
//! Cache do MagiCore sở hữu theo ecosystem thư viện.

use mgc_types::{MgError, MgResult};
use std::path::PathBuf;

/// Get the MagiCore-owned cache directory for a library ecosystem.
/// Lấy cache do MagiCore sở hữu cho ecosystem thư viện.
pub fn cache_dir(language: &str) -> MgResult<PathBuf> {
    let globals = mgc_platform::paths::GlobalPaths::new()
        .map_err(|e| MgError::Other(format!("cannot resolve MagiCore paths: {e}")))?;

    match language {
        "rust" => Ok(globals.store.join("cargo")),
        "python" => Ok(globals.store.join("pypi")),
        "ts" | "typescript" => {
            // TypeScript cache is isolated under the MagiCore global cache.
            // Cache TypeScript được cách ly trong cache global của MagiCore.
            Ok(globals.cache.join("web"))
        }
        _ => Err(MgError::Other(format!(
            "unsupported language: {}",
            language
        ))),
    }
}
