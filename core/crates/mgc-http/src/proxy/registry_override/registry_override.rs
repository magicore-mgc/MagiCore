// Registry override owner — add a proxy for one registry URL.
// Owner ghi đè registry — đặt proxy riêng cho một URL registry.

use crate::proxy::ProxyConfig;

impl ProxyConfig {
    /// Add an override for a specific registry URL.
    /// Thêm proxy ghi đè cho một URL registry cụ thể.
    pub fn with_registry_override(
        mut self,
        registry_url: impl Into<String>,
        proxy_url: impl Into<String>,
    ) -> Self {
        self.per_registry
            .insert(registry_url.into(), proxy_url.into());
        self
    }
}

#[cfg(test)]
#[path = "test/registry_override.rs"]
mod tests;
