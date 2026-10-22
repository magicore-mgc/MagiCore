//! Stream a registry signature page under a fixed byte ceiling.
//! Đọc streaming trang chữ ký registry với giới hạn byte cố định.

use super::error::BundleReadError;
use futures_util::StreamExt;
use mgc_registry_server::trusted::AttestationBundle;

pub(super) const MAX_SIGNATURE_BUNDLE_BYTES: usize = 16 * 1024 * 1024;

pub(in crate::commands::audit::signatures) async fn read_bundle(
    response: reqwest::Response,
) -> std::result::Result<AttestationBundle, BundleReadError> {
    ensure_response_size(response.content_length(), 0, 0)?;
    let mut stream = response.bytes_stream();
    let mut body = Vec::new();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|error| BundleReadError::Invalid(error.into()))?;
        ensure_response_size(None, body.len(), chunk.len())?;
        body.extend_from_slice(&chunk);
    }
    serde_json::from_slice(&body).map_err(|error| BundleReadError::Invalid(error.into()))
}

/// Reject oversized declared, accumulated, or incoming body lengths.
/// Từ chối độ dài khai báo, đã nhận hoặc sắp nhận vượt giới hạn.
pub(in crate::commands::audit::signatures) fn ensure_response_size(
    declared_bytes: Option<u64>,
    received_bytes: usize,
    incoming_bytes: usize,
) -> std::result::Result<(), BundleReadError> {
    if declared_bytes.is_some_and(|length| length > MAX_SIGNATURE_BUNDLE_BYTES as u64)
        || received_bytes > MAX_SIGNATURE_BUNDLE_BYTES
        || incoming_bytes > MAX_SIGNATURE_BUNDLE_BYTES.saturating_sub(received_bytes)
    {
        return Err(BundleReadError::TooLarge);
    }
    Ok(())
}
