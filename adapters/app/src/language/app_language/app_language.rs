//! Stable App language variants and ecosystem identifiers.
//! Variant language App và ecosystem ID ổn định.

/// Language identity used by the App adapter.
/// Định danh language được App adapter sử dụng.
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
    /// Returns the stable display/configuration value.
    /// Trả về giá trị hiển thị/cấu hình ổn định.
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

    /// Returns the canonical C0 firewall ecosystem ID consumed by `dep_gate::owner_for`.
    /// Trả ID ecosystem C0 chuẩn mà `dep_gate::owner_for` dùng để phân loại.
    /// React Native intentionally maps to `rn`, not its display name.
    /// React Native chủ ý map sang `rn`, không dùng tên hiển thị.
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
