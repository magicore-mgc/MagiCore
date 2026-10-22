//! `enqueue_policy.rs` — enqueue policy.
//! Quyết định enqueue resolver.

use super::super::provider::NpmDependencyProvider;
use mgc_resolver::{DependencyError, ResolvedDep};

impl NpmDependencyProvider {
    pub(in crate::provider) async fn should_enqueue_impl(
        &self,
        dep: &ResolvedDep,
    ) -> Result<bool, DependencyError> {
        let cache_key = Self::optional_enqueue_key(dep);
        if let Some(cached) = self.optional_enqueue_cache.get(&cache_key) {
            return Ok(*cached);
        }

        // Platform-targeted native packages may appear as required edges; keep only the host build.
        // (Gói native theo nền tảng đôi khi được khai báo bắt buộc; chỉ giữ bản cho host.)
        if let Some(supported) = Self::known_optional_native_binary_supported(&dep.package) {
            self.optional_enqueue_cache.insert(cache_key, supported);
            return Ok(supported);
        }

        if !dep.optional {
            return Ok(true);
        }

        let meta = self.metadata(&dep.package).await?;
        let versions = self.versions_with_age_gate(&dep.package, &meta)?;
        self.insert_versions_for(&dep.package, versions.clone());
        let Some(selected) = Self::select_best_version(&versions, &dep.spec)? else {
            self.optional_enqueue_cache.insert(cache_key, false);
            return Ok(false);
        };
        let Some(info) = meta.versions.get(&selected.to_string()) else {
            self.optional_enqueue_cache.insert(cache_key, false);
            return Ok(false);
        };

        let supported = Self::version_supported(info);
        self.optional_enqueue_cache.insert(cache_key, supported);
        Ok(supported)
    }
}
