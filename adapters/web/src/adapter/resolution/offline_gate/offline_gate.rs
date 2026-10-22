//! `offline_gate.rs` — Offline environment gate / cổng environment offline.
pub(in crate::adapter::resolution) fn mgc_env_flag(key: &str) -> bool {
    std::env::var(key)
        .ok()
        .map(|value| value.trim().to_ascii_lowercase())
        .is_some_and(|value| matches!(value.as_str(), "1" | "true" | "yes" | "on"))
}
