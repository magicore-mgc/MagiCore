//! Owns OCI upload completion and verified blob publication.
//! Sở hữu hoàn tất OCI upload và publish blob đã xác minh.

use super::super::prelude::*;
use super::super::*;

impl RegistryStore {
    pub async fn finish_oci_upload(
        &self,
        repo: &str,
        uuid: &str,
        expected_offset: i64,
    ) -> Result<bool> {
        validate_fs_segment(repo)?;
        validate_fs_segment(uuid)?;
        let session_lock = self.oci_upload_lock(repo, uuid)?;
        let _guard = session_lock.lock().await;
        let path = self.oci_upload_path(repo, uuid).await?;
        let result =
            sqlx::query("DELETE FROM oci_uploads WHERE repo = ? AND uuid = ? AND offset_bytes = ?")
                .bind(repo)
                .bind(uuid)
                .bind(expected_offset)
                .execute(&self.db)
                .await?;
        if result.rows_affected() == 0 {
            return Ok(false);
        }
        if let Some(path) = path {
            match fs::remove_file(path).await {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => return Err(error.into()),
            }
        }
        Ok(true)
    }

    /// Verify and publish an upload file with bounded memory, then remove its
    /// session while SQLite excludes concurrent appends from other instances.
    /// Xác minh và publish file upload với RAM giới hạn, rồi xóa session trong
    /// khi SQLite chặn append đồng thời từ instance khác.
    pub async fn finalize_oci_upload(
        &self,
        repo: &str,
        uuid: &str,
        expected_digest: &str,
    ) -> Result<u64> {
        validate_fs_segment(repo)?;
        validate_fs_segment(uuid)?;
        let session_lock = self.oci_upload_lock(repo, uuid)?;
        let _guard = session_lock.lock().await;
        let mut transaction = self.db.begin().await?;
        let row =
            sqlx::query("SELECT path, offset_bytes FROM oci_uploads WHERE repo = ? AND uuid = ?")
                .bind(repo)
                .bind(uuid)
                .fetch_optional(&mut *transaction)
                .await?
                .ok_or_else(|| anyhow::anyhow!("OCI upload session not found"))?;
        let path = PathBuf::from(row.get::<String, _>("path"));
        let expected_offset: i64 = row.get("offset_bytes");
        let claimed =
            sqlx::query("DELETE FROM oci_uploads WHERE repo = ? AND uuid = ? AND offset_bytes = ?")
                .bind(repo)
                .bind(uuid)
                .bind(expected_offset)
                .execute(&mut *transaction)
                .await?;
        if claimed.rows_affected() != 1 {
            anyhow::bail!("OCI upload changed while it was being finalized");
        }
        let size = fs::metadata(&path).await?.len();
        if expected_offset < 0 || size != expected_offset as u64 || size > max_oci_blob_bytes() {
            anyhow::bail!("OCI upload size or stored offset is invalid");
        }
        let written_size = self
            .publish_oci_blob_from_file(repo, expected_digest, &path, size, &mut transaction)
            .await?;
        transaction.commit().await?;
        match fs::remove_file(path).await {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
        Ok(written_size)
    }

    async fn publish_oci_blob_from_file(
        &self,
        repo: &str,
        digest: &str,
        source_path: &Path,
        expected_size: u64,
        transaction: &mut sqlx::Transaction<'_, Sqlite>,
    ) -> Result<u64> {
        use sha2::Digest;

        validate_fs_segment(repo)?;
        let repo_dir = self.blobs_dir.join("oci").join(repo);
        let (p1, p2) = digest_hex_path(digest)?;
        let path = repo_dir.join(p1).join(p2);
        let parent = path.parent().context("OCI blob path has no parent")?;
        fs::create_dir_all(parent).await?;
        let tmp = parent.join(format!(
            ".upload-{}-{}",
            uuid::Uuid::new_v4(),
            std::process::id()
        ));
        let mut guard = TempGuard::new(&tmp);
        let mut source = fs::File::open(source_path).await?;
        let mut output = fs::File::create(&tmp).await?;
        let mut buffer = vec![0; 128 * 1024];
        let mut hasher = sha2::Sha256::new();
        let mut total = 0_u64;
        loop {
            let count = source.read(&mut buffer).await?;
            if count == 0 {
                break;
            }
            total = total
                .checked_add(count as u64)
                .ok_or_else(|| anyhow::anyhow!("OCI blob size overflow"))?;
            if total > max_oci_blob_bytes() {
                anyhow::bail!("OCI blob exceeds configured maximum size");
            }
            hasher.update(&buffer[..count]);
            output.write_all(&buffer[..count]).await?;
        }
        let actual = format!("sha256:{}", hex::encode(hasher.finalize()));
        if total != expected_size || actual != digest {
            anyhow::bail!("OCI uploaded blob size or digest does not match its declaration");
        }
        output.flush().await?;
        output.sync_all().await?;
        drop(output);
        fs::rename(&tmp, &path).await?;
        guard.disarm();
        sqlx::query(
            "INSERT INTO oci_blobs (repo, digest, size, path) VALUES (?, ?, ?, ?) ON CONFLICT(repo, digest) DO UPDATE SET size = excluded.size, path = excluded.path",
        )
        .bind(repo)
        .bind(digest)
        .bind(i64::try_from(total)?)
        .bind(path.to_string_lossy().to_string())
        .execute(&mut **transaction)
        .await?;
        Ok(total)
    }
}
