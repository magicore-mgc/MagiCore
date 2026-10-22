//! Render optional install timing diagnostics from the existing environment policy.
//! In diagnostic thời gian install tùy chọn theo policy môi trường hiện có.

pub(crate) fn profile_install_mark(label: &str, started_at: std::time::Instant) {
    if profile_enabled(
        std::env::var("MAGICORE_WEB_PROFILE_INSTALL")
            .ok()
            .as_deref(),
    ) {
        eprintln!(
            "[magicore:web:command-profile] {}={}ms",
            label,
            started_at.elapsed().as_millis()
        );
    }
}

pub(super) fn profile_enabled(value: Option<&str>) -> bool {
    value
        .map(str::trim)
        .map(str::to_ascii_lowercase)
        .is_some_and(|value| matches!(value.as_str(), "1" | "true" | "yes" | "on"))
}
