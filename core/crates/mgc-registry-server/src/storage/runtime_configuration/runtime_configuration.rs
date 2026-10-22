//! Applies upstream and blob-backend configuration to an existing registry store.
//! Áp dụng cấu hình upstream và blob backend cho registry store hiện có.

use super::super::prelude::*;
use super::super::*;

impl RegistryStore {
    /// Cấu hình upstream proxy (ITEM 4). None = registry đóng (private-only).
    pub fn set_upstream(&mut self, upstream: Option<String>) {
        self.upstream = upstream.map(Upstream::new);
    }

    /// Cấu hình blob backend (ITEM 5): "local" hoặc "s3://bucket/prefix".
    pub fn set_backend(&mut self, spec: Option<&str>) -> Result<()> {
        self.backend = BlobBackend::parse(spec, &self.blobs_dir)?;
        Ok(())
    }
}
