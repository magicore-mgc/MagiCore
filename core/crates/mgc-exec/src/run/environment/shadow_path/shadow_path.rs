//! Owns the guarded temporary directory lifecycle and identity checks.
//! Sở hữu vòng đời thư mục tạm được bảo vệ và kiểm tra tên.

use crate::allowlist::FORBIDDEN_TOOLS;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicU64;

use super::super::{EnvironmentError, ShadowPath, write_blocker};

const MAX_SHADOW_PATH_ATTEMPTS: usize = 16;
static SHADOW_PATH_SEQUENCE: AtomicU64 = AtomicU64::new(0);

impl ShadowPath {
    pub(in crate::run::implementation) fn create(
        scoped_exempt: &[&str],
    ) -> std::result::Result<Self, EnvironmentError> {
        for _ in 0..MAX_SHADOW_PATH_ATTEMPTS {
            let dir = unique_temp_dir("mgc-exec-shadow-path")?;
            match Self::create_at(dir, scoped_exempt) {
                Ok(shadow_path) => return Ok(shadow_path),
                Err(EnvironmentError::CreateDirectory(source))
                    if source.kind() == std::io::ErrorKind::AlreadyExists =>
                {
                    continue;
                }
                Err(error) => return Err(error),
            }
        }

        Err(EnvironmentError::AllocationAttemptsExceeded {
            attempts: MAX_SHADOW_PATH_ATTEMPTS,
        })
    }

    pub(in crate::run::implementation) fn create_at(
        dir: PathBuf,
        scoped_exempt: &[&str],
    ) -> std::result::Result<Self, EnvironmentError> {
        create_shadow_directory(&dir).map_err(EnvironmentError::CreateDirectory)?;
        let mut shadow_path = Self {
            dir,
            cleaned: false,
            cleanup_attempted: false,
        };
        for tool in FORBIDDEN_TOOLS {
            if !scoped_exempt.contains(tool)
                && let Err(error) = write_blocker(shadow_path.path(), tool)
            {
                let setup = EnvironmentError::CreateBlocker {
                    tool: (*tool).to_owned(),
                    source: error,
                };
                return Err(match shadow_path.cleanup() {
                    Ok(()) => setup,
                    Err(cleanup) => EnvironmentError::CleanupAfterSetup {
                        setup: Box::new(setup),
                        cleanup: Box::new(cleanup),
                    },
                });
            }
        }
        #[cfg(unix)]
        if let Err(error) = set_shadow_directory_mode(shadow_path.path(), 0o500) {
            let setup = EnvironmentError::SealDirectory(error);
            return Err(match shadow_path.cleanup() {
                Ok(()) => setup,
                Err(cleanup) => EnvironmentError::CleanupAfterSetup {
                    setup: Box::new(setup),
                    cleanup: Box::new(cleanup),
                },
            });
        }

        Ok(shadow_path)
    }

    pub(in crate::run::implementation) fn path(&self) -> &Path {
        &self.dir
    }
}

#[cfg(unix)]
fn create_shadow_directory(dir: &Path) -> std::io::Result<()> {
    use std::os::unix::fs::DirBuilderExt;

    let mut builder = std::fs::DirBuilder::new();
    builder.mode(0o700);
    builder.create(dir)
}

#[cfg(not(unix))]
fn create_shadow_directory(dir: &Path) -> std::io::Result<()> {
    std::fs::create_dir(dir)
}

impl Drop for ShadowPath {
    fn drop(&mut self) {
        if !self.cleaned && !self.cleanup_attempted {
            // Explicit production cleanup reports failures. Drop retries as a
            // final safety net when control exits before that path can run.
            // Cleanup tường minh báo lỗi; Drop chỉ là lưới an toàn khi thoát sớm.
            if let Err(error) = self.cleanup() {
                eprintln!("MagiCore failed to clean guarded temporary directory: {error}");
            }
        }
    }
}

impl ShadowPath {
    pub(in crate::run::implementation) fn cleanup(
        &mut self,
    ) -> std::result::Result<(), EnvironmentError> {
        if self.cleaned {
            return Ok(());
        }
        self.cleanup_attempted = true;
        #[cfg(unix)]
        set_shadow_directory_mode(&self.dir, 0o700)
            .map_err(EnvironmentError::RestoreDirectoryPermissions)?;
        std::fs::remove_dir_all(&self.dir).map_err(|source| EnvironmentError::RemoveDirectory {
            path: self.dir.clone(),
            source,
        })?;
        self.cleaned = true;
        Ok(())
    }
}

#[cfg(unix)]
fn set_shadow_directory_mode(dir: &Path, mode: u32) -> std::io::Result<()> {
    use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};

    // Open without following a substituted symlink, then change permissions
    // through the verified handle so a path swap cannot redirect chmod.
    // Không đi theo symlink bị thay thế; chmod qua handle đã xác minh để tránh đổi đích.
    let directory = std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW)
        .open(dir)?;
    let metadata = directory.metadata()?;
    if !metadata.file_type().is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "package-manager deny path is no longer a real directory",
        ));
    }
    let mut permissions = metadata.permissions();
    permissions.set_mode(mode);
    directory.set_permissions(permissions)?;
    Ok(())
}

pub(in crate::run) fn unique_temp_dir(
    prefix: &str,
) -> std::result::Result<PathBuf, EnvironmentError> {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(EnvironmentError::SystemClock)?
        .as_nanos();
    let sequence = SHADOW_PATH_SEQUENCE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    Ok(std::env::temp_dir().join(format!(
        "{prefix}-{}-{nanos}-{sequence}",
        std::process::id()
    )))
}

/// Match only a direct child directory produced by `unique_temp_dir`.
/// Chỉ nhận thư mục con trực tiếp có tên đúng mẫu `unique_temp_dir` tạo ra.
#[cfg(unix)]
pub(in crate::run) fn is_generated_shadow_directory(dir: &Path) -> bool {
    let Some(parent) = dir.parent() else {
        return false;
    };
    let (Ok(actual_parent), Ok(temp_root)) = (
        std::fs::canonicalize(parent),
        std::fs::canonicalize(std::env::temp_dir()),
    ) else {
        return false;
    };
    if actual_parent != temp_root {
        return false;
    }
    let Some(name) = dir.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    let Some(suffix) = name.strip_prefix("mgc-exec-shadow-path-") else {
        return false;
    };
    let fields = suffix.split('-').collect::<Vec<_>>();
    fields.len() == 3
        && fields
            .iter()
            .all(|field| !field.is_empty() && field.bytes().all(|byte| byte.is_ascii_digit()))
}
