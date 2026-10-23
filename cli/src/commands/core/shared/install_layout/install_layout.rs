//! Select the Web install layout from the explicit environment policy.
//! Chọn layout install Web theo policy môi trường tường minh.

/// Read the Web layout override while preserving its established defaults.
/// Đọc override layout Web và giữ nguyên giá trị mặc định hiện tại.
pub fn should_use_legacy_flat_layout(core_type: &str) -> bool {
    if core_type != "web" {
        return false;
    }
    let env_value = std::env::var("MAGICORE_WEB_STRICT_LAYOUT").ok();
    legacy_flat_layout_for(core_type, env_value.as_deref())
}

pub(super) fn legacy_flat_layout_for(core_type: &str, env_value: Option<&str>) -> bool {
    if core_type != "web" {
        return false;
    }
    let normalized = env_value.map(|value| value.trim().to_ascii_lowercase());
    match normalized.as_deref() {
        Some("1" | "true" | "yes" | "on") => false,
        Some("0" | "false" | "no" | "off" | "legacy" | "flat") => true,
        Some(_) | None => false,
    }
}
