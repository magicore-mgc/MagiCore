//! Copies an OCI blob between repositories and records the destination row.
//! Sao chép OCI blob giữa các repository và ghi row đích.

use super::super::error::{OciBlobError, OciBlobResult};
use crate::storage::prelude::*;
use crate::storage::{RegistryStore, digest_hex_path, validate_fs_segment};

impl RegistryStore {
    /// Copy an OCI blob from another repository; return false when the source is absent.
    /// Sao chép blob OCI từ repository khác; trả false khi nguồn không tồn tại.
    pub async fn mount_oci_blob(
        &self,
        from_repo: &str,
        digest: &str,
        to_repo: &str,
    ) -> Result<bool> {
        mount_oci_blob(self, from_repo, digest, to_repo)
            .await
            .map_err(OciBlobError::into_anyhow)
    }
}

async fn mount_oci_blob(
    store: &RegistryStore,
    from_repo: &str,
    digest: &str,
    to_repo: &str,
) -> OciBlobResult<bool> {
    validate_fs_segment(from_repo).map_err(OciBlobError::InvalidRepository)?;
    validate_fs_segment(to_repo).map_err(OciBlobError::InvalidRepository)?;
    let row = sqlx::query("SELECT path FROM oci_blobs WHERE repo = ? AND digest = ?")
        .bind(from_repo)
        .bind(digest)
        .fetch_optional(&store.db)
        .await?;
    let Some(row) = row else { return Ok(false) };
    let src_path: String = row.get("path");

    let repo_dir = store.blobs_dir.join("oci").join(to_repo);
    let (p1, p2) = digest_hex_path(digest).map_err(OciBlobError::InvalidDigest)?;
    let dest = repo_dir.join(&p1).join(&p2);
    let parent = dest
        .parent()
        .context("mounted OCI blob path has no parent directory")?;
    fs::create_dir_all(parent).await?;
    if !dest.exists() {
        fs::copy(&src_path, &dest).await?;
    }
    sqlx::query(
        r#"
            INSERT INTO oci_blobs (repo, digest, size, path)
            VALUES (?, ?, ?, ?)
            ON CONFLICT(repo, digest) DO UPDATE SET path = excluded.path
        "#,
    )
    .bind(to_repo)
    .bind(digest)
    .bind(
        get_oci_blob_size(store, from_repo, digest)
            .await?
            .unwrap_or(0),
    )
    .bind(dest.to_string_lossy().to_string())
    .execute(&store.db)
    .await?;
    Ok(true)
}

async fn get_oci_blob_size(
    store: &RegistryStore,
    repo: &str,
    digest: &str,
) -> OciBlobResult<Option<i64>> {
    let size =
        sqlx::query_scalar::<_, i64>("SELECT size FROM oci_blobs WHERE repo = ? AND digest = ?")
            .bind(repo)
            .bind(digest)
            .fetch_optional(&store.db)
            .await?;
    Ok(size)
}

#[cfg(test)]
#[path = "test/mount_oci_blob.rs"]
mod tests;
