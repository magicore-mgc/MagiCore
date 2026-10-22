//! Download one file from its locked URL.
//! Tải một file từ URL đã lock.

use mgc_types::{MgError, MgResult};

use super::super::hfhub::HfHubProtocol;
use super::super::model_file::ModelFile;

impl HfHubProtocol {
    /// Download one locked file through the shared HTTP client.
    /// Tải một file đã lock qua HTTP client dùng chung.
    pub async fn download_file(&self, file: &ModelFile) -> MgResult<Vec<u8>> {
        let response = self
            .client
            .get(&file.url)
            .await
            .map_err(|error| MgError::Network(format!("GET {} failed: {error}", file.url)))?;
        let status = response.status();
        let bytes = response
            .bytes()
            .await
            .map_err(|error| MgError::Network(format!("read {} body failed: {error}", file.url)))?;
        if !status.is_success() {
            return Err(MgError::Network(format!(
                "GET {} returned {status} (fail-closed)",
                file.url
            )));
        }
        Ok(bytes.to_vec())
    }
}
