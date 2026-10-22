//! Verify LFS SHA-256 or Git blob SHA-1 for downloaded bytes.
//! Xác minh SHA-256 LFS hoặc Git blob SHA-1 cho byte đã tải.

use mgc_types::MgResult;

use super::super::hfhub::HfHubProtocol;
use super::super::model_file::ModelFile;
use super::error::FileIntegrityError;

impl HfHubProtocol {
    /// Verify bytes using LFS SHA-256 when recorded, else Git blob id.
    /// Xác minh byte bằng SHA-256 LFS nếu có, nếu không dùng Git blob id.
    pub fn verify_file(&self, file: &ModelFile, bytes: &[u8]) -> MgResult<()> {
        if let Some(expected) = file.sha256.as_deref() {
            let actual = crate::protocols::sha256_hex(bytes);
            if !actual.eq_ignore_ascii_case(expected) {
                return Err(FileIntegrityError::Sha256Mismatch {
                    path: file.path.clone(),
                    expected: expected.to_string(),
                    actual,
                }
                .into());
            }
            return Ok(());
        }
        let actual = git_blob_sha1(bytes);
        if actual != file.git_blob_sha1 {
            return Err(FileIntegrityError::GitBlobMismatch {
                path: file.path.clone(),
                expected: file.git_blob_sha1.clone(),
                actual,
            }
            .into());
        }
        Ok(())
    }
}

/// Compute Git's blob object id over a byte slice.
/// Tính Git blob object id trên một lát byte.
pub fn git_blob_sha1(bytes: &[u8]) -> String {
    use sha1::Digest;
    let mut hasher = sha1::Sha1::new();
    hasher.update(format!("blob {}\0", bytes.len()));
    hasher.update(bytes);
    hex::encode(hasher.finalize())
}
