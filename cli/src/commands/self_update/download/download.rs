//! Select bounded document reads and streamed archive transfers.
//! Chọn reader document có giới hạn và tải archive theo stream.

use super::bounded_response::download_capped;
use super::limits::{MAX_ARCHIVE_BYTES, MAX_DOC_BYTES};
use super::stream_to_file::download_stream;
use crate::commands::self_update::error::SelfUpdateError;
use anyhow::Result;

/// Download a URL to a path with the shared client, STREAMING with a
/// byte quota (never buffers the whole body in RAM — a hostile
/// oversized archive aborts mid-stream instead of exhausting memory).
/// (Tải URL ra file dạng stream có quota.)
pub(in crate::commands) async fn download_file(url: &str, dest: &std::path::Path) -> Result<()> {
    download_stream(url, dest, MAX_ARCHIVE_BYTES).await
}

/// Download a URL as text (checksums, manifests — small documents with
/// a tight quota).
/// (Tải URL dạng text.)
pub(in crate::commands) async fn download_text(url: &str) -> Result<String> {
    download_capped(url, MAX_DOC_BYTES).await.and_then(|bytes| {
        String::from_utf8(bytes).map_err(|e| SelfUpdateError::download_failed(url, &e.to_string()))
    })
}
