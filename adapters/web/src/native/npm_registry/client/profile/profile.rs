//! client profile.
// Chứa đúng trách nhiệm của module; giữ nguyên hành vi, API, chính sách lỗi và bảo mật hiện có.

pub(in crate::native::npm_registry) fn network_profile_enabled() -> bool {
    std::env::var("MAGICORE_WEB_PROFILE_NETWORK")
        .ok()
        .map(|value| value.trim().to_ascii_lowercase())
        .map(|value| matches!(value.as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false)
}

pub(in crate::native::npm_registry) fn network_profile_log(
    kind: &str,
    target: &str,
    message: &str,
) {
    if network_profile_enabled() {
        eprintln!(
            "[magicore:web:network-profile] kind={} target={} {}",
            kind, target, message
        );
    }
}
