// Finalize upload — PUT the completed blob with its digest.
// Hoàn tất upload — PUT blob hoàn chỉnh kèm digest.

use super::super::ChunkedUploader;
use super::error::FinalizeUploadError;
use anyhow::Result;

pub(super) fn build_finalize_url(
    base_url: &str,
    repo: &str,
    upload_id: &str,
    expected_digest: &str,
) -> String {
    format!(
        "{}/v2/{}/blobs/uploads/{}?digest={}",
        base_url.trim_end_matches('/'),
        repo,
        upload_id,
        expected_digest
    )
}

impl ChunkedUploader {
    /// Finalize the upload session — hoàn tất session upload.
    pub async fn finalize_upload(
        &self,
        repo: &str,
        upload_id: &str,
        expected_digest: &str,
    ) -> Result<String> {
        let url = build_finalize_url(&self.base_url, repo, upload_id, expected_digest);
        let resp = self.client.put(&url, Vec::new()).await?;
        if !resp.status().is_success() {
            return Err(anyhow::Error::new(FinalizeUploadError::Failed(
                resp.status(),
            )));
        }

        // The server returns the finalized blob Location.
        // Server trả Location của blob đã finalize.
        let location = resp
            .headers()
            .get("location")
            .ok_or_else(|| anyhow::Error::new(FinalizeUploadError::MissingLocation))?
            .to_str()?;
        Ok(location.to_string())
    }
}

#[cfg(test)]
#[path = "test/finalize_upload.rs"]
mod tests;
