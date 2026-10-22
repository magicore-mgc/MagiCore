//! Validate and stream regular package inputs into tar entries.
//! Kiểm tra và stream file package thường vào tar entry.

use crate::tarball::PackError;
use anyhow::Result;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub(in crate::tarball) struct EntrySummary {
    pub unpacked_size: u64,
    pub entry_count: usize,
}

pub(in crate::tarball) fn append_entries<W: Write>(
    builder: &mut tar::Builder<W>,
    root: &Path,
    files: &[PathBuf],
    prefix: &str,
) -> Result<EntrySummary> {
    let mut unpacked_size = 0_u64;
    for relative in files {
        let input_path = root.join(relative);
        let entry_name = if prefix.is_empty() {
            relative.to_string_lossy().to_string()
        } else {
            format!("{}/{}", prefix, relative.to_string_lossy())
        };
        let mut input = open_package_input(&input_path, relative)?;
        let metadata = input.metadata()?;
        let size = metadata.len();
        unpacked_size = unpacked_size
            .checked_add(size)
            .ok_or(PackError::PackageUnpackedSizeOverflow)?;

        let mut header = tar::Header::new_gnu();
        header.set_size(size);
        #[cfg(unix)]
        let mode = {
            use std::os::unix::fs::PermissionsExt;
            if metadata.permissions().mode() & 0o111 != 0 {
                0o755
            } else {
                0o644
            }
        };
        #[cfg(not(unix))]
        let mode = 0o644;
        header.set_mode(mode);
        header.set_mtime(0);
        header.set_cksum();

        {
            let mut bounded = (&mut input).take(size);
            builder.append_data(&mut header, &entry_name, &mut bounded)?;
            if bounded.limit() != 0 {
                return Err(PackError::PackageInputShrank(relative.display().to_string()).into());
            }
        }
        if input.read(&mut [0_u8; 1])? != 0 {
            return Err(PackError::PackageInputGrew(relative.display().to_string()).into());
        }
    }

    Ok(EntrySummary {
        unpacked_size,
        entry_count: files.len(),
    })
}

pub(super) fn open_package_input(path: &Path, relative: &Path) -> Result<File> {
    match mgc_platform::open_regular_file_no_follow(path) {
        Ok(file) => Ok(file),
        Err(
            mgc_platform::OpenRegularFileError::Symlink
            | mgc_platform::OpenRegularFileError::NotRegular
            | mgc_platform::OpenRegularFileError::OpenedNotRegular,
        ) => Err(PackError::PackageInputNotRegular(relative.display().to_string()).into()),
        #[cfg(windows)]
        Err(mgc_platform::OpenRegularFileError::ReparsePoint) => {
            Err(PackError::PackageInputReparsePoint(relative.display().to_string()).into())
        }
        Err(error) => Err(error.into()),
    }
}
