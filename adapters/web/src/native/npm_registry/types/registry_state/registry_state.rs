//! types registry state.
// Chứa đúng trách nhiệm của module; giữ nguyên hành vi, API, chính sách lỗi và bảo mật hiện có.

use crate::native::npm_registry::ScopedRegistryClient;

pub struct NpmRegistry {
    pub(in crate::native::npm_registry) registry_url: String,
    pub(in crate::native::npm_registry) token: Option<String>,
    /// Multi-registry fallback chain (ITEM 1): fetch metadata từ registry kế
    /// tiếp khi primary 404/network/5xx — KHÔNG fallback khi 401/403 (auth
    /// fail = fail-closed, không leak package từ registry khác).
    pub(in crate::native::npm_registry) fallbacks: Vec<(String, Option<String>)>,
    pub(in crate::native::npm_registry) scoped_clients: Option<Vec<(String, ScopedRegistryClient)>>,
    /// Per-client age-gate fetch mode (P0/F6): armed by the owning
    /// provider per operation — full packuments + no stale abbreviated
    /// cache. Never a process global.
    /// (Chế độ fetch theo cổng tuổi — riêng từng client.)
    pub(in crate::native::npm_registry) age_armed: std::sync::atomic::AtomicBool,
}
