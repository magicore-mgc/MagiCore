//! Recognized core names shared by package mutation routers.
//! Danh sách tên core dùng chung cho router package mutation.

/// Return whether a core identity is known to the shared CLI firewall.
/// Cho biết identity core có nằm trong danh sách firewall CLI dùng chung.
pub(crate) fn is_known_core(name: &str) -> bool {
    matches!(
        name,
        "web" | "game" | "ai" | "clo" | "cicd" | "iot" | "app" | "lib" | "hardware"
    )
}
