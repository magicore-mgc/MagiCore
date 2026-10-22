//! Reads, validates, creates, and locates the registry signing key under the bounded no-symlink policy.
//! Đọc, xác minh, tạo và tìm signing key registry theo policy bounded/no-symlink.

use super::super::prelude::*;

const SIGNING_KEY_FILE: &str = "registry-attestation-key.json";

const MAX_SIGNING_KEY_BYTES: u64 = 16 * 1024;

/// Load a private signing key without following symlinks, or create it with owner-only permissions.
/// Tải khóa ký riêng không theo symlink, hoặc tạo khóa chỉ chủ sở hữu được đọc.
pub fn load_or_create_signing_key(store_dir: &Path) -> Result<KeyPair> {
    let path = store_dir.join(SIGNING_KEY_FILE);
    match read_signing_key(&path) {
        Ok(Some(key)) => return Ok(key),
        Ok(None) => {}
        Err(error) => return Err(error),
    }

    let key = KeyPair::generate().context("generate registry provenance key")?;
    let serialized = serde_json::to_vec(&key).context("serialize registry provenance key")?;
    let mut temporary = tempfile::NamedTempFile::new_in(store_dir)
        .context("create temporary registry provenance key")?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        temporary
            .as_file()
            .set_permissions(std::fs::Permissions::from_mode(0o600))
            .context("secure temporary registry provenance key")?;
    }
    temporary
        .write_all(&serialized)
        .context("write registry provenance key")?;
    temporary
        .as_file()
        .sync_all()
        .context("sync registry provenance key")?;
    match temporary.persist_noclobber(&path) {
        Ok(_) => Ok(key),
        Err(error) if error.error.kind() == std::io::ErrorKind::AlreadyExists => {
            read_signing_key(&path)?.context("registry provenance key disappeared")
        }
        Err(error) => Err(error.error).context("persist registry provenance key"),
    }
}

pub fn signing_key_path(store_dir: &Path) -> PathBuf {
    store_dir.join(SIGNING_KEY_FILE)
}

fn read_signing_key(path: &Path) -> Result<Option<KeyPair>> {
    let metadata = match std::fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error).context("inspect registry provenance key"),
    };
    if !metadata.file_type().is_file() || metadata.len() > MAX_SIGNING_KEY_BYTES {
        anyhow::bail!("registry provenance key is not a bounded regular file");
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if metadata.permissions().mode() & 0o077 != 0 {
            anyhow::bail!("registry provenance key permissions must be 0600");
        }
    }
    let content = std::fs::read(path).context("read registry provenance key")?;
    let key: KeyPair = serde_json::from_slice(&content).context("parse registry provenance key")?;
    let signer = key.signer().context("validate registry provenance key")?;
    let expected_key_id =
        hex::encode(&mgc_crypto::Blake3Hasher::hash_bytes(&key.public_key.0).0[..8]);
    if signer.public_key() != key.public_key || key.key_id != expected_key_id {
        anyhow::bail!("registry provenance key pair does not match");
    }
    Ok(Some(key))
}
