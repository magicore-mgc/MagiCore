//! Coordinates explicit-language selection and ordered project markers.
//! Điều phối lựa chọn language tường minh và thứ tự marker của project.

use std::path::Path;

use super::{AppLanguage, explicit_language, framework_markers};

/// Detects the App language without changing the existing precedence.
/// Nhận diện language App và giữ nguyên thứ tự ưu tiên hiện tại.
pub fn detect_language(root: &Path) -> Option<AppLanguage> {
    if let Some(language) = explicit_language::read(root) {
        return match language.as_str() {
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
    framework_markers::detect(root)
}
