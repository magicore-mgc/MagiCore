//! Stores verified packages through an exclusive staging file and atomic replacement.
//! Lưu package đã xác minh bằng staging độc quyền và thay thế atomic.

use crate::package_cache::{
    error::CacheError, package_cache::PackageCache, path_guard::ensure_cache_directory,
};
use anyhow::Result;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

/// Avoid an unbounded retry if another actor continuously occupies names.
/// Giới hạn retry nếu tiến trình khác liên tục chiếm tên file.
const MAX_TEMP_FILE_ATTEMPTS: usize = 32;

/// Process-wide suffix prevents collisions between cache instances.
/// Hậu tố toàn process tránh trùng tên giữa các cache instance.
static TEMP_FILE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

struct StagedPackage {
    path: PathBuf,
    file: Option<File>,
}

impl StagedPackage {
    fn create(directory: &Path, package_id: &str) -> Result<Self> {
        for _ in 0..MAX_TEMP_FILE_ATTEMPTS {
            let sequence = TEMP_FILE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
            let path = directory.join(format!(
                ".package.tgz.tmp-{}-{sequence}",
                std::process::id()
            ));
            match OpenOptions::new().write(true).create_new(true).open(&path) {
                Ok(file) => {
                    return Ok(Self {
                        path,
                        file: Some(file),
                    });
                }
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(error.into()),
            }
        }
        Err(CacheError::TemporaryFileExhausted(package_id.to_owned()).into())
    }

    fn file_mut(&mut self) -> io::Result<&mut File> {
        self.file
            .as_mut()
            .ok_or_else(|| io::Error::other("staging file is closed"))
    }

    fn finish(&mut self) -> Result<()> {
        let Some(mut file) = self.file.take() else {
            return Ok(());
        };
        file.flush()?;
        file.sync_all()?;
        Ok(())
    }
}

impl Drop for StagedPackage {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

impl PackageCache {
    /// Store a package only after its streamed integrity matches the caller's value.
    /// Chỉ lưu package sau khi integrity tính theo luồng khớp giá trị caller cung cấp.
    pub fn store_package(
        &self,
        package_id: &str,
        source: &Path,
        integrity: &str,
    ) -> Result<PathBuf> {
        let _guard = self.locks.write_for(package_id)?;
        let package_dir = self.package_dir(package_id);
        ensure_cache_directory(&self.root, &package_dir)?;

        let mut source_file = File::open(source)?;
        let mut staged = StagedPackage::create(&package_dir, package_id)?;
        io::copy(&mut source_file, staged.file_mut()?)?;
        staged.finish()?;

        let actual_integrity = self.compute_integrity_no_follow(&staged.path)?;
        if actual_integrity != integrity {
            return Err(CacheError::IntegrityMismatch {
                package_id: package_id.to_owned(),
                expected: integrity.to_owned(),
                actual: actual_integrity,
                during_store: true,
            }
            .into());
        }

        let package_path = package_dir.join("package.tgz");
        mgc_platform::atomic_replace::atomic_replace_file(&staged.path, &package_path)?;
        Ok(package_path)
    }
}
