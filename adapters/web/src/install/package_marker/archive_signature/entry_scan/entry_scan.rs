//! Read regular tar files and reject links or unsupported entry types.
//! Đọc file tar thường và từ chối link/entry type không hỗ trợ.

use std::io::Read;

use mgc_types::{MgError, MgResult};
use sha2::{Digest, Sha256};

use super::super::archive_path::sanitize_tarball_signature_path;
use crate::install::package_marker::{SHA256_READ_BUFFER_BYTES, mode_is_executable};

pub(in crate::install::package_marker::archive_signature) type TarballFileSignature =
    (String, u64, bool, String);

pub(in crate::install::package_marker::archive_signature) fn read_tarball_files<R: Read>(
    archive: &mut tar::Archive<R>,
) -> MgResult<Vec<TarballFileSignature>> {
    let mut files = Vec::<TarballFileSignature>::new();
    for entry in archive
        .entries()
        .map_err(|err| MgError::Other(format!("failed to read tarball entries: {err}")))?
    {
        let mut entry =
            entry.map_err(|err| MgError::Other(format!("failed to read tarball entry: {err}")))?;
        let entry_type = entry.header().entry_type();
        if entry_type.is_dir() || matches!(entry_type.as_byte(), b'g' | b'x') {
            continue;
        }
        if entry_type.is_symlink() || entry_type.is_hard_link() {
            return Err(MgError::Other(format!(
                "tar links are not allowed in cached package signature: {}",
                entry
                    .path()
                    .map(|path| path.display().to_string())
                    .unwrap_or_default()
            )));
        }
        if !entry_type.is_file() {
            return Err(MgError::Other(format!(
                "unsupported tar entry type in cached package signature: {}",
                entry
                    .path()
                    .map(|path| path.display().to_string())
                    .unwrap_or_default()
            )));
        }

        let path = sanitize_tarball_signature_path(
            entry
                .path()
                .map_err(|err| MgError::Other(format!("failed to read tarball entry path: {err}")))?
                .as_ref(),
        )?;
        let size = entry.header().size().map_err(|err| {
            MgError::Other(format!(
                "failed to read tarball entry size '{}': {err}",
                path.display()
            ))
        })?;
        let mode = entry.header().mode().map_err(|err| {
            MgError::Other(format!(
                "failed to read tarball entry mode '{}': {err}",
                path.display()
            ))
        })?;
        let mut content_hasher = Sha256::new();
        let mut actual_size = 0u64;
        let mut buffer = [0u8; SHA256_READ_BUFFER_BYTES];
        loop {
            let read = entry.read(&mut buffer).map_err(|err| {
                MgError::Other(format!(
                    "failed to read tarball entry '{}': {err}",
                    path.display()
                ))
            })?;
            if read == 0 {
                break;
            }
            actual_size = actual_size.checked_add(read as u64).ok_or_else(|| {
                MgError::Other(format!(
                    "tarball entry '{}' byte count overflowed",
                    path.display()
                ))
            })?;
            content_hasher.update(&buffer[..read]);
        }
        if actual_size != size {
            return Err(MgError::Other(format!(
                "tarball entry '{}' size mismatch: header {size}, read {actual_size}",
                path.display()
            )));
        }
        files.push((
            super::super::archive_path::path_to_signature_string(&path),
            size,
            mode_is_executable(mode),
            hex::encode(content_hasher.finalize()),
        ));
    }
    Ok(files)
}
