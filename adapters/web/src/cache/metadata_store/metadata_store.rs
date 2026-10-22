//! Read and persist registry package metadata in the shared cache.
//! Đọc và lưu metadata package registry trong cache dùng chung.

use crate::cache::SharedWebCache;
use crate::cache::current_unix_secs;
use crate::cache_metadata::{CachedMetadataEnvelope, CachedMetadataRecord};
use crate::manifest::atomic_write;
use crate::native;
use mgc_resolver::DependencyError;
use std::path::PathBuf;

impl SharedWebCache {
    pub fn metadata_path(&self, package: &str, registry_url: &str) -> PathBuf {
        let reg_key: String = registry_url
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '_' })
            .collect();
        self.root
            .join("metadata")
            .join("v2")
            .join(reg_key)
            .join(package)
            .join("metadata.json")
    }

    pub fn read_metadata(
        &self,
        package: &str,
        registry_url: &str,
    ) -> Result<Option<CachedMetadataRecord>, DependencyError> {
        let path = self.metadata_path(package, registry_url);
        if !path.exists() {
            return Ok(None);
        }

        let contents = std::fs::read_to_string(&path).map_err(|e| {
            DependencyError(format!(
                "failed to read cached metadata for '{}': {}",
                package, e
            ))
        })?;
        if let Ok(envelope) = serde_json::from_str::<CachedMetadataEnvelope>(&contents) {
            return Ok(Some(CachedMetadataRecord {
                fetched_at: envelope.fetched_at,
                etag: envelope.etag.clone(),
                stale_retry_after: envelope.stale_retry_after,
                metadata: envelope.metadata,
            }));
        }

        let metadata = serde_json::from_str(&contents).map_err(|e| {
            DependencyError(format!(
                "failed to parse cached metadata for '{}': {}",
                package, e
            ))
        })?;
        Ok(Some(CachedMetadataRecord {
            fetched_at: 0,
            etag: None,
            stale_retry_after: None,
            metadata,
        }))
    }

    pub fn write_metadata(
        &self,
        package: &str,
        metadata: &native::npm_registry::PackageMetadata,
        etag: Option<String>,
        registry_url: &str,
    ) -> Result<(), DependencyError> {
        self.write_metadata_record(
            package,
            metadata,
            etag,
            current_unix_secs(),
            None,
            registry_url,
        )
    }

    pub fn write_metadata_record(
        &self,
        package: &str,
        metadata: &native::npm_registry::PackageMetadata,
        etag: Option<String>,
        fetched_at: u64,
        stale_retry_after: Option<u64>,
        registry_url: &str,
    ) -> Result<(), DependencyError> {
        let path = self.metadata_path(package, registry_url);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                DependencyError(format!(
                    "failed to create metadata cache dir for '{}': {}",
                    package, e
                ))
            })?;
        }
        let payload = serde_json::to_vec(&CachedMetadataEnvelope {
            fetched_at,
            etag,
            stale_retry_after,
            metadata: metadata.clone(),
        })
        .map_err(|e| {
            DependencyError(format!(
                "failed to serialize cached metadata for '{}': {}",
                package, e
            ))
        })?;
        atomic_write(&path, &payload).map_err(|e| {
            DependencyError(format!(
                "failed to write cached metadata for '{}': {}",
                package, e
            ))
        })?;
        Ok(())
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
