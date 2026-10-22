//! Stage, rename and sync a project file through an opened parent dirfd.
//! Tạo staging, rename và sync file qua parent dirfd đã mở.

use super::super::{
    error::{error, error_with_source},
    project_path::project_parent_dir,
};
use mgc_types::MgError;
use std::path::Path;

#[cfg(unix)]
pub(in crate::project_file) fn atomic_write_project_regular_dirfd(
    project_root: &Path,
    relative_path: &Path,
    bytes: &[u8],
) -> Result<(), MgError> {
    use rustix::fs::{Mode, OFlags, openat, renameat, unlinkat};
    use std::io::Write;
    use std::os::fd::AsFd;

    let (parent, leaf) = project_parent_dir(project_root, relative_path)?;
    let existing = openat(
        parent.as_fd(),
        &leaf,
        OFlags::RDONLY | OFlags::NOFOLLOW | OFlags::NONBLOCK | OFlags::CLOEXEC,
        Mode::empty(),
    )
    .map_err(|source| {
        error_with_source(source, |e| {
            format!("open existing project manifest without following links: {e}")
        })
    })?;
    let existing = std::fs::File::from(existing);
    let metadata = existing.metadata().map_err(|source| {
        error_with_source(source, |e| {
            format!("inspect existing project manifest: {e}")
        })
    })?;
    if !metadata.is_file() {
        return Err(error("refusing to replace a non-regular project manifest"));
    }

    let temp = std::ffi::OsString::from(format!(
        ".{}.mgc-tmp-{}-{}",
        leaf.to_string_lossy(),
        std::process::id(),
        uuid::Uuid::new_v4()
    ));
    let staged = openat(
        parent.as_fd(),
        &temp,
        OFlags::WRONLY | OFlags::CREATE | OFlags::EXCL | OFlags::NOFOLLOW | OFlags::CLOEXEC,
        Mode::from_bits_truncate(0o600),
    )
    .map_err(|source| {
        error_with_source(source, |e| {
            format!("create project manifest staging file: {e}")
        })
    })?;
    let result = (|| {
        let mut file = std::fs::File::from(staged);
        file.set_permissions(metadata.permissions())
            .map_err(|source| {
                error_with_source(source, |e| {
                    format!("preserve project manifest permissions: {e}")
                })
            })?;
        file.write_all(bytes)
            .and_then(|()| file.sync_all())
            .map_err(|source| {
                error_with_source(source, |e| {
                    format!("sync project manifest staging file: {e}")
                })
            })?;
        // renameat is anchored to the already-open parent directory; a path
        // swap cannot redirect the write outside this directory.
        // renameat neo vào parent dirfd đã mở nên đổi path không thể chuyển ghi ra ngoài.
        renameat(parent.as_fd(), &temp, parent.as_fd(), &leaf).map_err(|source| {
            error_with_source(source, |e| {
                format!("publish project manifest atomically: {e}")
            })
        })?;
        parent.sync_all().map_err(|source| {
            error_with_source(source, |e| format!("sync project manifest directory: {e}"))
        })?;
        Ok(())
    })();
    if result.is_err() {
        let _ = unlinkat(parent.as_fd(), &temp, rustix::fs::AtFlags::empty());
    }
    result
}

#[cfg(test)]
#[path = "test/project_write.rs"]
mod tests;
