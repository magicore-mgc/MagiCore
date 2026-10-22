//! OCI blob retrieval and upload.
//! Tải và upload blob OCI.

use anyhow::{Result, bail};
use sha2::{Digest, Sha256};

use super::super::error::OciClientError;
use crate::client::OciClient;

impl OciClient {
    /// Get blob by digest (HEAD first to check existence, then GET)
    pub async fn pull_blob(&self, repo: &str, digest: &str) -> Result<Vec<u8>> {
        let url = format!(
            "{}/v2/{}/blobs/{}",
            self.base_url.trim_end_matches('/'),
            repo,
            digest
        );

        // First check if blob exists (HEAD)
        let head = self.client.get(&url).await?;
        if head.status().as_u16() == 404 {
            bail!(OciClientError::BlobNotFound(digest.to_string()));
        }
        if !head.status().is_success() {
            bail!(OciClientError::HeadBlobFailed(head.status().to_string()));
        }

        // GET the blob
        let resp = self.client.get(&url).await?;
        if !resp.status().is_success() {
            bail!(OciClientError::GetBlobFailed(resp.status().to_string()));
        }

        let data = resp.bytes().await?.to_vec();

        // Verify digest
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let computed = format!("sha256:{}", hex::encode(hasher.finalize()));
        if computed != digest {
            bail!(OciClientError::DigestMismatch {
                expected: digest.to_string(),
                got: computed,
            });
        }

        Ok(data)
    }

    /// Push blob (initiate upload, upload chunks, finalize)
    pub async fn push_blob(&self, repo: &str, data: &[u8]) -> Result<String> {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let digest = format!("sha256:{}", hex::encode(hasher.finalize()));

        // Check if already exists
        if self.blob_exists(repo, &digest).await? {
            return Ok(digest);
        }

        // Write data to temp file for upload
        let temp_dir = tempfile::tempdir()?;
        let temp_file = temp_dir.path().join("blob.bin");
        std::fs::write(&temp_file, data)?;

        // Start upload
        let uploader =
            mgc_http::upload::ChunkedUploader::new(self.client.clone(), self.base_url.clone());
        uploader.upload_file(repo, &temp_file).await?;

        Ok(digest)
    }

    async fn blob_exists(&self, repo: &str, digest: &str) -> Result<bool> {
        let url = format!(
            "{}/v2/{}/blobs/{}",
            self.base_url.trim_end_matches('/'),
            repo,
            digest
        );
        match self.client.get(&url).await {
            Ok(resp) => Ok(resp.status().is_success()),
            Err(_) => Ok(false),
        }
    }
}
