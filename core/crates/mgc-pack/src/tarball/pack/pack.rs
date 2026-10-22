//! Coordinate deterministic archive construction and publication.
//! Điều phối tạo archive xác định và publish file kết quả.

use crate::ignore::select_files;
use crate::tarball::{
    PackError, PackResult, append_entries, create_staged_output, digest_archive,
    publish_staged_output,
};
use anyhow::Result;
use flate2::write::GzEncoder;
use std::path::Path;

/// Pack a project root into a deterministic gzip tarball.
/// Đóng gói project root thành gzip tarball xác định.
pub fn pack(root: &Path, output_path: &Path, prefix: &str) -> Result<PackResult> {
    let root = root.canonicalize()?;
    let parent = output_path
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let output = parent.canonicalize()?.join(
        output_path
            .file_name()
            .ok_or(PackError::ArchiveOutputMustNameFile)?,
    );
    let mut files = select_files(&root)?;
    files.retain(|relative| root.join(relative) != output);
    let mut staged = create_staged_output(parent)?;
    let entry_summary;

    let mut gzip = GzEncoder::new(staged.as_file_mut(), flate2::Compression::default());
    {
        let mut builder = tar::Builder::new(&mut gzip);
        builder.follow_symlinks(false);
        entry_summary = append_entries(&mut builder, &root, &files, prefix)?;
        builder.finish()?;
    }
    gzip.finish()?;

    let digest = digest_archive(staged.as_file_mut())?;
    publish_staged_output(staged, output_path)?;

    Ok(PackResult {
        tarball_path: output_path.to_path_buf(),
        shasum: digest.shasum,
        integrity: digest.integrity,
        size: digest.size,
        unpacked_size: entry_summary.unpacked_size,
        entry_count: entry_summary.entry_count,
    })
}
