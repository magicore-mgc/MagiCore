//! Persist and validate resolved dependency graphs in the shared cache.
//! Lưu và xác thực dependency graph đã resolve trong cache dùng chung.

use crate::cache::{CachedResolutionEnvelope, SharedWebCache};
use crate::manifest::atomic_write;
use mgc_types::{MgResult, adapter::ResolvedGraph};
use std::path::PathBuf;

const RESOLUTION_CACHE_TTL_SECS: u64 = 300;

impl SharedWebCache {
    pub fn resolution_path(&self, key: &str) -> PathBuf {
        self.root.join("resolutions").join(format!("{key}.json"))
    }

    pub fn read_resolution(
        &self,
        key: &str,
        registry_url: &str,
    ) -> MgResult<Option<ResolvedGraph>> {
        let path = self.resolution_path(key);
        if !path.exists() {
            return Ok(None);
        }

        let contents = match std::fs::read_to_string(&path) {
            Ok(contents) => contents,
            Err(_) => return Ok(None),
        };
        let envelope = match serde_json::from_str::<CachedResolutionEnvelope>(&contents) {
            Ok(envelope) => envelope,
            Err(_) => {
                let _ = std::fs::remove_file(&path);
                return Ok(None);
            }
        };
        if envelope.cache_version != 1 || envelope.registry_url != registry_url {
            return Ok(None);
        }
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        if now.saturating_sub(envelope.fetched_at) > RESOLUTION_CACHE_TTL_SECS {
            let _ = std::fs::remove_file(&path);
            return Ok(None);
        }
        Ok(Some(envelope.graph))
    }

    pub fn write_resolution(
        &self,
        key: &str,
        registry_url: &str,
        graph: &ResolvedGraph,
    ) -> MgResult<()> {
        let path = self.resolution_path(key);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let payload = serde_json::to_vec(&CachedResolutionEnvelope {
            cache_version: 1,
            registry_url: registry_url.to_string(),
            fetched_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
            graph: graph.clone(),
        })?;
        atomic_write(&path, &payload)
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
