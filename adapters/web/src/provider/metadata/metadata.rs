//! `metadata.rs` — metadata.
//! Metadata và cache alias.

use super::super::provider::NpmDependencyProvider;
use crate::cache::load_metadata_with_fallback;
use crate::native;
use mgc_resolver::DependencyError;
use mgc_types::{PackageName, Version};
use std::sync::Arc;

impl NpmDependencyProvider {
    pub async fn metadata(
        &self,
        package: &PackageName,
    ) -> Result<Arc<native::npm_registry::PackageMetadata>, DependencyError> {
        let source_package = self.source_package_name(package);
        let key = source_package.as_str().to_string();
        if let Some(cached) = self.metadata_cache.get(&key) {
            return Ok(cached);
        }
        let lock = self
            .metadata_locks
            .entry(key.clone())
            .or_insert_with(|| Arc::new(tokio::sync::Mutex::new(())))
            .clone();
        let _guard = lock.lock().await;
        if let Some(cached) = self.metadata_cache.get(&key) {
            return Ok(cached);
        }
        let meta = load_metadata_with_fallback(
            &source_package,
            &self.registry,
            self.shared_cache.as_ref(),
        )
        .await?;
        self.metadata_cache.insert(key, Arc::clone(&meta));
        Ok(meta)
    }

    pub fn source_package_name(&self, package: &PackageName) -> PackageName {
        self.alias_targets
            .get(package.as_str())
            .map(|entry| entry.clone())
            .unwrap_or_else(|| package.clone())
    }

    pub fn record_alias_target(&self, alias: &PackageName, target: &PackageName) {
        self.alias_targets
            .insert(alias.as_str().to_string(), target.clone());
    }

    pub fn cached_versions_for(&self, package: &PackageName) -> Option<Vec<Version>> {
        self.registry_cache
            .get_versions(package.as_str())
            .or_else(|| {
                let source = self.source_package_name(package);
                if source == *package {
                    None
                } else {
                    self.registry_cache.get_versions(source.as_str())
                }
            })
    }

    pub fn insert_versions_for(&self, package: &PackageName, versions: Vec<Version>) {
        self.registry_cache
            .insert_versions(package.as_str().to_string(), versions.clone());
        let source = self.source_package_name(package);
        if source != *package {
            self.registry_cache
                .insert_versions(source.as_str().to_string(), versions);
        }
    }
}
