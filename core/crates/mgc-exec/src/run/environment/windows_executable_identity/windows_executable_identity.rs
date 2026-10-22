//! Resolves and streams the Windows executable SHA-256 identity.
//! Lấy và băm SHA-256 executable Windows theo luồng.

use std::path::Path;

use super::super::EnvironmentError;

#[cfg(windows)]
static CURRENT_EXECUTABLE_SHA256: std::sync::OnceLock<[u8; 32]> = std::sync::OnceLock::new();
#[cfg(windows)]
pub(in crate::run::implementation) fn windows_current_executable_sha256()
-> std::result::Result<[u8; 32], EnvironmentError> {
    if let Some(digest) = CURRENT_EXECUTABLE_SHA256.get() {
        return Ok(*digest);
    }

    let executable = std::env::current_exe().map_err(EnvironmentError::ResolveCurrentExecutable)?;
    let digest = windows_sha256_file(&executable)?;
    match CURRENT_EXECUTABLE_SHA256.set(digest) {
        Ok(()) => Ok(digest),
        Err(_already_initialized) => CURRENT_EXECUTABLE_SHA256
            .get()
            .copied()
            .ok_or(EnvironmentError::ExecutableHashCacheUnavailable),
    }
}

#[cfg(windows)]
pub(in crate::run::implementation) fn windows_sha256_file(
    path: &Path,
) -> std::result::Result<[u8; 32], EnvironmentError> {
    use sha2::{Digest, Sha256};

    let mut file =
        std::fs::File::open(path).map_err(|source| EnvironmentError::OpenExecutable {
            path: path.to_path_buf(),
            source,
        })?;
    use std::io::Read;

    let mut digest = Sha256::new();
    // Bound identity hashing memory independently of executable size.
    // Đọc theo khối cố định, không nạp toàn bộ executable vào RAM.
    const HASH_BUFFER_BYTES: usize = 64 * 1024;
    let mut buffer = [0_u8; HASH_BUFFER_BYTES];
    loop {
        let read = file
            .read(&mut buffer)
            .map_err(|source| EnvironmentError::ReadExecutable {
                path: path.to_path_buf(),
                source,
            })?;
        if read == 0 {
            break;
        }
        digest.update(&buffer[..read]);
    }
    Ok(digest.finalize().into())
}
