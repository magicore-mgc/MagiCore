//! Locate the extracted package root inside a tarball extraction dir.
//! Tìm package root đã giải nén trong thư mục giải nén tarball.

use mgc_types::{MgError, MgResult};
use std::path::{Path, PathBuf};

pub fn locate_package_dir(extract_root: &Path) -> MgResult<PathBuf> {
    let package_dir = extract_root.join("package");
    if package_dir.is_dir() {
        return Ok(package_dir);
    }

    let first_dir = std::fs::read_dir(extract_root)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .find(|path| path.is_dir());

    first_dir.ok_or_else(|| {
        MgError::Other(format!(
            "extracted tarball missing package root in '{}'",
            extract_root.display()
        ))
    })
}
