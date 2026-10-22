use mgc_types::{MgError, MgResult};
use std::path::Path;

pub(in crate::install) fn atomic_write(
    directory: &Path,
    relative: &Path,
    bytes: &[u8],
) -> MgResult<()> {
    use std::io::Write;
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);
    let destination = directory.join(relative);
    match std::fs::symlink_metadata(&destination) {
        Ok(metadata) if metadata.file_type().is_file() => {
            let current =
                mgc_adapter_base::project_file::read_regular_text(&destination, "checksum")?;
            if current.as_bytes() == bytes {
                return Ok(());
            }
        }
        Ok(_) => {
            return Err(MgError::Other(format!(
                "refusing non-regular managed Cargo checksum '{}'",
                destination.display()
            )));
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(MgError::Other(format!(
                "inspect checksum destination: {error}"
            )));
        }
    }
    let sequence = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
    let temp = directory.join(format!(
        ".cargo-checksum.{}.{}.tmp",
        std::process::id(),
        sequence
    ));
    let result = (|| {
        let mut options = std::fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600).custom_flags(libc::O_NOFOLLOW);
        }
        let mut file = options.open(&temp).map_err(|error| {
            MgError::Other(format!("create Cargo checksum staging file: {error}"))
        })?;
        file.write_all(bytes)
            .and_then(|()| file.sync_all())
            .map_err(|error| {
                MgError::Other(format!("write Cargo checksum staging file: {error}"))
            })?;
        match std::fs::symlink_metadata(&destination) {
            Ok(metadata) if metadata.file_type().is_file() => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Ok(_) => {
                return Err(MgError::Other(
                    "refusing linked Cargo checksum destination".to_string(),
                ));
            }
            Err(error) => {
                return Err(MgError::Other(format!(
                    "inspect Cargo checksum destination: {error}"
                )));
            }
        }
        mgc_lockfile::atomic::atomic_replace_file(&temp, &destination).map_err(|error| {
            MgError::Other(format!("replace Cargo checksum atomically: {error}"))
        })?;
        #[cfg(unix)]
        std::fs::File::open(directory)
            .and_then(|parent| parent.sync_all())
            .map_err(|error| MgError::Other(format!("sync Cargo source directory: {error}")))?;
        Ok(())
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&temp);
    }
    result
}
