use super::super::safe_oci_upload_location;
use anyhow::{Result, anyhow};
use std::time::Duration;

/// Bearer token and deadline that must apply to one OCI upload request.
/// Token Bearer và thời hạn bắt buộc áp dụng cho một request tải OCI.
pub(in crate::commands::publish) struct OciRequestAuth<'a> {
    pub(in crate::commands::publish::trusted_oci_blob_upload) token: &'a str,
    pub(in crate::commands::publish::trusted_oci_blob_upload) timeout: Duration,
}

pub(in crate::commands::publish) async fn upload_oci_chunk(
    client: &reqwest::Client,
    registry: &url::Url,
    repository: &str,
    upload_url: &mut url::Url,
    offset: u64,
    chunk: &[u8],
    request_auth: OciRequestAuth<'_>,
) -> Result<()> {
    if chunk.is_empty() {
        return Ok(());
    }
    let end = offset
        .checked_add(chunk.len() as u64)
        .and_then(|next| next.checked_sub(1))
        .ok_or_else(|| anyhow!("OCI blob size overflow"))?;
    let response = client
        .patch(upload_url.clone())
        .bearer_auth(request_auth.token)
        .timeout(request_auth.timeout)
        .header("Content-Range", format!("{offset}-{end}"))
        .header(reqwest::header::CONTENT_TYPE, "application/octet-stream")
        .body(chunk.to_vec())
        .send()
        .await?;
    if response.status() != reqwest::StatusCode::ACCEPTED {
        return Err(crate::error::trusted_oci_upload_chunk_failed(
            response.status().as_u16(),
        ));
    }
    let location = response
        .headers()
        .get(reqwest::header::LOCATION)
        .and_then(|value| value.to_str().ok())
        .ok_or_else(crate::error::trusted_oci_upload_location_missing)?;
    *upload_url = safe_oci_upload_location(registry, repository, location)?;
    let expected_range = format!("0-{end}");
    if response
        .headers()
        .get(reqwest::header::RANGE)
        .and_then(|value| value.to_str().ok())
        != Some(expected_range.as_str())
    {
        return Err(crate::error::trusted_oci_upload_range_invalid());
    }
    Ok(())
}

pub(in crate::commands::publish) async fn finish_oci_blob_upload(
    client: &reqwest::Client,
    upload_url: &url::Url,
    digest: &str,
    token: &str,
    timeout: Duration,
) -> Result<()> {
    let mut finish_url = upload_url.clone();
    finish_url.query_pairs_mut().append_pair("digest", digest);
    let response = client
        .put(finish_url)
        .bearer_auth(token)
        .timeout(timeout)
        .body(Vec::new())
        .send()
        .await?;
    if response.status() != reqwest::StatusCode::CREATED {
        return Err(crate::error::trusted_oci_upload_complete_failed(
            response.status().as_u16(),
        ));
    }
    Ok(())
}
