//! Own unique download staging paths and atomic cache publication.
//! Quản lý đường dẫn staging riêng và công bố cache nguyên tử.

use std::path::{Path, PathBuf};

use super::StagingError;

/// Give each concurrent install its own same-filesystem download staging path.
/// A fixed `<tarball>.tmp` lets one process rename another process's in-flight
/// download, producing missing or truncated package cache entries.
/// Mỗi install đồng thời có staging riêng cùng filesystem; tên `.tmp` cố định
/// khiến process khác có thể rename nhầm download đang chạy.
pub(crate) fn unique_tarball_staging_path(final_path: &Path) -> PathBuf {
    let parent = final_path.parent().unwrap_or_else(|| Path::new("."));
    let name = final_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("package.tgz");
    parent.join(format!(
        ".{name}.download-{}-{}.tmp",
        std::process::id(),
        uuid::Uuid::new_v4()
    ))
}

pub(crate) struct DownloadStagingGuard(PathBuf);

impl DownloadStagingGuard {
    pub(crate) fn new(path: PathBuf) -> Self {
        Self(path)
    }

    pub(crate) fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for DownloadStagingGuard {
    fn drop(&mut self) {
        // The file can be absent after successful publication; cleanup is
        // best-effort so cancellation/failure never leaves owned staging.
        // File đã được chuyển khi thành công; drop chỉ dọn staging còn sót.
        let _ = std::fs::remove_file(&self.0);
    }
}

/// Publish a fully downloaded tarball without replacing another writer's
/// cache entry. A losing writer accepts only a winner with the same SHA-512.
/// Publish file đã tải xong theo kiểu first-writer-wins; writer thua chỉ nhận
/// cache thắng nếu SHA-512 khớp.
pub(crate) fn publish_downloaded_tarball(
    staging_path: &Path,
    final_path: &Path,
    expected_integrity: &str,
) -> Result<(), StagingError> {
    if let Some(parent) = final_path.parent() {
        std::fs::create_dir_all(parent).map_err(StagingError::CreateDirectory)?;
    }

    let publish_result = match std::fs::hard_link(staging_path, final_path) {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => {
            verify_cached_tarball_integrity(final_path, expected_integrity)
        }
        Err(source) => Err(StagingError::Publish {
            path: final_path.display().to_string(),
            source,
        }),
    };

    if publish_result.is_err() {
        let _ = std::fs::remove_file(staging_path);
        return publish_result;
    }
    std::fs::remove_file(staging_path).map_err(|source| StagingError::RemoveStaging {
        path: staging_path.display().to_string(),
        source,
    })
}

fn verify_cached_tarball_integrity(
    path: &Path,
    expected_integrity: &str,
) -> Result<(), StagingError> {
    use base64::Engine;
    use sha2::{Digest, Sha512};
    use std::io::Read;

    let metadata = std::fs::symlink_metadata(path).map_err(StagingError::InspectWinner)?;
    if !metadata.file_type().is_file() {
        return Err(StagingError::WinnerNotRegular(path.display().to_string()));
    }

    let mut file = std::fs::File::open(path).map_err(StagingError::OpenWinner)?;
    let mut hasher = Sha512::new();
    let mut buffer = [0u8; 64 * 1024];
    loop {
        let read = file.read(&mut buffer).map_err(StagingError::ReadWinner)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    let actual = format!(
        "sha512-{}",
        base64::engine::general_purpose::STANDARD.encode(hasher.finalize())
    );
    if actual != expected_integrity {
        return Err(StagingError::WinnerIntegrityConflict(
            path.display().to_string(),
        ));
    }
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
