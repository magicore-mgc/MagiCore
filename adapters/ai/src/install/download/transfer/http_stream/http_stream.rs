use super::super::{MAX_REMOTE_MODEL_BYTES_ENV, max_remote_model_bytes, next_model_download_total};
use mgc_types::{MgError, MgResult};
use std::path::Path;

/// Stream an HTTP artifact to a unique staging path; publish happens only
/// after checksum verification in `download_model`.
/// Stream artifact vào staging riêng; chỉ publish sau khi verify checksum.
pub(in crate::install) async fn http_download_to_staging(
    url: &str,
    staging: &Path,
) -> MgResult<u64> {
    http_download_to_staging_with_limit(url, staging, max_remote_model_bytes()?).await
}

pub(in crate::install) async fn http_download_to_staging_with_limit(
    url: &str,
    staging: &Path,
    max_bytes: u64,
) -> MgResult<u64> {
    use tokio::io::AsyncWriteExt;

    let client = reqwest::Client::builder()
        .connect_timeout(std::time::Duration::from_secs(20))
        .timeout(std::time::Duration::from_secs(2 * 60 * 60))
        .build()
        .map_err(|error| MgError::Network(format!("create model HTTP client: {error}")))?;
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| MgError::Network(format!("download failed: {e}")))?;

    if !response.status().is_success() {
        return Err(MgError::Network(format!(
            "model download returned HTTP {}",
            response.status()
        )));
    }

    let declared_length = response.content_length();
    if declared_length.is_some_and(|declared| declared > max_bytes) {
        return Err(MgError::Integrity(format!(
            "model download exceeds {MAX_REMOTE_MODEL_BYTES_ENV} limit ({max_bytes} bytes)"
        )));
    }
    let mut response = response;
    let mut file = tokio::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(staging)
        .await
        .map_err(|error| MgError::Other(format!("create model staging file: {error}")))?;
    let mut written = 0_u64;
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|error| MgError::Network(format!("read model response body: {error}")))?
    {
        written = next_model_download_total(written, chunk.len(), max_bytes)?;
        file.write_all(&chunk)
            .await
            .map_err(|error| MgError::Other(format!("write model staging file: {error}")))?;
    }
    if declared_length.is_some_and(|expected| expected != written) {
        return Err(MgError::Integrity(format!(
            "model download length mismatch: expected {}, received {written}",
            declared_length.unwrap_or_default()
        )));
    }
    file.sync_all()
        .await
        .map_err(|error| MgError::Other(format!("sync model staging file: {error}")))?;
    Ok(written)
}
