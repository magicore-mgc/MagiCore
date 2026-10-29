//! Target data passed from manifest selection to serial resolution.
//! Dữ liệu target chuyển từ chọn manifest sang resolve tuần tự.

/// Preserve the dependency name, current pin and native-manager flags.
/// Giữ tên dependency, pin hiện tại và các cờ native manager.
pub(in crate::commands::core::shared) struct UpdateTarget {
    pub(in crate::commands::core::shared::implementation::implementation::package_update) name:
        String,
    pub(in crate::commands::core::shared::implementation::implementation::package_update) current_version:
        String,
    pub(in crate::commands::core::shared::implementation::implementation::package_update) dev: bool,
    pub(in crate::commands::core::shared::implementation::implementation::package_update) optional:
        bool,
    pub(in crate::commands::core::shared::implementation::implementation::package_update) peer:
        bool,
}
