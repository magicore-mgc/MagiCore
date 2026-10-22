use mgc_types::{MgError, MgResult};
use std::path::Path;

pub(in crate::install::download) fn ensure_real_directory(path: &Path) -> MgResult<()> {
    match std::fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_dir() => Ok(()),
        Ok(_) => Err(MgError::Integrity(format!(
            "model destination root must be a real directory: {}",
            path.display()
        ))),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            std::fs::create_dir_all(path)?;
            let metadata = std::fs::symlink_metadata(path)?;
            if metadata.file_type().is_dir() {
                Ok(())
            } else {
                Err(MgError::Integrity(format!(
                    "model destination root must be a real directory: {}",
                    path.display()
                )))
            }
        }
        Err(error) => Err(error.into()),
    }
}
