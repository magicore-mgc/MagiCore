//! Reports OCI blob presence only when its database row and backing file exist.
//! Chỉ báo OCI blob tồn tại khi có cả database row và file nền.

use super::super::error::{OciBlobError, OciBlobResult};
use crate::storage::prelude::*;
use crate::storage::{RegistryStore, validate_fs_segment};

impl RegistryStore {
    /// OCI blob existence with on-disk verification (P0-6): a DB row whose
    /// backing file is gone or unreadable reports false (fail-closed
    /// existence — clients get an honest 404 instead of trusting stale DB
    /// state).
    ///
    /// Kiểm tra tồn tại blob OCI có verify trên đĩa (P0-6): row DB mà file
    /// nền mất/đọc không được thì trả false (existence fail-closed — client
    /// nhận 404 trung thực thay vì tin DB stale).
    pub async fn oci_blob_exists(&self, repo: &str, digest: &str) -> Result<bool> {
        oci_blob_exists(self, repo, digest)
            .await
            .map_err(OciBlobError::into_anyhow)
    }
}

async fn oci_blob_exists(store: &RegistryStore, repo: &str, digest: &str) -> OciBlobResult<bool> {
    validate_fs_segment(repo).map_err(OciBlobError::InvalidRepository)?;

    let row = sqlx::query("SELECT path FROM oci_blobs WHERE repo = ? AND digest = ?")
        .bind(repo)
        .bind(digest)
        .fetch_optional(&store.db)
        .await?;

    match row {
        Some(r) => {
            let path: String = r.get("path");
            // Trust-but-verify: the DB says it exists; the disk must
            // agree (metadata read, not full rehash — rehash happens on
            // serve in get_oci_blob).
            // Tin-nhưng-kiểm-tra: DB nói có; đĩa phải đồng ý (đọc
            // metadata, không rehash full — rehash xảy ra lúc serve
            // trong get_oci_blob).
            Ok(std::path::Path::new(&path).exists())
        }
        None => Ok(false),
    }
}

#[cfg(test)]
#[path = "test/oci_blob_exists.rs"]
mod tests;
