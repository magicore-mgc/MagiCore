//! Platform symlink fixtures for candidate-plan tests.
//! Fixture symlink theo OS cho test candidate plan.

use std::io;
use std::path::Path;

#[cfg(unix)]
pub(super) fn symlink_directory(target: &Path, link: &Path) -> io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(windows)]
pub(super) fn symlink_directory(target: &Path, link: &Path) -> io::Result<()> {
    std::os::windows::fs::symlink_dir(target, link)
}
