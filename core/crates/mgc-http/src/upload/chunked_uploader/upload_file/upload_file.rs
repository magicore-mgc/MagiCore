// File upload — hash the blob and coordinate chunk requests.
// Upload file — hash blob và điều phối các request chunk.

use super::super::ChunkedUploader;
use super::error::OpenUploadFileError;
use anyhow::Result;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub(super) fn full_file_digest(file_path: &Path) -> Result<String> {
    // Preserve the bounded 8 KiB digest buffer — giữ buffer digest 8 KiB có giới hạn.
    let mut hasher = Sha256::new();
    let mut buffer = vec![0u8; 8192];
    let mut file = File::open(file_path)?;
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    Ok(format!("sha256:{}", hex::encode(hasher.finalize())))
}

impl ChunkedUploader {
    /// Upload a file in chunks — upload file thành nhiều chunk.
    pub async fn upload_file(&self, repo: &str, file_path: &Path) -> Result<String> {
        let file = File::open(file_path)
            .map_err(OpenUploadFileError::from)
            .map_err(anyhow::Error::new)?;
        let file_size = file.metadata()?.len();
        let digest = full_file_digest(file_path)?;

        // This starts a new session; persistent resume is not implemented.
        // Bước này tạo session mới; resume bền vững chưa được triển khai.
        let upload_id = self.start_upload(repo).await?;
        let mut offset = 0u64;
        let mut file = file;

        loop {
            let mut chunk = vec![0u8; self.chunk_size as usize];
            let count = file.read(&mut chunk)?;
            if count == 0 {
                break;
            }
            chunk.truncate(count);
            offset = self.upload_chunk(repo, &upload_id, offset, &chunk).await?;
            tracing::info!("Uploaded {} / {} bytes", offset, file_size);
        }

        let location = self.finalize_upload(repo, &upload_id, &digest).await?;
        Ok(location)
    }
}

#[cfg(test)]
#[path = "test/upload_file.rs"]
mod tests;
