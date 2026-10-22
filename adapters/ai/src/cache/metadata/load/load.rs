//! Load metadata without following the sidecar's final link — đọc metadata không theo symlink cuối.

use crate::cache::metadata::{CacheEntry, MetadataError, metadata_path};
use mgc_types::{MgError, MgResult};
use std::fs;
use std::io::Read;
use std::path::PathBuf;

/// Load a JSON sidecar or fall back to metadata derived from the model path.
/// Đọc JSON sidecar hoặc fallback sang metadata lấy từ model path.
pub fn load_metadata(path: &PathBuf) -> MgResult<CacheEntry> {
    let sidecar = metadata_path(path);
    match fs::symlink_metadata(&sidecar) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return CacheEntry::from_path("unknown", path);
        }
        Err(source) => {
            return Err(MgError::from(MetadataError::io(
                "inspect metadata sidecar",
                source,
            )));
        }
        Ok(metadata) if metadata.file_type().is_symlink() => {
            return Err(MetadataError::SymlinkSidecar.into());
        }
        Ok(metadata) if !metadata.file_type().is_file() => {
            return Err(MetadataError::NonRegularSidecar.into());
        }
        Ok(_) => {}
    }

    let mut file = mgc_platform::open_regular_file_no_follow(&sidecar)
        .map_err(|source| MgError::from(MetadataError::OpenSidecar(source)))?;
    let mut json = String::new();
    file.read_to_string(&mut json)
        .map_err(|source| MgError::from(MetadataError::io("read metadata sidecar", source)))?;
    serde_json::from_str(&json).map_err(|source| MgError::from(MetadataError::Deserialize(source)))
}
