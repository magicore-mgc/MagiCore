//! Deletes an OCI blob database row and attempts to remove its backing file.
//! Xóa database row OCI blob và thử gỡ file nền tương ứng.

use super::super::error::{OciBlobError, OciBlobResult};
use crate::storage::RegistryStore;
use crate::storage::prelude::*;

impl RegistryStore {
    pub async fn delete_oci_blob(&self, repo: &str, digest: &str) -> Result<bool> {
        delete_oci_blob(self, repo, digest)
            .await
            .map_err(OciBlobError::into_anyhow)
    }
}

async fn delete_oci_blob(store: &RegistryStore, repo: &str, digest: &str) -> OciBlobResult<bool> {
    let row = sqlx::query("SELECT path FROM oci_blobs WHERE repo = ? AND digest = ?")
        .bind(repo)
        .bind(digest)
        .fetch_optional(&store.db)
        .await?;
    if let Some(row) = row {
        let path: String = row.get("path");
        // Preserve the existing best-effort removal contract for this extraction.
        // Giữ nguyên hợp đồng gỡ file best-effort trong lần tách cấu trúc này.
        let _ = fs::remove_file(&path).await;
        sqlx::query("DELETE FROM oci_blobs WHERE repo = ? AND digest = ?")
            .bind(repo)
            .bind(digest)
            .execute(&store.db)
            .await?;
        return Ok(true);
    }
    Ok(false)
}

#[cfg(test)]
#[path = "test/delete_oci_blob.rs"]
mod tests;
