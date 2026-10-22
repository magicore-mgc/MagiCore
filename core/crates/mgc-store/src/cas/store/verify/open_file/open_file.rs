//! Opens one verified-file handle and preserves CAS diagnostics.
//! Mở một handle file cần xác minh và giữ diagnostic CAS.

use std::fs::File;
use std::path::Path;

use crate::cas::store::StoreError;
use mgc_platform::open_regular_file_no_follow;

use super::error::OpenFileError;

pub(in crate::cas::store) fn open_regular_file(
    path: &Path,
    symlink_message: &'static str,
) -> Result<File, StoreError> {
    open_regular_file_no_follow(path)
        .map_err(|error| OpenFileError::from_platform(error, symlink_message))
        .map_err(|error| error.into_store_error(path))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
