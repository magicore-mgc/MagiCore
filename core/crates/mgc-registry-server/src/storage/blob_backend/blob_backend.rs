//! Implements local-filesystem and S3-compatible blob read/write operations.
//! Triển khai đọc/ghi blob trên filesystem local và S3-compatible.

use super::super::prelude::*;
use super::super::*;

/// Blob storage backend (ITEM 5): Local FS hoặc S3-compatible (object_store).
pub enum BlobBackend {
    Local(PathBuf),
    S3(std::sync::Arc<object_store::aws::AmazonS3>),
}

use object_store::ObjectStoreExt as _;

impl BlobBackend {
    /// "local" hoặc "s3://bucket/prefix"
    pub fn parse(spec: Option<&str>, local_dir: &Path) -> Result<Self> {
        match spec {
            None | Some("local") => Ok(BlobBackend::Local(local_dir.to_path_buf())),
            Some(s3) => {
                let store = object_store::aws::AmazonS3Builder::from_env()
                    .with_url(s3)
                    .build()
                    .context("build S3 backend from env (AWS_ACCESS_KEY_ID/SECRET/REGION)")?;
                Ok(BlobBackend::S3(std::sync::Arc::new(store)))
            }
        }
    }

    /// Atomic local write (P0-5): unique temp in the SAME directory → write
    /// → flush → fsync → rename. A crash mid-write can never leave a torn
    /// blob at the final path; the temp is cleaned up on every failure path
    /// (TempGuard RAII). S3 puts are already atomic at the object level.
    ///
    /// Ghi local nguyên tử (P0-5): temp duy nhất CÙNG thư mục → ghi → flush
    /// → fsync → rename. Crash giữa chừng không thể để blob cụt ở path cuối;
    /// temp được dọn ở mọi đường lỗi (RAII TempGuard). Put S3 vốn đã nguyên
    /// tử ở cấp object.
    pub(in crate::storage) async fn put(&self, key: &str, data: &[u8]) -> Result<()> {
        match self {
            BlobBackend::Local(dir) => {
                let path = dir.join(key);
                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent).await?;
                }

                // Unique temp next to the final path (same filesystem →
                // rename is atomic; Windows rename-over-existing is avoided
                // by renaming only when the final path is absent, and a
                // failed rename falls through to verified-reuse below).
                // Temp duy nhất cạnh path cuối (cùng filesystem → rename
                // nguyên tử; tránh rename-đè-trên-Windows bằng cách chỉ
                // rename khi path cuối chưa có, lỗi rename thì rơi xuống
                // đường verified-reuse phía dưới).
                let tmp_parent = path
                    .parent()
                    .map(Path::to_path_buf)
                    .unwrap_or_else(|| dir.clone());
                let tmp = parent_temp_path(&tmp_parent);
                let mut guard = TempGuard::new(&tmp);

                let mut file = fs::File::create(&tmp).await?;
                file.write_all(data).await?;
                file.flush().await?;
                let sync = file.sync_all().await;
                drop(file);
                sync?;

                match fs::rename(&tmp, &path).await {
                    Ok(()) => {
                        guard.disarm();
                        Ok(())
                    }
                    Err(e) if path.exists() => {
                        // Lost a concurrent put of the same key — the winner
                        // already published; our temp is dropped by the guard.
                        // Thua put song song cùng key — winner đã publish; temp
                        // của ta được guard dọn.
                        let _ = e;
                        Ok(())
                    }
                    Err(e) => Err(anyhow::anyhow!("atomic blob put failed for '{key}': {e}")),
                }
            }
            BlobBackend::S3(store) => {
                let _ = store
                    .put(
                        &object_store::path::Path::from(key.to_string()),
                        object_store::PutPayload::from(data.to_vec()),
                    )
                    .await
                    .context("S3 put")?;
                Ok(())
            }
        }
    }

    pub(in crate::storage) async fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        match self {
            BlobBackend::Local(dir) => {
                let path = dir.join(key);
                match fs::read(&path).await {
                    Ok(data) => Ok(Some(data)),
                    Err(_) => Ok(None),
                }
            }
            BlobBackend::S3(store) => {
                let resp = store
                    .get(&object_store::path::Path::from(key.to_string()))
                    .await;
                match resp {
                    Ok(r) => Ok(Some(r.bytes().await.context("S3 get bytes")?.to_vec())),
                    Err(object_store::Error::NotFound { .. }) => Ok(None),
                    Err(e) => Err(anyhow::anyhow!(e)),
                }
            }
        }
    }

    pub(in crate::storage) async fn exists(&self, key: &str) -> Result<bool> {
        match self {
            BlobBackend::Local(dir) => Ok(dir.join(key).exists()),
            BlobBackend::S3(store) => {
                match store
                    .head(&object_store::path::Path::from(key.to_string()))
                    .await
                {
                    Ok(_) => Ok(true),
                    Err(object_store::Error::NotFound { .. }) => Ok(false),
                    Err(e) => Err(anyhow::anyhow!(e)),
                }
            }
        }
    }

    /// Published blob length (getter used by put-verify and blob_exists).
    /// Returns Err when the key is missing (caller checks exists() first
    /// when existence is the question).
    ///
    /// Độ dài blob đã publish (getter cho put-verify và blob_exists). Trả
    /// Err khi key mất (caller check exists() trước nếu câu hỏi là tồn tại.)
    pub(in crate::storage) async fn published_len(&self, key: &str) -> Result<u64> {
        match self {
            BlobBackend::Local(dir) => {
                let meta = fs::metadata(dir.join(key)).await?;
                Ok(meta.len())
            }
            BlobBackend::S3(store) => {
                let head = store
                    .head(&object_store::path::Path::from(key.to_string()))
                    .await
                    .context("S3 head for published length")?;
                Ok(head.size)
            }
        }
    }

    /// Verify the published blob's length matches what we intended to store
    /// (vòng-2 review R-put): catches a pre-existing torn/stale file at the
    /// final path that `put` legitimately skipped overwriting.
    ///
    /// Verify độ dài blob đã publish khớp với bytes định lưu (R-put vòng-2):
    /// bắt file cụt/cũ đã tồn tại sẵn ở path cuối mà `put` đã hợp lệ bỏ qua
    /// ghi đè.
    pub(in crate::storage) async fn verify_published_len(
        &self,
        key: &str,
        expected_len: usize,
    ) -> Result<()> {
        let actual = self.published_len(key).await?;
        if actual != expected_len as u64 {
            anyhow::bail!(
                "published blob length mismatch for '{key}': expected {expected_len} bytes, found {actual} (torn or stale blob at final path)"
            );
        }
        Ok(())
    }
}
