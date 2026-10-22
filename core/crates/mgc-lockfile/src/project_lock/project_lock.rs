//! Acquires and holds the kernel-backed project writer lock through RAII.
//! Acquire và giữ project writer lock của kernel bằng RAII.

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use fs2::FileExt;

use crate::{LockfileError, LockfileResult};

use super::contention::is_lock_contention;
use super::error::ProjectLockError;
use super::path_safety::{ensure_real_dir, refuse_link_swap};

/// Guard-file path for project writers — File guard cho writer project.
pub const WRITE_LOCK_FILE: &str = "lock.write";

/// RAII guard whose file close releases the OS lock, including on unwind.
/// Guard RAII; đóng file sẽ nhả OS lock, kể cả khi unwind.
#[derive(Debug)]
pub struct ProjectWriteLock {
    _file: std::fs::File,
    guard_path: PathBuf,
}

impl ProjectWriteLock {
    /// Return the guard-file path for a project root (`.magicore/lock.write`).
    /// Trả về path guard của project root (`.magicore/lock.write`).
    pub fn guard_path_for(project_root: &Path) -> PathBuf {
        project_root.join(".magicore").join(WRITE_LOCK_FILE)
    }

    /// Acquire the exclusive OS lock, waiting up to `timeout`; never reclaim a live lock.
    /// Acquire OS lock độc quyền, chờ tối đa `timeout`; không cướp lock còn sống.
    pub fn acquire(project_root: &Path, timeout: Duration) -> LockfileResult<Self> {
        let magicore = project_root.join(".magicore");
        // Check before creating: project ancestors are operator-owned; `.magicore` is the trust boundary.
        // Kiểm tra trước khi tạo: ancestor do operator sở hữu; `.magicore` là trust boundary.
        ensure_real_dir(&magicore).map_err(LockfileError::from)?;
        let guard_path = magicore.join(WRITE_LOCK_FILE);
        refuse_link_swap(&guard_path).map_err(LockfileError::from)?;
        let mut options = std::fs::OpenOptions::new();
        options.read(true).write(true).create(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.custom_flags(libc::O_NOFOLLOW);
        }
        let file = options
            .open(&guard_path)
            .map_err(|source| ProjectLockError::OpenGuard {
                path: guard_path.display().to_string(),
                source,
            })?;
        let start = Instant::now();
        loop {
            match file.try_lock_exclusive() {
                Ok(()) => {
                    return Ok(Self {
                        _file: file,
                        guard_path,
                    });
                }
                Err(source) if is_lock_contention(&source) => {
                    if start.elapsed() >= timeout {
                        return Err(ProjectLockError::Busy {
                            path: guard_path.display().to_string(),
                        }
                        .into());
                    }
                    std::thread::sleep(Duration::from_millis(25));
                }
                Err(source) => {
                    return Err(ProjectLockError::Lock {
                        path: guard_path.display().to_string(),
                        source,
                    }
                    .into());
                }
            }
        }
    }

    /// Return the diagnostic guard path; it never determines lock ownership.
    /// Trả về path chẩn đoán; path này không quyết định quyền sở hữu lock.
    pub fn guard_path(&self) -> &Path {
        &self.guard_path
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
