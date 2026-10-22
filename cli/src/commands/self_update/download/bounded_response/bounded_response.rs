//! Read small HTTP response bodies under a byte quota.
//! Đọc body HTTP nhỏ trong quota byte.

use crate::commands::self_update::error::SelfUpdateError;
use anyhow::Result;

pub(in crate::commands) async fn download_capped(url: &str, quota: u64) -> Result<Vec<u8>> {
    let client = mgc_http::HttpClient::default();
    let response = client
        .get(url)
        .await
        .map_err(|e| SelfUpdateError::download_failed(url, &e.to_string()))?;
    let status = response.status();
    if !status.is_success() {
        return Err(SelfUpdateError::download_failed(
            url,
            &format!("HTTP {status}"),
        ));
    }
    if let Some(len) = response.content_length()
        && len > quota
    {
        return Err(SelfUpdateError::download_failed(
            url,
            &format!("declared {len} bytes over quota {quota}"),
        ));
    }
    use futures_util::StreamExt;
    let mut stream = response.bytes_stream();
    let mut buf = Vec::new();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| SelfUpdateError::download_failed(url, &e.to_string()))?;
        if (buf.len() as u64) + (chunk.len() as u64) > quota {
            return Err(SelfUpdateError::download_failed(
                url,
                &format!("body over quota {quota}"),
            ));
        }
        buf.extend_from_slice(&chunk);
    }
    Ok(buf)
}
