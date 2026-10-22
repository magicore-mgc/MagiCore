//! App language detection tests — kiểm thử nhận diện ngôn ngữ App.
//! Owns marker parsing and symlink cases. — Sở hữu parse marker và case symlink.

use mgc_app_adapter::{AppLanguage, adapter_for, detect_language};
use mgc_types::adapter::PackageAdapter;

use super::support::tmp;

#[cfg(unix)]
pub(crate) fn language_detection_does_not_follow_external_mgc_config_symlink() {
    let project = tmp("external-mgc-config-link");
    let external = tmp("external-mgc-config-target");
    std::fs::write(external.join("mgc.toml"), "[app]\nlanguage = 'flutter'\n").unwrap();
    std::os::unix::fs::symlink(external.join("mgc.toml"), project.join("mgc.toml")).unwrap();

    assert_eq!(detect_language(&project), None);
    let _ = std::fs::remove_dir_all(project);
    let _ = std::fs::remove_dir_all(external);
}

#[cfg(unix)]
pub(crate) fn language_detection_does_not_follow_external_package_json_symlink() {
    let project = tmp("external-package-json-link");
    let external = tmp("external-package-json-target");
    std::fs::write(
        external.join("package.json"),
        r#"{"dependencies":{"react-native":"0.74.0"}}"#,
    )
    .unwrap();
    std::os::unix::fs::symlink(external.join("package.json"), project.join("package.json"))
        .unwrap();

    assert_eq!(detect_language(&project), None);
    let _ = std::fs::remove_dir_all(project);
    let _ = std::fs::remove_dir_all(external);
}

pub(crate) fn detect_flutter_via_pubspec_yaml() {
    let dir = tmp("flutter");
    std::fs::write(dir.join("pubspec.yaml"), "name: myapp\nversion: 1.0.0\n").unwrap();
    assert_eq!(detect_language(&dir), Some(AppLanguage::Flutter));
}

pub(crate) fn detect_kotlin_via_build_gradle_kts() {
    let dir = tmp("kotlin-kts");
    std::fs::write(
        dir.join("build.gradle.kts"),
        "plugins { kotlin(\"jvm\") }\n",
    )
    .unwrap();
    assert_eq!(detect_language(&dir), Some(AppLanguage::Kotlin));
}

pub(crate) fn detect_kotlin_via_build_gradle() {
    let dir = tmp("kotlin-groovy");
    std::fs::write(dir.join("build.gradle"), "apply plugin: 'kotlin'\n").unwrap();
    assert_eq!(detect_language(&dir), Some(AppLanguage::Kotlin));
}

pub(crate) fn detect_swift_via_package_swift() {
    let dir = tmp("swift");
    std::fs::write(dir.join("Package.swift"), "// swift-tools-version:5.9\n").unwrap();
    assert_eq!(detect_language(&dir), Some(AppLanguage::Swift));
}

pub(crate) fn detect_maui_via_explicit_use_maui_csproj() {
    let dir = tmp("maui");
    std::fs::write(
        dir.join("Sample.csproj"),
        "<Project Sdk=\"Microsoft.NET.Sdk\"><PropertyGroup><UseMaui>true</UseMaui></PropertyGroup></Project>",
    )
    .unwrap();

    assert_eq!(detect_language(&dir), Some(AppLanguage::Maui));
    let adapter = adapter_for(&dir).unwrap();
    let identity = adapter.manifest_identity().unwrap();
    assert_eq!(identity.language, "maui");
    assert_eq!(identity.format, "csproj");
    assert_eq!(identity.relpath, "Sample.csproj");
    assert!(
        !adapter
            .capabilities()
            .contains(&mgc_types::capabilities::Capability::DependencyResolver)
    );
}

pub(crate) fn maui_detection_refuses_ambiguous_root_projects() {
    let dir = tmp("maui-ambiguous");
    for name in ["First.csproj", "Second.csproj"] {
        std::fs::write(
            dir.join(name),
            "<Project><PropertyGroup><UseMaui>true</UseMaui></PropertyGroup></Project>",
        )
        .unwrap();
    }

    assert_eq!(detect_language(&dir), None);
}

pub(crate) fn detect_tauri_via_rust_manifest_and_config() {
    let dir = tmp("tauri");
    std::fs::create_dir_all(dir.join("src-tauri")).unwrap();
    std::fs::write(
        dir.join("src-tauri/Cargo.toml"),
        "[package]\nname='sample'\n",
    )
    .unwrap();
    std::fs::write(dir.join("src-tauri/tauri.conf.json"), "{}\n").unwrap();

    assert_eq!(detect_language(&dir), Some(AppLanguage::Tauri));
    let adapter = adapter_for(&dir).unwrap();
    let identity = adapter.manifest_identity().unwrap();
    assert_eq!(identity.language, "tauri");
    assert_eq!(identity.format, "Cargo.toml");
    assert_eq!(identity.relpath, "src-tauri/Cargo.toml");
    assert!(
        !adapter
            .capabilities()
            .contains(&mgc_types::capabilities::Capability::DependencyResolver)
    );
}

pub(crate) fn detect_react_native_via_package_json_dep() {
    let dir = tmp("rn");
    std::fs::write(
        dir.join("package.json"),
        r#"{"name":"rn","dependencies":{"react-native":"0.74.0"}}"#,
    )
    .unwrap();
    assert_eq!(detect_language(&dir), Some(AppLanguage::ReactNative));
}

pub(crate) fn react_native_manifest_identity_uses_canonical_gate_id() {
    let dir = tmp("rn-manifest-identity");
    std::fs::write(
        dir.join("package.json"),
        r#"{"name":"rn","dependencies":{"react-native":"0.74.0"}}"#,
    )
    .unwrap();

    let identity = adapter_for(&dir).unwrap().manifest_identity().unwrap();
    assert_eq!(identity.core, "app");
    assert_eq!(identity.language, "rn");
    assert_eq!(identity.format, "package.json");
    assert_eq!(identity.relpath, "package.json");
}

pub(crate) fn detect_objc_via_bridge_header_and_impl_pair() {
    let dir = tmp("objc");
    std::fs::write(dir.join("ObjcBridge.h"), "@interface MGShared\n@end\n").unwrap();
    std::fs::write(dir.join("ObjcBridge.m"), "@implementation MGShared\n@end\n").unwrap();
    assert_eq!(detect_language(&dir), Some(AppLanguage::ObjC));
}

pub(crate) fn detect_via_mgc_toml_language_overrides_marker() {
    let dir = tmp("mgc-override");
    // pubspec.yaml → Flutter, mgc.toml override → Kotlin
    std::fs::write(dir.join("pubspec.yaml"), "name: x\n").unwrap();
    std::fs::write(dir.join("mgc.toml"), "[app]\nlanguage = \"kotlin\"\n").unwrap();
    assert_eq!(detect_language(&dir), Some(AppLanguage::Kotlin));
}

pub(crate) fn detect_multi_via_mgc_toml() {
    let dir = tmp("multi");
    std::fs::write(
        dir.join("mgc.toml"),
        "[app]\nlanguage = \"multi\"\nplatforms = [\"android\",\"ios\"]\n",
    )
    .unwrap();
    assert_eq!(detect_language(&dir), Some(AppLanguage::Multi));
}

pub(crate) fn detect_react_native_via_explicit_mgc_toml_language() {
    let dir = tmp("rn-explicit");
    std::fs::write(dir.join("mgc.toml"), "[app]\nlanguage = \"react-native\"\n").unwrap();

    assert_eq!(detect_language(&dir), Some(AppLanguage::ReactNative));
    assert_eq!(
        adapter_for(&dir).map(|adapter| adapter.language),
        Some(AppLanguage::ReactNative)
    );
}

pub(crate) fn detect_returns_none_for_empty_dir() {
    let dir = tmp("empty");
    assert!(detect_language(&dir).is_none());
}

pub(crate) fn detect_returns_none_for_unknown_mgc_toml_language() {
    let dir = tmp("unknown-lang");
    std::fs::write(dir.join("mgc.toml"), "[app]\nlanguage = \"xamarin\"\n").unwrap();
    // "xamarin" không được hỗ trợ → None
    assert!(detect_language(&dir).is_none());
}

pub(crate) fn package_json_without_react_native_is_not_detected_as_rn() {
    let dir = tmp("non-rn");
    std::fs::write(
        dir.join("package.json"),
        r#"{"name":"web","dependencies":{"react":"^18.0.0"}}"#,
    )
    .unwrap();
    // react thuần web — không phải RN
    assert!(detect_language(&dir).is_none());
}

pub(crate) fn adapter_for_returns_some_with_pubspec_marker() {
    let dir = tmp("af-flutter");
    std::fs::write(dir.join("pubspec.yaml"), "name: a\n").unwrap();
    assert!(adapter_for(&dir).is_some());
}

pub(crate) fn adapter_for_returns_none_without_any_marker() {
    let dir = tmp("af-none");
    assert!(adapter_for(&dir).is_none());
}
