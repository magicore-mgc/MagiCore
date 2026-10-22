//! Incremental registry response-body reader.
// Đọc body response tăng dần và chuyển sang file theo dung lượng thực tế.

use std::path::Path;

use anyhow::Result;
use futures_util::{Stream, StreamExt};
use sha2::Digest;
use tokio::io::AsyncWriteExt;

use crate::native::npm_registry::{DownloadedTarball, base64_encode};

use super::{cleanup::StagingCleanup, error::BodyStreamError};

use super::super::LARGE_PKG_THRESHOLD_BYTES;

pub(in crate::native::npm_registry::registry::tarball::automatic::automatic) async fn download_response_body<
    S,
    C,
    E,
>(
    stream: S,
    content_length: Option<u64>,
    destination: &Path,
) -> Result<DownloadedTarball, BodyStreamError>
where
    S: Stream<Item = std::result::Result<C, E>> + Send + Unpin,
    C: AsRef<[u8]> + Send,
    E: std::error::Error + Send + Sync + 'static,
{
    let declared_large = content_length.is_some_and(|length| length > LARGE_PKG_THRESHOLD_BYTES);
    let threshold = LARGE_PKG_THRESHOLD_BYTES as usize;
    let mut cleanup = StagingCleanup::new(destination);
    let mut output = None;
    let mut buffered = Vec::new();
    let mut hasher = sha2::Sha512::new();
    let mut bytes_len = 0u64;

    if declared_large {
        let file = tokio::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(destination)
            .await
            .map_err(BodyStreamError::StagingCreate)?;
        cleanup.arm();
        output = Some(file);
    }

    let mut stream = stream;
    while let Some(chunk) = stream.next().await {
        let chunk =
            chunk.map_err(|error| BodyStreamError::ResponseStream(anyhow::Error::new(error)))?;
        let chunk = chunk.as_ref();
        if chunk.is_empty() {
            continue;
        }

        let fits_buffer =
            output.is_none() && chunk.len() <= threshold.saturating_sub(buffered.len());
        if fits_buffer {
            buffered.extend_from_slice(chunk);
            continue;
        }

        if output.is_none() {
            let file = tokio::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(destination)
                .await
                .map_err(BodyStreamError::StagingCreate)?;
            cleanup.arm();
            output = Some(file);

            let prefix = std::mem::take(&mut buffered);
            bytes_len = u64::try_from(prefix.len()).map_err(|_| BodyStreamError::LengthOverflow)?;
            hasher.update(&prefix);
            let writer = output.as_mut().ok_or(BodyStreamError::StagingNotOpen)?;
            writer
                .write_all(&prefix)
                .await
                .map_err(BodyStreamError::StagingWrite)?;
        }

        let chunk_len = u64::try_from(chunk.len()).map_err(|_| BodyStreamError::LengthOverflow)?;
        bytes_len = bytes_len
            .checked_add(chunk_len)
            .ok_or(BodyStreamError::LengthOverflow)?;
        hasher.update(chunk);
        let writer = output.as_mut().ok_or(BodyStreamError::StagingNotOpen)?;
        writer
            .write_all(chunk)
            .await
            .map_err(BodyStreamError::StagingWrite)?;
    }

    let Some(mut file) = output else {
        return Ok(DownloadedTarball::Bytes(buffered));
    };
    file.flush().await.map_err(BodyStreamError::StagingFlush)?;
    drop(file);
    cleanup.disarm();

    let digest = hasher.finalize();
    let encoded = base64_encode(&digest);
    Ok(DownloadedTarball::Streamed {
        computed_integrity: format!("sha512-{encoded}"),
        bytes_len,
    })
}

#[cfg(test)]
#[path = "test/body_stream.rs"]
mod tests;
