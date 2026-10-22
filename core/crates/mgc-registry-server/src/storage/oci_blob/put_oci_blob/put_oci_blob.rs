//! Writes digest-verified OCI blobs atomically and records their registry row.
//! Ghi OCI blob đã xác minh digest theo cách nguyên tử và cập nhật registry row.

use super::super::error::{OciBlobError, OciBlobResult};
use crate::storage::prelude::*;
use crate::storage::{RegistryStore, digest_hex_path, max_oci_blob_bytes, validate_fs_segment};

impl RegistryStore {
    pub async fn put_oci_blob(&self, repo: &str, digest: &str, data: &[u8]) -> Result<()> {
        put_oci_blob(self, repo, digest, data)
            .await
            .map_err(OciBlobError::into_anyhow)
    }
}

async fn put_oci_blob(
    store: &RegistryStore,
    repo: &str,
    digest: &str,
    data: &[u8],
) -> OciBlobResult<()> {
    // Repo is an OCI path segment — validate before joining the FS.
    // Repo là path segment OCI — validate trước khi join vào filesystem.
    validate_fs_segment(repo).map_err(OciBlobError::InvalidRepository)?;
    let repo_dir = store.blobs_dir.join("oci").join(repo);
    fs::create_dir_all(&repo_dir).await?;

    let (p1, p2) = digest_hex_path(digest).map_err(OciBlobError::InvalidDigest)?;
    let path = repo_dir.join(&p1).join(&p2);

    // Defense in depth: the HTTP handler already verifies the digest,
    // but the storage API recomputes it too — a future caller that
    // forgets cannot poison the blob store (P0-2).
    // Phòng vệ nhiều lớp: HTTP handler đã verify digest, nhưng storage
    // API tự tính lại — caller tương lai quên verify không thể đầu độc
    // blob store (P0-2).
    let actual = RegistryStore::compute_wire_digest(digest, data)?;
    if actual != digest {
        return Err(OciBlobError::DigestMismatch {
            declared: digest.to_owned(),
            actual,
        });
    }
    if data.len() as u64 > max_oci_blob_bytes() {
        return Err(OciBlobError::SizeLimitExceeded);
    }

    let parent = path
        .parent()
        .context("OCI blob path has no parent directory")?;
    fs::create_dir_all(parent).await?;

    // Atomic write: unique temp + rename. Concurrent pushes of the same
    // digest cannot leave a torn blob at the final path.
    // Ghi nguyên tử: temp duy nhất + rename. Push song song cùng digest
    // không thể để lại blob nửa vời ở path cuối.
    let tmp = parent.join(format!(
        ".upload-{}-{}",
        uuid::Uuid::new_v4(),
        std::process::id()
    ));
    let mut file = fs::File::create(&tmp).await?;
    file.write_all(data).await?;
    file.flush().await?;
    let sync = file.sync_all().await;
    drop(file);
    sync?;
    fs::rename(&tmp, &path).await?;

    sqlx::query(
        r#"
            INSERT INTO oci_blobs (repo, digest, size, path)
            VALUES (?, ?, ?, ?)
            ON CONFLICT(repo, digest) DO UPDATE SET size = excluded.size, path = excluded.path
        "#,
    )
    .bind(repo)
    .bind(digest)
    .bind(data.len() as i64)
    .bind(path.to_string_lossy().to_string())
    .execute(&store.db)
    .await?;

    Ok(())
}

#[cfg(test)]
#[path = "test/put_oci_blob.rs"]
mod tests;
