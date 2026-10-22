//! Owns resumable OCI session creation, locking, append and offset operations.
//! Sở hữu tạo phiên OCI tiếp tục được, khóa, append và thao tác offset.

use super::super::oci_limits::validate_oci_blob_size;
use super::super::prelude::*;
use super::super::*;

impl RegistryStore {
    /// Return the per-session lock used to serialize upload offset checks.
    /// Trả khóa theo session để tuần tự hóa kiểm tra offset upload.
    pub(in crate::storage) fn oci_upload_lock(
        &self,
        repo: &str,
        uuid: &str,
    ) -> Result<std::sync::Arc<tokio::sync::Mutex<()>>> {
        let mut locks = self
            .upload_locks
            .lock()
            .map_err(|_| anyhow::anyhow!("OCI upload lock map is poisoned"))?;
        locks.retain(|_, lock| lock.strong_count() > 0);
        let key = (repo.to_owned(), uuid.to_owned());
        if let Some(lock) = locks.get(&key).and_then(|lock| lock.upgrade()) {
            return Ok(lock);
        }
        let lock = std::sync::Arc::new(tokio::sync::Mutex::new(()));
        locks.insert(key, std::sync::Arc::downgrade(&lock));
        Ok(lock)
    }

    pub async fn create_oci_upload(&self, repo: &str, uuid: &str) -> Result<PathBuf> {
        // Both segments reach the filesystem — validate fail-closed.
        // Cả hai segment đều chạm filesystem — validate fail-closed.
        validate_fs_segment(repo)?;
        validate_fs_segment(uuid)?;
        let dir = self.blobs_dir.join("oci").join(repo).join("uploads");
        fs::create_dir_all(&dir).await?;
        let path = dir.join(uuid);
        sqlx::query("INSERT OR REPLACE INTO oci_uploads (repo, uuid, path, offset_bytes) VALUES (?, ?, ?, 0)")
            .bind(repo)
            .bind(uuid)
            .bind(path.to_string_lossy().to_string())
            .execute(&self.db)
            .await?;
        Ok(path)
    }

    pub async fn append_oci_upload(
        &self,
        repo: &str,
        uuid: &str,
        expected_offset: i64,
        data: &[u8],
    ) -> std::result::Result<i64, OciUploadAppendError> {
        validate_fs_segment(repo)?;
        validate_fs_segment(uuid)?;
        if expected_offset < 0 || data.is_empty() {
            return Err(OciUploadAppendError::OffsetMismatch);
        }
        let session_lock = self.oci_upload_lock(repo, uuid)?;
        let _guard = session_lock.lock().await;
        let data_len = i64::try_from(data.len())?;
        let next_offset = expected_offset
            .checked_add(data_len)
            .ok_or_else(|| anyhow::anyhow!("OCI upload offset overflow"))?;
        validate_oci_blob_size(next_offset as u64, max_oci_blob_bytes())?;
        let mut transaction = self.db.begin().await?;
        let claim = sqlx::query(
            "UPDATE oci_uploads SET offset_bytes = ? WHERE repo = ? AND uuid = ? AND offset_bytes = ?",
        )
        .bind(next_offset)
        .bind(repo)
        .bind(uuid)
        .bind(expected_offset)
        .execute(&mut *transaction)
        .await?;
        if claim.rows_affected() != 1 {
            let exists = sqlx::query_scalar::<_, i64>(
                "SELECT EXISTS(SELECT 1 FROM oci_uploads WHERE repo = ? AND uuid = ?)",
            )
            .bind(repo)
            .bind(uuid)
            .fetch_one(&mut *transaction)
            .await?;
            transaction.rollback().await?;
            return Err(if exists == 0 {
                OciUploadAppendError::SessionNotFound
            } else {
                OciUploadAppendError::OffsetMismatch
            });
        }
        let row = sqlx::query("SELECT path FROM oci_uploads WHERE repo = ? AND uuid = ?")
            .bind(repo)
            .bind(uuid)
            .fetch_optional(&mut *transaction)
            .await?;
        let Some(row) = row else {
            transaction.rollback().await?;
            return Err(OciUploadAppendError::SessionNotFound);
        };
        let path = PathBuf::from(row.get::<String, _>("path"));
        let file_len = match fs::metadata(&path).await {
            Ok(metadata) => metadata.len(),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound && expected_offset == 0 => 0,
            Err(error) => return Err(error.into()),
        };
        if file_len < expected_offset as u64 {
            return Err(anyhow::anyhow!(
                "OCI upload file length is shorter than its stored offset"
            )
            .into());
        }
        if file_len > expected_offset as u64 {
            Self::truncate_oci_upload(&path, expected_offset as u64).await?;
        }
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .await?;
        if let Err(error) = file.write_all(data).await {
            drop(file);
            let _ = Self::truncate_oci_upload(&path, expected_offset as u64).await;
            let _ = transaction.rollback().await;
            return Err(error.into());
        }
        if let Err(error) = file.flush().await {
            drop(file);
            let _ = Self::truncate_oci_upload(&path, expected_offset as u64).await;
            let _ = transaction.rollback().await;
            return Err(error.into());
        }
        if let Err(error) = file.sync_data().await {
            drop(file);
            let _ = Self::truncate_oci_upload(&path, expected_offset as u64).await;
            let _ = transaction.rollback().await;
            return Err(error.into());
        }
        drop(file);
        if fs::metadata(&path).await?.len() != next_offset as u64 {
            let _ = Self::truncate_oci_upload(&path, expected_offset as u64).await;
            transaction.rollback().await?;
            return Err(
                anyhow::anyhow!("OCI upload chunk did not write the declared byte count").into(),
            );
        }
        transaction.commit().await?;
        Ok(next_offset)
    }

    async fn truncate_oci_upload(path: &Path, offset: u64) -> Result<()> {
        let file = fs::OpenOptions::new().write(true).open(path).await?;
        file.set_len(offset).await?;
        Ok(())
    }

    pub async fn oci_upload_path(&self, repo: &str, uuid: &str) -> Result<Option<PathBuf>> {
        let row = sqlx::query("SELECT path FROM oci_uploads WHERE repo = ? AND uuid = ?")
            .bind(repo)
            .bind(uuid)
            .fetch_optional(&self.db)
            .await?;
        Ok(row.map(|r| PathBuf::from(r.get::<String, _>("path"))))
    }

    pub async fn oci_upload_offset(&self, repo: &str, uuid: &str) -> Result<Option<i64>> {
        validate_fs_segment(repo)?;
        validate_fs_segment(uuid)?;
        let offset = sqlx::query_scalar::<_, i64>(
            "SELECT offset_bytes FROM oci_uploads WHERE repo = ? AND uuid = ?",
        )
        .bind(repo)
        .bind(uuid)
        .fetch_optional(&self.db)
        .await?;
        Ok(offset)
    }
}
