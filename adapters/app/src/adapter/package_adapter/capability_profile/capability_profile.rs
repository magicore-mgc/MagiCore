//! Select truthful capability claims for each App language.
//! Chọn claim capability đúng theo từng ngôn ngữ App.

use crate::adapter::AppAdapter;
use crate::language::AppLanguage;
use mgc_types::capabilities::Capability;

pub(in crate::adapter::package_adapter) fn select_capabilities(
    adapter: &AppAdapter,
) -> &'static [Capability] {
    // Per-language truth: only lanes with a native resolver claim it.
    // Claim theo ngôn ngữ: chỉ lane có resolver native mới nhận capability.
    match adapter.language {
        AppLanguage::Flutter | AppLanguage::Swift => AppAdapter::CAPABILITIES_RESOLVER,
        AppLanguage::ReactNative => AppAdapter::CAPABILITIES_BASE,
        AppLanguage::Kotlin
        | AppLanguage::ObjC
        | AppLanguage::Multi
        | AppLanguage::Maui
        | AppLanguage::Tauri => AppAdapter::CAPABILITIES_BASE,
    }
}
