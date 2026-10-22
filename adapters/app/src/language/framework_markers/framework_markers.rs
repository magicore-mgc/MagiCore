//! Detects frameworks in the established marker precedence.
//! Nhận diện framework theo thứ tự marker đã được xác lập.

use std::path::Path;

use super::super::{AppLanguage, is_regular_manifest, maui_project_manifest};

pub(in crate::language) fn detect(root: &Path) -> Option<AppLanguage> {
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
    if let Ok(Some(content)) = mgc_config::project::read_regular_project_text(
        &root.join("package.json"),
        "package manifest",
    ) && content.contains(r#""react-native""#)
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
