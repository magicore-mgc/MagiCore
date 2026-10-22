// Start-upload operation — POST to create an OCI upload session.
// Thao tác bắt đầu upload — POST để tạo session upload OCI.

use super::super::ChunkedUploader;
use super::error::StartUploadError;
use anyhow::Result;

pub(super) fn build_start_url(base_url: &str, repo: &str) -> String {
    format!(
        "{}/v2/{}/blobs/uploads/",
        base_url.trim_end_matches('/'),
        repo
    )
}

impl ChunkedUploader {
    /// Create an upload session — tạo session upload.
    pub async fn start_upload(&self, repo: &str) -> Result<String> {
        let url = build_start_url(&self.base_url, repo);
        let resp = self.client.post(&url, Vec::new()).await?;
        if !resp.status().is_success() {
            return Err(anyhow::Error::new(StartUploadError::Failed(resp.status())));
        }

        // The server returns the upload identifier in Location.
        // Server trả mã phiên upload trong header Location.
        let location = resp
            .headers()
            .get("location")
            .ok_or_else(|| anyhow::Error::new(StartUploadError::MissingLocation))?
            .to_str()?;
        let upload_id = location
            .split('/')
            .next_back()
            .ok_or_else(|| {
                anyhow::Error::new(StartUploadError::InvalidLocation(location.to_string()))
            })?
            .to_string();
        Ok(upload_id)
    }
}

#[cfg(test)]
#[path = "test/start_upload.rs"]
mod tests;
