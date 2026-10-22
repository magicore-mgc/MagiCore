//! Create temporary output and publish only after the completed archive is synced.
//! Tạo output tạm và chỉ publish sau khi archive hoàn tất được sync.

use anyhow::Result;
use std::path::Path;
use tempfile::NamedTempFile;

pub(in crate::tarball) fn create_staged_output(parent: &Path) -> Result<NamedTempFile> {
    Ok(NamedTempFile::new_in(parent)?)
}

pub(in crate::tarball) fn publish_staged_output(
    staged: NamedTempFile,
    output_path: &Path,
) -> Result<()> {
    staged.as_file().sync_all()?;
    staged.persist(output_path)?;
    Ok(())
}
