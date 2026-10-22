//! Atomic private keyring publication — Publish keyring private dạng atomic.
//! Preserve write, sync, replace and cleanup order — Giữ thứ tự write/sync/cleanup.

use crate::CryptoResult;
use crate::keyring::storage::atomic_replace::atomic_replace_file;
use crate::keyring::storage::temporary_keyring_file::TemporaryKeyringFile;
use ring::rand::{SecureRandom, SystemRandom};
use std::fs;
use std::path::Path;

const MAX_TEMPORARY_KEYRING_NAME_ATTEMPTS: usize = 16;

pub(in crate::keyring) fn write_private_file_atomic(
    path: &Path,
    contents: &[u8],
) -> CryptoResult<()> {
    use std::io::Write;

    let parent = path
        .parent()
        .ok_or(crate::keyring::error::KeyringError::MissingParentDirectory)?;
    let rng = SystemRandom::new();

    for _ in 0..MAX_TEMPORARY_KEYRING_NAME_ATTEMPTS {
        let mut nonce = [0u8; 16];
        rng.fill(&mut nonce)
            .map_err(|_| crate::keyring::error::KeyringError::TemporaryNameGeneration)?;
        let nonce_hex = nonce
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>();
        let file_name = path
            .file_name()
            .unwrap_or_else(|| std::ffi::OsStr::new("keyring"))
            .to_string_lossy();
        let temporary_path = parent.join(format!(".{file_name}.{nonce_hex}.tmp"));

        let mut options = fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600).custom_flags(libc::O_NOFOLLOW);
        }

        let mut file = match options.open(&temporary_path) {
            Ok(file) => file,
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(error.into()),
        };
        let mut cleanup = TemporaryKeyringFile::new(temporary_path.clone());
        file.write_all(contents)?;
        file.sync_all()?;
        drop(file);
        atomic_replace_file(&temporary_path, path)?;
        cleanup.disarm();

        #[cfg(unix)]
        fs::File::open(parent)?.sync_all()?;

        return Ok(());
    }

    Err(crate::keyring::error::KeyringError::TemporaryNameExhausted.into())
}
