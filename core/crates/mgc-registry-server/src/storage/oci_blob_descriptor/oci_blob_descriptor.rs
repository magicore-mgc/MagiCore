//! Owns bounded OCI blob descriptor size and digest verification.
//! Sở hữu xác minh size và digest descriptor OCI blob có giới hạn.

use super::super::prelude::*;
use super::super::*;

impl RegistryStore {
    /// Verify a manifest descriptor without allocating a layer-sized buffer.
    /// Xác minh descriptor manifest mà không cấp phát buffer cỡ toàn bộ layer.
    pub async fn verify_oci_blob_descriptor(
        &self,
        repo: &str,
        digest: &str,
        expected_size: u64,
    ) -> Result<bool> {
        validate_fs_segment(repo)?;
        let row = sqlx::query("SELECT path, size FROM oci_blobs WHERE repo = ? AND digest = ?")
            .bind(repo)
            .bind(digest)
            .fetch_optional(&self.db)
            .await?;
        let Some(row) = row else {
            return Ok(false);
        };
        let path: String = row.get("path");
        let stored_size: i64 = row.get("size");
        if stored_size < 0 || stored_size as u64 != expected_size {
            return Ok(false);
        }
        let metadata = match fs::metadata(&path).await {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
            Err(error) => return Err(error.into()),
        };
        if metadata.len() != expected_size {
            return Ok(false);
        }
        let actual = hash_file_sha256(Path::new(&path)).await?;
        if format!("sha256:{actual}") != digest {
            quarantine_corrupt_blob(&path, digest, &format!("sha256:{actual}"))?;
            anyhow::bail!("OCI blob integrity mismatch for '{digest}' in repo '{repo}'");
        }
        Ok(true)
    }
}
