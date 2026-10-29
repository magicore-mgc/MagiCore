//! Validate, stream and stage archive entries for CAS import.
//! Xác minh, đọc stream và staging entry archive để import CAS.

use super::super::{ExtractionError, ExtractionResult};
use flate2::read::GzDecoder;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};
use std::time::Instant;
use tar::Archive;

pub(in crate::extract) struct StagedFile {
    pub(in crate::extract) path: PathBuf,
    pub(in crate::extract) source_path: PathBuf,
    pub(in crate::extract) executable: bool,
}

pub(in crate::extract) struct StagedArchive {
    _spool_root: tempfile::TempDir,
    pub(in crate::extract) files: Vec<StagedFile>,
    pub(in crate::extract) directories: Vec<PathBuf>,
    pub(in crate::extract) archive_read_ms: u64,
}

pub(in crate::extract) fn spool_archive<R: Read>(
    reader: R,
    dest_root: &Path,
    verify_existing_package_root: bool,
) -> ExtractionResult<Option<StagedArchive>> {
    let spool_parent = dest_root
        .parent()
        .ok_or(ExtractionError::DestinationHasNoParent)?;
    let spool_root = tempfile::Builder::new()
        .prefix(".mgc-extract-")
        .tempdir_in(spool_parent)?;
    let decoder = GzDecoder::new(reader);
    let mut archive = Archive::new(decoder);
    let mut files_map = HashMap::new();
    let mut directories = Vec::new();
    let mut entry_count = 0usize;
    let mut total_unpacked_bytes = 0u64;
    let archive_read_started = Instant::now();

    for entry in archive.entries()? {
        let mut entry = entry?;
        entry_count = entry_count
            .checked_add(1)
            .ok_or(ExtractionError::EntryCountOverflow)?;
        if entry_count > super::super::archive_limits::MAX_ARCHIVE_ENTRIES {
            return Err(ExtractionError::EntryLimit);
        }
        let entry_size = entry.size();
        if entry_size > super::super::archive_limits::MAX_ENTRY_BYTES {
            return Err(ExtractionError::EntryTooLarge);
        }
        total_unpacked_bytes = total_unpacked_bytes
            .checked_add(entry_size)
            .ok_or(ExtractionError::TotalSizeOverflow)?;
        if total_unpacked_bytes > super::super::archive_limits::MAX_TOTAL_FILE_BYTES {
            return Err(ExtractionError::TotalSizeLimit);
        }
        let relative = super::super::archive_path::sanitize_archive_path(entry.path()?.as_ref())?;
        let entry_type = entry.header().entry_type();
        if matches!(entry_type.as_byte(), b'g' | b'x') {
            continue;
        }
        // SEC-3: refuse interpreter install hooks before staging any byte.
        // (SEC-3: từ chối hook cài đặt interpreter trước khi staging byte nào.)
        super::super::install_hook_guard::InstallHookGuard::deny()
            .check(&relative, entry.header().mode().ok())?;
        let mapped = if verify_existing_package_root {
            let mut components = relative.components();
            if !matches!(components.next(), Some(Component::Normal(part)) if part == "package") {
                return Ok(None);
            }
            let remainder: PathBuf = components.collect();
            if remainder.as_os_str().is_empty() && !entry_type.is_dir() {
                return Ok(None);
            }
            remainder
        } else {
            relative
        };
        let target = dest_root.join(&mapped);
        if !target.starts_with(dest_root) {
            return Err(ExtractionError::PathEscapes(target));
        }
        if entry_type.is_symlink() || entry_type.is_hard_link() {
            return Err(ExtractionError::LinksNotAllowed(target));
        }
        if entry_type.is_dir() {
            directories.push(target);
            continue;
        }
        if !entry_type.is_file() {
            return Err(ExtractionError::UnsupportedEntry(target));
        }
        let executable = entry.header().mode().unwrap_or(0o644) & 0o111 != 0;
        let source_path = spool_root.path().join(format!("entry-{entry_count:05}"));
        let mut output = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&source_path)?;
        let copied = std::io::copy(&mut entry, &mut output)?;
        if copied != entry_size {
            return Err(ExtractionError::TruncatedEntry(target));
        }
        output.flush()?;
        if let Some(previous) = files_map.insert(
            target.clone(),
            StagedFile {
                path: target,
                source_path,
                executable,
            },
        ) {
            std::fs::remove_file(previous.source_path)?;
        }
    }

    Ok(Some(StagedArchive {
        _spool_root: spool_root,
        files: files_map.into_values().collect(),
        directories,
        archive_read_ms: archive_read_started.elapsed().as_millis() as u64,
    }))
}
