//! Policy-governed download with a hard byte cap and partial-file cleanup.
//! Tải file theo policy HTTP chung, giới hạn byte cứng và dọn file dở khi hủy.
//!
//! SEC-9: every download goes through `mgc_http::HttpClient` so TLS, timeout,
//! proxy and retry policy match the rest of the product. The old
//! `reqwest::Client::new()` bypassed that policy and streamed an unbounded
//! body to disk.
//! (SEC-9: mọi lần tải đi qua `mgc_http::HttpClient` để TLS/timeout/proxy/retry
//! khớp phần còn lại của sản phẩm. `reqwest::Client::new()` cũ bỏ qua policy
//! và ghi body không giới hạn xuống đĩa.)

use anyhow::{Context, Result};
use futures_util::StreamExt;
use mgc_http::HttpClient;
use std::path::Path;
use tokio::io::AsyncWriteExt;

/// Default hard cap for a streamed download body.
/// Giới hạn byte mặc định cho một body tải stream.
pub const DEFAULT_MAX_DOWNLOAD_BYTES: u64 = 1024 * 1024 * 1024;

/// Environment override for the download cap, in bytes.
/// Biến môi trường ghi đè giới hạn tải, đơn vị byte.
pub const MAX_DOWNLOAD_BYTES_ENV: &str = "MAGICORE_MAX_DOWNLOAD_BYTES";

/// Download progress callback: `(downloaded_bytes, total_bytes_or_zero)`.
/// Callback tiến độ: `(số byte đã tải, tổng số byte hoặc 0 nếu chưa biết)`.
pub type ProgressCallback = Box<dyn Fn(u64, u64) + Send + Sync>;

/// Download with streaming and progress tracking without loading the payload
/// into RAM. The body is capped at [`DEFAULT_MAX_DOWNLOAD_BYTES`] (or the
/// [`MAX_DOWNLOAD_BYTES_ENV`] override); on any failure, including the cap
/// being exceeded, the partially written file is deleted.
/// (Tải stream kèm tiến độ, không nạp cả payload vào RAM. Body bị chặn theo
/// [`DEFAULT_MAX_DOWNLOAD_BYTES`] (hoặc biến môi trường
/// [`MAX_DOWNLOAD_BYTES_ENV`]); khi lỗi — kể cả vượt giới hạn — file dở bị xóa.)
pub async fn download_with_progress(
    url: &str,
    dest: &Path,
    progress: Option<ProgressCallback>,
) -> Result<()> {
    let cap = max_download_bytes();
    let client = HttpClient::new()?;
    download_with_progress_and_client(&client, url, dest, cap, progress).await
}

/// Resolve the effective download cap: environment override (validated) or the
/// default constant. An invalid or zero override is rejected loudly rather than
/// silently disabling the cap.
/// (Xác định giới hạn hiệu lực: biến môi trường hợp lệ hoặc hằng mặc định.
/// Giá trị sai/0 bị từ chối rõ ràng thay vì âm thầm bỏ giới hạn.)
pub fn max_download_bytes() -> u64 {
    match std::env::var(MAX_DOWNLOAD_BYTES_ENV) {
        Ok(raw) => match raw.trim().parse::<u64>() {
            Ok(value) if value > 0 => value,
            _ => {
                eprintln!(
                    "WARNING: {MAX_DOWNLOAD_BYTES_ENV}='{raw}' is not a positive integer; \
                     using the {} byte default limit",
                    DEFAULT_MAX_DOWNLOAD_BYTES
                );
                DEFAULT_MAX_DOWNLOAD_BYTES
            }
        },
        Err(_) => DEFAULT_MAX_DOWNLOAD_BYTES,
    }
}

/// Shared streaming implementation used by the public entry point and tests.
/// Phần triển khai stream dùng chung cho entry point công khai và test.
pub(crate) async fn download_with_progress_and_client(
    client: &HttpClient,
    url: &str,
    dest: &Path,
    max_bytes: u64,
    progress: Option<ProgressCallback>,
) -> Result<()> {
    // Reject the cap before any I/O so a zero cap can never mean "unlimited".
    // Kiểm tra giới hạn trước mọi I/O để cap 0 không bao giờ thành "vô hạn".
    if max_bytes == 0 {
        anyhow::bail!("download byte limit must be greater than zero");
    }

    let response = client
        .get(url)
        .await
        .with_context(|| format!("failed to send GET request to {url}"))?;
    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("download of {url} failed with HTTP status {status}");
    }

    // Reject an oversized declared body before creating the destination file.
    // Từ chối body khai báo quá lớn trước khi tạo file đích.
    if let Some(length) = response.content_length() {
        if length > max_bytes {
            anyhow::bail!(
                "declared download size {length} bytes exceeds the {max_bytes} byte limit for {url}"
            );
        }
    }
    let total_size = response.content_length().unwrap_or(0);

    if let Some(parent) = dest.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    let mut file = tokio::fs::File::create(dest)
        .await
        .with_context(|| format!("failed to create output file '{}'", dest.display()))?;

    let mut stream = response.bytes_stream();
    let mut downloaded: u64 = 0;

    loop {
        let chunk = match stream.next().await {
            Some(Ok(chunk)) => chunk,
            Some(Err(error)) => {
                return abort_download(dest, file, error.into()).await;
            }
            None => break,
        };
        downloaded = downloaded.saturating_add(chunk.len() as u64);
        if downloaded > max_bytes {
            // Cap exceeded mid-stream: the declared length lied or was absent.
            // Vượt giới hạn giữa stream: Content-Length nói dối hoặc không có.
            return abort_download(
                dest,
                file,
                anyhow::anyhow!(
                    "download of {url} exceeded the {max_bytes} byte limit; partial file removed"
                ),
            )
            .await;
        }
        if let Err(error) = file.write_all(&chunk).await {
            return abort_download(dest, file, error.into()).await;
        }
        if let Some(ref cb) = progress {
            cb(downloaded, total_size);
        }
    }

    file.flush().await?;
    file.sync_all().await?;
    Ok(())
}

/// Close and delete a partially written destination, then propagate `error`.
/// Đóng và xóa file đích đang dở, sau đó trả lỗi gốc.
async fn abort_download(
    dest: &Path,
    file: tokio::fs::File,
    error: anyhow::Error,
) -> Result<()> {
    drop(file);
    // Removal failure must not mask the original error; the original error is
    // what the caller acts on.
    // (Lỗi xóa không được che lỗi gốc — caller hành động theo lỗi gốc.)
    let _ = tokio::fs::remove_file(dest).await;
    Err(error)
}

#[cfg(test)]
#[path = "test/download.rs"]
mod tests;
