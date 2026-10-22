//! App project-helper tests.
//! Test helper project App.

use super::super::manifest_hint;

#[test]
fn manifest_hints_preserve_flutter_and_name_new_app_manifests() {
    let flutter = manifest_hint(mgc_app_adapter::AppLanguage::Flutter, "install");
    assert!(flutter.to_string().contains("pubspec.yaml"));

    let maui = manifest_hint(mgc_app_adapter::AppLanguage::Maui, "install");
    assert!(maui.to_string().contains("MAUI *.csproj"));

    let tauri = manifest_hint(mgc_app_adapter::AppLanguage::Tauri, "install");
    assert!(tauri.to_string().contains("src-tauri/Cargo.toml"));
}
