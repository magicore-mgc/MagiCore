//! Legacy token and registry lookups / Tra token và registry theo API tương thích cũ.

use crate::npmrc::NpmRc;

impl NpmRc {
    /// Find a token by a normalized registry host.
    /// Token cho một registry host (vd: registry.npmjs.org).
    pub fn token_for(&self, host: &str) -> Option<&String> {
        self.auth_tokens.get(&Self::normalize_host(host))
    }

    /// Normalize host key: //registry.npmjs.org/ → registry.npmjs.org
    /// Chuẩn hóa host key bằng cách bỏ dấu `/` ở hai đầu.
    pub fn normalize_host(host: &str) -> String {
        host.trim_start_matches('/')
            .trim_end_matches('/')
            .to_string()
    }

    /// Resolve the scope-specific registry or fall back to the default.
    /// Registry cho scoped package (@scope → URL), fallback registry mặc định.
    pub fn registry_for(&self, scope: Option<&str>) -> Option<String> {
        scope
            .and_then(|s| self.scope_registries.get(s))
            .cloned()
            .or_else(|| self.registry.clone())
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
