//! registry tarball stream.
// Chứa đúng trách nhiệm của module; giữ nguyên hành vi, API, chính sách lỗi và bảo mật hiện có.

use crate::native::npm_registry::{NpmRegistry, base64_encode, with_retry};
use anyhow::Result;

impl NpmRegistry {
    /// **Fast-lane streaming download** — writes directly to `dest` without
    /// holding the full tarball in RAM. Returns the SHA-512 integrity hash
    /// (`sha512-<base64>`) computed on-the-fly as the stream is written.
    ///
    /// This is the primary path for large packages (> LARGE_PKG_THRESHOLD_BYTES).
    pub async fn download_tarball_to_file(
        &self,
        url: &str,
        dest: &std::path::Path,
    ) -> Result<String> {
        use futures_util::StreamExt;
        use sha2::Digest;
        use tokio::io::AsyncWriteExt;

        with_retry("tarball-stream", url, || async {
            if let Some(parent) = dest.parent() {
                tokio::fs::create_dir_all(parent).await?;
            }
            let resp = self
                .request_for(url, true)?
                .send()
                .await?
                .error_for_status()?;

            let mut file = tokio::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(dest)
                .await?;
            let mut stream = resp.bytes_stream();
            let mut hasher = sha2::Sha512::new();

            let streamed = async {
                while let Some(chunk) = stream.next().await {
                    let chunk = chunk?;
                    hasher.update(&chunk);
                    file.write_all(&chunk).await?;
                }
                file.flush().await?;
                Ok::<(), anyhow::Error>(())
            }
            .await;
            if let Err(error) = streamed {
                drop(file);
                let _ = tokio::fs::remove_file(dest).await;
                return Err(error);
            }

            let digest = hasher.finalize();
            let b64 = base64_encode(&digest);
            Ok(format!("sha512-{b64}"))
        })
        .await
    }
}
