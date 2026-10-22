//! App manifest guidance for unsupported CLI lanes.
//! Hướng dẫn manifest App cho lane chưa có CLI.

/// Describe the manifest for an App dependency verb without claiming support.
/// Nêu manifest của verb dependency App nhưng không nhận là đã hỗ trợ.
pub fn manifest_hint(lang: mgc_app_adapter::AppLanguage, verb: &str) -> anyhow::Error {
    let file = match lang {
        mgc_app_adapter::AppLanguage::Flutter => "pubspec.yaml",
        mgc_app_adapter::AppLanguage::Kotlin => "android/app/build.gradle(.kts)",
        mgc_app_adapter::AppLanguage::Swift => "Package.swift",
        mgc_app_adapter::AppLanguage::ReactNative => {
            "package.json (MagiCore-native runner pending)"
        }
        mgc_app_adapter::AppLanguage::ObjC => "iOS Podfile",
        mgc_app_adapter::AppLanguage::Multi => "platform subproject manifest",
        mgc_app_adapter::AppLanguage::Maui => "MAUI *.csproj",
        mgc_app_adapter::AppLanguage::Tauri => "src-tauri/Cargo.toml",
    };
    anyhow::Error::new(super::super::error::AppCoreError::manifest_hint(
        verb,
        &format!("{lang:?}"),
        file,
    ))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
