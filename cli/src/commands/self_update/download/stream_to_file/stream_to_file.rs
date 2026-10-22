//! Stream large HTTP response bodies to disk under a byte quota.
//! Ghi body HTTP lớn xuống đĩa theo stream và quota byte.

use crate::commands::self_update::error::SelfUpdateError;
use anyhow::Result;

pub(in crate::commands::self_update) async fn download_stream(
    url: &str,
    dest: &std::path::Path,
    quota: u64,
) -> Result<()> {
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
    use tokio::io::AsyncWriteExt;
    let file = tokio::fs::File::create(dest)
        .await
        .map_err(|e| SelfUpdateError::download_failed(url, &e.to_string()))?;
    let mut out = tokio::io::BufWriter::new(file);
    let mut written: u64 = 0;
    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| SelfUpdateError::download_failed(url, &e.to_string()))?;
        written += chunk.len() as u64;
        if written > quota {
            return Err(SelfUpdateError::download_failed(
                url,
                &format!("body over quota {quota}"),
            ));
        }
        out.write_all(&chunk)
            .await
            .map_err(|e| SelfUpdateError::download_failed(url, &e.to_string()))?;
    }
    out.flush()
        .await
        .map_err(|e| SelfUpdateError::download_failed(url, &e.to_string()))?;
    Ok(())
}
