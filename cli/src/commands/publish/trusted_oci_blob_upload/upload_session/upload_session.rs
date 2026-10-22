use anyhow::{Result, bail};
use std::time::Duration;

pub(in crate::commands) async fn begin_oci_blob_upload(
    client: &reqwest::Client,
    registry: &url::Url,
    repository: &str,
    token: &str,
    timeout: Duration,
) -> Result<url::Url> {
    let start_url = registry.join(&format!("/v2/{repository}/blobs/uploads/"))?;
    let response = client
        .post(start_url)
        .bearer_auth(token)
        .timeout(timeout)
        .header(reqwest::header::CONTENT_LENGTH, "0")
        .send()
        .await?;
    if response.status() != reqwest::StatusCode::ACCEPTED {
        return Err(crate::error::trusted_oci_upload_start_failed(
            response.status().as_u16(),
        ));
    }
    let location = response
        .headers()
        .get(reqwest::header::LOCATION)
        .and_then(|value| value.to_str().ok())
        .ok_or_else(crate::error::trusted_oci_upload_location_missing)?;
    safe_oci_upload_location(registry, repository, location)
}

/// Accept upload URLs only from the configured registry and repository.
/// Chỉ chấp nhận URL upload cùng registry và đúng repository đã cấu hình.
pub(in crate::commands) fn safe_oci_upload_location(
    registry: &url::Url,
    repository: &str,
    location: &str,
) -> Result<url::Url> {
    let upload_url = registry.join(location)?;
    let expected_prefix = format!("/v2/{repository}/blobs/uploads/");
    let session = upload_url
        .path()
        .strip_prefix(&expected_prefix)
        .filter(|session| {
            !session.is_empty()
                && session.bytes().all(|byte| {
                    byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~')
                })
                && !matches!(*session, "." | "..")
        });
    if upload_url.origin() != registry.origin()
        || !upload_url.username().is_empty()
        || upload_url.password().is_some()
        || upload_url.fragment().is_some()
        || session.is_none()
    {
        bail!("OCI registry returned an unsafe upload Location");
    }
    Ok(upload_url)
}
