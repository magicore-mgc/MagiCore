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
    // SEC-9 sibling: a quota abort used to leave the truncated archive on disk,
    // where a later step could unzip it. Any failure now removes the partial
    // file before the error propagates.
    // (Anh em với SEC-9: hủy vì quota từng để lại archive bị cắt trên đĩa và
    // bước sau có thể giải nén nó. Mọi lỗi giờ xóa file dở trước khi trả lỗi.)
    let failure = {
        let mut failure = None;
        while let Some(chunk) = stream.next().await {
            match chunk {
                Ok(chunk) => {
                    written = written.saturating_add(chunk.len() as u64);
                    if written > quota {
                        failure = Some(SelfUpdateError::download_failed(
                            url,
                            &format!("body over quota {quota}"),
                        ));
                        break;
                    }
                    if let Err(error) = out.write_all(&chunk).await {
                        failure = Some(SelfUpdateError::download_failed(url, &error.to_string()));
                        break;
                    }
                }
                Err(error) => {
                    failure = Some(SelfUpdateError::download_failed(url, &error.to_string()));
                    break;
                }
            }
        }
        if failure.is_none()
            && let Err(error) = out.flush().await
        {
            failure = Some(SelfUpdateError::download_failed(url, &error.to_string()));
        }
        failure
    };
    if let Some(failure) = failure {
        drop(out);
        let _ = tokio::fs::remove_file(dest).await;
        return Err(failure);
    }
    Ok(())
}
