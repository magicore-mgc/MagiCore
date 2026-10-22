use mgc_types::{MgError, MgResult};
use std::path::{Path, PathBuf};

pub(in crate::install::download) fn ensure_safe_model_destination(
    root: &Path,
    relative: &Path,
) -> MgResult<PathBuf> {
    use std::path::Component;

    let canonical_root = root.canonicalize()?;
    let mut parent = canonical_root.clone();
    let components = relative.components().collect::<Vec<_>>();
    for component in components.iter().take(components.len().saturating_sub(1)) {
        let Component::Normal(segment) = component else {
            return Err(MgError::Other(
                "invalid model artifact path component".into(),
            ));
        };
        parent.push(segment);
        match std::fs::symlink_metadata(&parent) {
            Ok(metadata) if metadata.file_type().is_dir() => {}
            Ok(_) => {
                return Err(MgError::Integrity(format!(
                    "model artifact parent is not a real directory: {}",
                    parent.display()
                )));
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                match std::fs::create_dir(&parent) {
                    Ok(()) => {}
                    Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                        let metadata = std::fs::symlink_metadata(&parent)?;
                        if !metadata.file_type().is_dir() {
                            return Err(MgError::Integrity(format!(
                                "model artifact parent is not a real directory: {}",
                                parent.display()
                            )));
                        }
                    }
                    Err(error) => return Err(error.into()),
                }
            }
            Err(error) => return Err(error.into()),
        }
    }
    let Component::Normal(file_name) = components
        .last()
        .copied()
        .ok_or_else(|| MgError::Other("empty model artifact path".into()))?
    else {
        return Err(MgError::Other("invalid model artifact filename".into()));
    };
    let dest = parent.join(file_name);
    match std::fs::symlink_metadata(&dest) {
        Ok(metadata) if metadata.file_type().is_file() => {}
        Ok(_) => {
            return Err(MgError::Integrity(format!(
                "model artifact destination is not a regular file: {}",
                dest.display()
            )));
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error.into()),
    }
    if !dest.starts_with(&canonical_root) {
        return Err(MgError::Integrity(
            "model artifact destination escaped its cache root".into(),
        ));
    }
    Ok(dest)
}
