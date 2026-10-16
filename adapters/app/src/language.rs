//! App language detection for mgc-app-adapter.
//! Nhận diện core app qua mgc.toml và marker file nền tảng.

use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppLanguage {
    Flutter,
    Kotlin,
    Swift,
    ReactNative,
    ObjC,
    Multi,
    Maui,
    Tauri,
}

impl AppLanguage {
    pub fn as_str(&self) -> &'static str {
        match self {
            AppLanguage::Flutter => "flutter",
            AppLanguage::Kotlin => "kotlin",
            AppLanguage::Swift => "swift",
            AppLanguage::ReactNative => "react-native",
            AppLanguage::ObjC => "objc",
            AppLanguage::Multi => "multi",
            AppLanguage::Maui => "maui",
            AppLanguage::Tauri => "tauri",
        }
    }

    /// Canonical C0 firewall ecosystem id — the ONLY string
    /// `dep_gate::owner_for` matches on. NOTE: ReactNative maps to "rn",
    /// NOT the display name "react-native": passing `as_str()` to the
    /// gate would silently miss the Unsupported rule (P0 bypass fix).
    /// (Id ecosystem chuẩn cho tường lửa C0 — ReactNative map sang "rn".)
    pub fn ecosystem(&self) -> &'static str {
        match self {
            AppLanguage::Flutter => "flutter",
            AppLanguage::Kotlin => "kotlin",
            AppLanguage::Swift => "swift",
            AppLanguage::ReactNative => "rn",
            AppLanguage::ObjC => "objc",
            AppLanguage::Multi => "multi",
            AppLanguage::Maui => "dotnet",
            AppLanguage::Tauri => "rust",
        }
    }
}

pub fn detect_language(root: &Path) -> Option<AppLanguage> {
    // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
    if let Ok(Some(content)) =
        mgc_config::project::read_regular_project_text(&root.join("mgc.toml"), "project config")
        && let Ok(v) = toml::from_str::<toml::Value>(&content)
        && let Some(p) = v
            .get("app")
            .and_then(|c| c.get("language"))
            .and_then(|p| p.as_str())
    {
        return match p {
            "flutter" => Some(AppLanguage::Flutter),
            "kotlin" => Some(AppLanguage::Kotlin),
            "swift" => Some(AppLanguage::Swift),
            "react-native" => Some(AppLanguage::ReactNative),
            "objc" => Some(AppLanguage::ObjC),
            "multi" => Some(AppLanguage::Multi),
            "maui" => Some(AppLanguage::Maui),
            "tauri" => Some(AppLanguage::Tauri),
            _ => None,
        };
    }
    if is_regular_manifest(&root.join("pubspec.yaml")) {
        return Some(AppLanguage::Flutter);
    }
    if is_regular_manifest(&root.join("build.gradle.kts"))
        || is_regular_manifest(&root.join("build.gradle"))
    {
        return Some(AppLanguage::Kotlin);
    }
    if is_regular_manifest(&root.join("Package.swift")) {
        return Some(AppLanguage::Swift);
    }
    if is_regular_manifest(&root.join("src-tauri").join("Cargo.toml"))
        && is_regular_manifest(&root.join("src-tauri").join("tauri.conf.json"))
    {
        return Some(AppLanguage::Tauri);
    }
    if maui_project_manifest(root).is_some() {
        return Some(AppLanguage::Maui);
    }
    // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
    if let Ok(Some(content)) = mgc_config::project::read_regular_project_text(
        &root.join("package.json"),
        "package manifest",
    ) && content.contains("\"react-native\"")
    {
        return Some(AppLanguage::ReactNative);
    }
    if is_regular_manifest(&root.join("ObjcBridge.h"))
        && is_regular_manifest(&root.join("ObjcBridge.m"))
    {
        return Some(AppLanguage::ObjC);
    }
    None
}

/// Detect a .NET project only when its SDK manifest explicitly enables MAUI.
/// Chỉ nhận project .NET khi manifest khai báo rõ MAUI.
pub(crate) fn maui_project_manifest(root: &Path) -> Option<PathBuf> {
    let entries = std::fs::read_dir(root).ok()?;
    let mut projects = entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "csproj")
                && is_regular_manifest(path)
                && mgc_config::project::read_regular_project_text(path, "MAUI project")
                    .ok()
                    .flatten()
                    .is_some_and(|contents| contents.contains("<UseMaui>true</UseMaui>"))
        })
        .collect::<Vec<_>>();
    projects.sort();
    (projects.len() == 1).then(|| projects.remove(0))
}

/// Ignore symlinked and special-file framework markers during core selection.
/// Bỏ qua marker framework là symlink hoặc file đặc biệt khi chọn core.
pub(crate) fn is_regular_manifest(path: &Path) -> bool {
    std::fs::symlink_metadata(path).is_ok_and(|metadata| metadata.file_type().is_file())
}

pub(crate) fn manifest_is_app(root: &Path) -> bool {
    // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
    if let Ok(Some(content)) =
        mgc_config::project::read_regular_project_text(&root.join("mgc.toml"), "project config")
        && let Ok(v) = toml::from_str::<toml::Value>(&content)
    {
        if v.get("ecosystem").and_then(|e| e.as_str()) == Some("app") {
            return true;
        }
        if v.get("app").is_some() {
            return true;
        }
    }
    detect_language(root).is_some()
}
