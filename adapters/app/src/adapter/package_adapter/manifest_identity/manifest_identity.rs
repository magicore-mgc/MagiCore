//! Resolve the stable identity of an App language manifest.
//! Xác định identity ổn định cho manifest ngôn ngữ App.

use crate::adapter::AppAdapter;
use crate::language::AppLanguage;

pub(in crate::adapter::package_adapter) fn manifest_identity(
    adapter: &AppAdapter,
) -> Option<mgc_types::ManifestIdentity> {
    // Source-verified manifest per language (manifest/ module).
    // Xác định theo manifest mà module ngôn ngữ thực sự đọc/ghi.
    let (language, format, relpath) = match adapter.language {
        AppLanguage::Flutter => ("flutter", "pubspec.yaml", "pubspec.yaml"),
        AppLanguage::Swift => ("swift", "Package.swift", "Package.swift"),
        AppLanguage::Kotlin => ("kotlin", "build.gradle", "build.gradle"),
        AppLanguage::ReactNative => ("rn", "package.json", "package.json"),
        AppLanguage::ObjC => ("objc", "Podfile", "Podfile"),
        AppLanguage::Multi => ("multi", "multi", "multi"),
        AppLanguage::Maui => {
            let path = crate::language::maui_project_manifest(&adapter.project_root)?;
            let relpath = path
                .strip_prefix(&adapter.project_root)
                .ok()?
                .to_str()?
                .to_string();
            return Some(mgc_types::ManifestIdentity {
                core: "app".to_string(),
                language: "maui".to_string(),
                format: "csproj".to_string(),
                relpath,
            });
        }
        AppLanguage::Tauri => {
            if !crate::language::is_regular_manifest(
                &adapter.project_root.join("src-tauri/Cargo.toml"),
            ) {
                return None;
            }
            ("tauri", "Cargo.toml", "src-tauri/Cargo.toml")
        }
    };
    Some(mgc_types::ManifestIdentity {
        core: "app".to_string(),
        language: language.to_string(),
        format: format.to_string(),
        relpath: relpath.to_string(),
    })
}
