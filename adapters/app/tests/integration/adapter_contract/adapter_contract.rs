//! App adapter contract tests — kiểm thử hợp đồng App adapter.
//! Owns language/core identity and project ownership. — Sở hữu định danh ngôn ngữ/core và ownership.

use mgc_app_adapter::{AppAdapter, AppLanguage};
use mgc_types::capabilities::{CoreIdent, ProjectDetector};

use super::support::tmp;

pub(crate) fn applanguage_as_str_values() {
    assert_eq!(AppLanguage::Flutter.as_str(), "flutter");
    assert_eq!(AppLanguage::Kotlin.as_str(), "kotlin");
    assert_eq!(AppLanguage::Swift.as_str(), "swift");
    assert_eq!(AppLanguage::ReactNative.as_str(), "react-native");
    assert_eq!(AppLanguage::ObjC.as_str(), "objc");
    assert_eq!(AppLanguage::Multi.as_str(), "multi");
    assert_eq!(AppLanguage::Maui.as_str(), "maui");
    assert_eq!(AppLanguage::Tauri.as_str(), "tauri");
}

pub(crate) fn adapter_name_and_ecosystem() {
    let a = AppAdapter::new(AppLanguage::Flutter);
    assert_eq!(a.name(), "app");
    assert_eq!(format!("{:?}", a.ecosystem()), "App");
}

pub(crate) fn can_handle_returns_true_for_pubspec_project() {
    let dir = tmp("ch-true");
    std::fs::write(dir.join("pubspec.yaml"), "name: a\n").unwrap();
    let a = AppAdapter::new(AppLanguage::Flutter);
    assert!(a.can_handle(&dir));
}

pub(crate) fn can_handle_returns_false_for_empty_dir() {
    let dir = tmp("ch-false");
    let a = AppAdapter::new(AppLanguage::Flutter);
    assert!(!a.can_handle(&dir));
}
