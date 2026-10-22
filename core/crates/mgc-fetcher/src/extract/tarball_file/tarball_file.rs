//! Open a tarball file and delegate to the bounded reader API.
//! Mở file tarball và chuyển cho API đọc stream có giới hạn.

use anyhow::Result;
use std::fs::File;
use std::path::Path;

/// Extracts a gzip-compressed tarball to a destination directory.
/// Entries are path-checked; archive links/special files and symlink destination
/// roots, parents or final targets are rejected before they can escape the root.
/// Giải nén tarball gzip vào thư mục đích. Path được kiểm tra; link/special
/// entry và root/parent/target dạng symlink bị từ chối để chặn thoát root.
pub fn extract_tarball(tarball_path: &Path, dest: &Path) -> Result<()> {
    let file = File::open(tarball_path)?;
    super::super::extract_tarball_from_reader(file, dest)
}
