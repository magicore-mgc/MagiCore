// Removes only the staging file created by this download. / Chỉ xóa file staging do lượt tải này tạo.
use std::path::{Path, PathBuf};

pub(in crate::native::npm_registry::registry::tarball::automatic::automatic::body_stream) struct StagingCleanup
{
    path: PathBuf,
    armed: bool,
}

impl StagingCleanup {
    pub(in crate::native::npm_registry::registry::tarball::automatic::automatic::body_stream) fn new(
        path: &Path,
    ) -> Self {
        Self {
            path: path.to_path_buf(),
            armed: false,
        }
    }

    pub(in crate::native::npm_registry::registry::tarball::automatic::automatic::body_stream) fn arm(
        &mut self,
    ) {
        self.armed = true;
    }

    pub(in crate::native::npm_registry::registry::tarball::automatic::automatic::body_stream) fn disarm(
        &mut self,
    ) {
        self.armed = false;
    }
}

impl Drop for StagingCleanup {
    fn drop(&mut self) {
        if self.armed {
            let _ = std::fs::remove_file(&self.path);
        }
    }
}
