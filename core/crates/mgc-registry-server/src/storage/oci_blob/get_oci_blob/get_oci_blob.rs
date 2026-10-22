//! Reads OCI blobs and quarantines data whose digest no longer matches.
//! Đọc OCI blob và cách ly dữ liệu có digest không còn khớp.

use super::super::error::{OciBlobError, OciBlobResult};
use crate::storage::prelude::*;
use crate::storage::{RegistryStore, quarantine_corrupt_blob, validate_fs_segment};

impl RegistryStore {
    /// Serve an OCI blob with integrity verification (P0-6): the bytes are
    /// re-hashed (SHA-256) and compared to the declared digest BEFORE they
    /// reach the client. A corrupted/tampered on-disk blob is quarantined
    /// for forensics and reported as a hard error — poison is never served.
    ///
    /// Phục vụ blob OCI có kiểm tra tính toàn vẹn (P0-6): bytes được rehash
    /// (SHA-256) và so với digest khai TRƯỚC khi tới client. Blob hỏng/sửa
    /// trên đĩa bị cách ly để điều tra và báo lỗi cứng — không bao giờ serve
    /// dữ liệu bẩn.
    pub async fn get_oci_blob(&self, repo: &str, digest: &str) -> Result<Option<Vec<u8>>> {
        get_oci_blob(self, repo, digest)
            .await
            .map_err(OciBlobError::into_anyhow)
    }
}

async fn get_oci_blob(
    store: &RegistryStore,
    repo: &str,
    digest: &str,
) -> OciBlobResult<Option<Vec<u8>>> {
    // Validate the repo before DB/FS access; verify the requested digest after reading bytes.
    // Validate repo trước khi chạm DB/FS; kiểm tra digest yêu cầu sau khi đọc bytes.
    validate_fs_segment(repo).map_err(OciBlobError::InvalidRepository)?;

    let row = sqlx::query("SELECT path FROM oci_blobs WHERE repo = ? AND digest = ?")
        .bind(repo)
        .bind(digest)
        .fetch_optional(&store.db)
        .await?;

    let Some(row) = row else {
        return Ok(None);
    };
    let path: String = row.get("path");
    let data = fs::read(&path).await?;

    // Recompute the digest on the serving path — same algorithm the
    // put path used, so a store-entry mismatch fails closed here.
    // Tính lại digest trên đường serve — cùng thuật toán với đường put,
    // entry lệch sẽ fail cứng tại đây.
    let actual = RegistryStore::compute_wire_digest(digest, &data)?;
    if actual != digest {
        // Quarantine the corrupt blob for forensics (best-effort) and
        // fail closed — the client must not receive tampered bytes.
        // Cách ly blob hỏng để điều tra (best-effort) rồi fail cứng —
        // client không được nhận bytes đã bị sửa.
        quarantine_corrupt_blob(&path, digest, &actual)?;
        return Err(OciBlobError::IntegrityMismatch {
            digest: digest.to_owned(),
            repo: repo.to_owned(),
            actual,
        });
    }
    Ok(Some(data))
}

#[cfg(test)]
#[path = "test/get_oci_blob.rs"]
mod tests;
