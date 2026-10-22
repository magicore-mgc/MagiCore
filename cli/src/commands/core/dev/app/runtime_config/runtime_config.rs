use std::path::Path;

/// Lấy scheme từ mgc.toml [app] dev_scheme.
pub(in crate::commands::core::dev) fn dev_scheme(root: &Path) -> Option<String> {
    let content =
        mgc_config::project::read_regular_project_text(&root.join("mgc.toml"), "project config")
            .ok()??;
    let v: toml::Value = toml::from_str(&content).ok()?;
    v.get("app")
        .and_then(|a| a.get("dev_scheme"))
        .and_then(|s| s.as_str())
        .map(str::to_string)
        .filter(|s| !s.is_empty())
}

/// Detect app runtime for optimizer env loading
/// Phát hiện runtime app để load env optimizer
pub(in crate::commands::core::dev) fn detect_app_runtime(
    _root: &Path,
    lang: &mgc_app_adapter::AppLanguage,
) -> crate::commands::optimizer::runtime_detect::DetectedRuntime {
    use crate::commands::optimizer::runtime_detect::DetectedRuntime;

    match lang {
        mgc_app_adapter::AppLanguage::Flutter => DetectedRuntime::Flutter,
        mgc_app_adapter::AppLanguage::ReactNative => DetectedRuntime::ReactNative,
        mgc_app_adapter::AppLanguage::Swift | mgc_app_adapter::AppLanguage::ObjC => {
            DetectedRuntime::RustNative // iOS native
        }
        mgc_app_adapter::AppLanguage::Kotlin => DetectedRuntime::RustNative, // Android native
        mgc_app_adapter::AppLanguage::Multi => DetectedRuntime::Flutter, // Multi defaults to Flutter
        mgc_app_adapter::AppLanguage::Maui | mgc_app_adapter::AppLanguage::Tauri => {
            DetectedRuntime::Unknown
        }
    }
}

/// Đọc applicationId từ mgc.toml [app] application_id.
pub(in crate::commands::core::dev) fn read_app_id(root: &Path) -> Option<String> {
    let content =
        mgc_config::project::read_regular_project_text(&root.join("mgc.toml"), "project config")
            .ok()??;
    let v: toml::Value = toml::from_str(&content).ok()?;
    v.get("app")
        .and_then(|a| a.get("application_id"))
        .and_then(|s| s.as_str())
        .map(str::to_string)
        .filter(|s| !s.is_empty())
}

#[cfg(test)]
#[path = "test/runtime_config.rs"]
mod tests;
