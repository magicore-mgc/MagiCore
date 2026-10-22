//! Registry lifecycle.
// Các phương thức vòng đời registry.

use crate::native::npm_registry::NpmRegistry;

impl NpmRegistry {
    pub fn new(registry_url: &str) -> Self {
        Self {
            registry_url: registry_url.to_string(),
            token: None,
            fallbacks: Vec::new(),
            scoped_clients: None,
            age_armed: std::sync::atomic::AtomicBool::new(false),
        }
    }

    pub fn new_with_token(registry_url: &str, token: Option<String>) -> Self {
        Self {
            registry_url: registry_url.to_string(),
            token,
            fallbacks: Vec::new(),
            scoped_clients: None,
            age_armed: std::sync::atomic::AtomicBool::new(false),
        }
    }

    /// Primary + fallback chain. Fallback entries: (url, token).
    pub fn new_with_chain(
        registry_url: &str,
        token: Option<String>,
        fallbacks: Vec<(String, Option<String>)>,
    ) -> Self {
        Self {
            registry_url: registry_url.to_string(),
            token,
            fallbacks,
            scoped_clients: None,
            age_armed: std::sync::atomic::AtomicBool::new(false),
        }
    }
}
