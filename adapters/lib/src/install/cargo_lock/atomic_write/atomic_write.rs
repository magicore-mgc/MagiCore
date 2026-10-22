//! Atomically write Cargo.lock without replacing symlinks or signed locks.
//! Ghi Cargo.lock atomic, không thay symlink hoặc lock đã ký.

use mgc_types::{MgError, MgResult};
use std::path::Path;

/// Atomically create or replace the Cargo lock that backs an empty native
/// Rust graph. Existing symlinks/special files are refused before replacement.
/// Tạo/thay Cargo.lock atomic cho graph Rust native rỗng; từ chối link/file đặc biệt.
pub(crate) fn atomic_write_cargo_lock(project_root: &Path, bytes: &[u8]) -> MgResult<()> {
    use std::io::Write;
    use std::time::{SystemTime, UNIX_EPOCH};

    let destination = project_root.join("Cargo.lock");
    let destination_metadata = match std::fs::symlink_metadata(&destination) {
        Ok(metadata) if metadata.file_type().is_file() => {
            let current =
                mgc_adapter_base::project_file::read_regular_text(&destination, "Cargo.lock")?;
            if current.as_bytes() == bytes {
                return Ok(());
            }
            Some(metadata)
        }
        Ok(_) => {
            return Err(MgError::Other(
                "refusing non-regular or symlink Cargo.lock".to_string(),
            ));
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
        Err(error) => return Err(MgError::Other(format!("inspect Cargo.lock: {error}"))),
    };
    let signature_path = mgc_lockfile::parser::signature_path_for(&destination);
    match std::fs::symlink_metadata(&signature_path) {
        Ok(_) => {
            return Err(MgError::Other(format!(
                "refusing to replace Cargo.lock while signature sidecar '{}' exists",
                signature_path.display()
            )));
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(MgError::Other(format!(
                "cannot inspect Cargo.lock signature sidecar '{}': {error}",
                signature_path.display()
            )));
        }
    }
    let parent = destination.parent().unwrap_or_else(|| Path::new("."));
    let filename = destination
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| MgError::Other("Cargo.lock path has no valid filename".to_string()))?;
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| MgError::Other(format!("system clock is invalid: {error}")))?
        .as_nanos();
    let temp = parent.join(format!(".{filename}.{}.{nonce:x}.tmp", std::process::id()));

    let result = (|| {
        let mut options = std::fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600).custom_flags(libc::O_NOFOLLOW);
        }
        let mut file = options
            .open(&temp)
            .map_err(|error| MgError::Other(format!("create Cargo.lock staging file: {error}")))?;
        if let Some(metadata) = &destination_metadata {
            file.set_permissions(metadata.permissions())
                .map_err(|error| {
                    MgError::Other(format!("preserve Cargo.lock permissions: {error}"))
                })?;
        }
        file.write_all(bytes)
            .and_then(|()| file.sync_all())
            .map_err(|error| MgError::Other(format!("sync Cargo.lock staging file: {error}")))?;
        match std::fs::symlink_metadata(&destination) {
            Ok(metadata) if metadata.file_type().is_file() => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Ok(_) => {
                return Err(MgError::Other(
                    "refusing non-regular or symlink Cargo.lock".to_string(),
                ));
            }
            Err(error) => return Err(MgError::Other(format!("inspect Cargo.lock: {error}"))),
        }
        replace_lock_file(&temp, &destination)
            .map_err(|error| MgError::Other(format!("replace Cargo.lock atomically: {error}")))?;
        #[cfg(unix)]
        std::fs::File::open(parent)
            .and_then(|directory| directory.sync_all())
            .map_err(|error| MgError::Other(format!("sync Cargo.lock directory: {error}")))?;
        Ok(())
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&temp);
    }
    result
}

/// Atomically replace a lockfile on every supported OS. Rust's Windows
/// `rename` does not promise replacement when the destination exists, so
/// use the same write-through MoveFileEx semantics as mgc-lockfile.
/// (Thay lockfile nguyên tử trên mọi OS; Windows cần cờ replace tường minh.)
pub(crate) fn replace_lock_file(temp: &Path, destination: &Path) -> std::io::Result<()> {
    mgc_lockfile::atomic::atomic_replace_file(temp, destination)
}
