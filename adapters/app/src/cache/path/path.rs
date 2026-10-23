//! MGC-owned cache directory per App language.
//! Thư mục cache do MGC quản lý theo ngôn ngữ App.

use crate::language::AppLanguage;
use mgc_types::{MgError, MgResult};
use std::path::PathBuf;

/// Get app adapter cache directory for language.
pub fn cache_dir(language: AppLanguage) -> MgResult<PathBuf> {
    match language {
        AppLanguage::Flutter => Ok(mgc_platform::paths::mgc_data_dir()
            .join("store")
            .join("pub")),
        AppLanguage::Swift => Ok(mgc_platform::paths::mgc_data_dir()
            .join("store")
            .join("swift")),
        AppLanguage::Multi => Ok(mgc_platform::paths::mgc_data_dir()
            .join("cache")
            .join("app")
            .join("multi")),
        // These ecosystems use shared external package-manager caches. Until
        // MagiCore owns those stores, refusing is safer than offering a clear
        // operation that could delete data used by unrelated projects.
        AppLanguage::Kotlin
        | AppLanguage::ReactNative
        | AppLanguage::ObjC
        | AppLanguage::Maui
        | AppLanguage::Tauri => Err(MgError::Unsupported {
            core: "app",
            capability: "managed-cache",
            guidance: format!(
                "MagiCore does not manage the external {} package cache",
                language.as_str()
            ),
        }),
    }
}
